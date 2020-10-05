use crate::runtime::Env;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Clone)]
pub struct LispCell {
    pub car: Box<Exp>,
    pub cdr: Box<Exp>,
}

#[derive(Clone)]
pub struct Function<'a> {
    params: Box<Exp>,
    body: Box<Exp>,
    env: Box<Env<'a>>,
}

#[derive(Clone)]
pub struct Lambda {
    pub params: Rc<Exp>,
    pub body: Rc<Exp>,
}

pub enum LispErr {
    Reason(String),
    Bug(String),
    UnexpectedToken(String),
}

pub type Primitive = fn(env: &mut Env, params: Exp) -> Result<Exp, LispErr>;

#[derive(Clone)]
pub enum Exp {
    Nil,
    Number(f64),
    Symbol(String),
    Pair(LispCell),
    Primitive(Primitive),
    Lambda(Lambda),
}

impl LispCell {
    pub fn new(car: Exp, cdr: Exp) -> LispCell {
        LispCell {
            car: Box::new(car),
            cdr: Box::new(cdr),
        }
    }

    pub fn set_cdr(&mut self, exp: Exp) {
        self.cdr = Box::new(exp);
    }

    pub fn append(&mut self, exp: Exp) -> Result<(), LispErr> {
        let mut cell = self;
        loop {
            if let Exp::Nil = *cell.cdr {
                cell.set_cdr(Exp::Pair(cons(exp, Exp::Nil)));
                return Ok(());
            }
            if let Exp::Pair(ref mut x) = &mut *cell.cdr {
                cell = x;
            } else {
                return Err(LispErr::Reason(
                    "Tried to append to an unproper list".to_string(),
                ));
            }
        }
    }
}

pub struct ListIter {
    pub list: Option<LispCell>,
}

impl Iterator for ListIter {
    type Item = Exp;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(list) = self.list.clone() {
            let item = list.car;
            if let Exp::Pair(rest) = *list.cdr {
                self.list = Some(rest);
            } else {
                self.list = None;
            }
            return Some(*item);
        } else {
            return None;
        }
    }
}

impl IntoIterator for LispCell {
    type Item = Exp;
    type IntoIter = ListIter;

    fn into_iter(self) -> Self::IntoIter {
        ListIter { list: Some(self) }
    }
}

impl Display for Exp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.format())
    }
}

impl Exp {
    fn format(&self) -> String {
        match self {
            Exp::Nil => "()".to_string(),
            Exp::Number(x) => x.to_string(),
            Exp::Symbol(x) => x.to_string(),
            Exp::Primitive(_) => "primitive function".to_string(),
            Exp::Pair(x) => {
                let mut s = String::from("(");
                for exp in x.clone() {
                    s.push_str(&exp.format());
                    s.push(' ');
                }
                s.pop();
                s.push(')');
                s
            }
            Exp::Lambda(_) => "lambda function".to_string(),
        }
    }
}

pub fn cons(car: Exp, cdr: Exp) -> LispCell {
    LispCell::new(car, cdr)
}
