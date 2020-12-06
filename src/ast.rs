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
    Boolean(bool),
    Number(f64),
    Char(char),
    Symbol(String),
    String(String),
    Vector(Vec<Exp>),
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
    
    fn format(&self) -> String {
        let mut s = String::from("(");
        s.push_str(&self.format_naked());
        s.push(')');
        s
    }

    fn format_naked(&self) -> String {
        let x = self.clone();
        let mut s = x.car.format();
        match *x.cdr {
            Exp::Pair(cdr) => {
                s.push(' ');
                s.push_str(&cdr.format_naked());
            },
            Exp::Nil => {}, // do nothing
            _ => {
                s.push_str(" . ");
                s.push_str(&x.cdr.format());
            }
        }
        s
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
            Exp::Number(s) => s.to_string(),
            Exp::Symbol(s) => s.to_string(),
            Exp::Primitive(_) => "primitive function".to_string(),
            Exp::Pair(x) => x.format(),
            Exp::Lambda(_) => "lambda function".to_string(),
            Exp::Boolean(b) => if *b { "#t" } else { "#f" }.to_string(),
            Exp::Char(c) => c.to_string(),
            Exp::Vector(vec) => {
                let mut str = String::from("[");
                for exp in vec {
                    str.push_str(&exp.format());
                    str.push(' ');
                }
                str.push(']');
                str
            }
            Exp::String(s) => s.to_string(),
        }
    }
}

pub fn cons(car: Exp, cdr: Exp) -> LispCell {
    LispCell::new(car, cdr)
}
