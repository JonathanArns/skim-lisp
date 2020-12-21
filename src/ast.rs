use crate::runtime::Env;
use crate::Exception::*;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Clone)]
pub struct LispCell {
    pub car: Box<Item>,
    pub cdr: Box<Item>,
}

#[derive(Clone)]
pub struct Lambda {
    pub params: Rc<Vec<String>>,
    pub body: Rc<Item>,
}

pub type Primitive = fn(env: &mut Env, params: Item) -> Result<Exp, Exn>;

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

#[derive(Clone)]
pub struct Item {
    pub meta: Meta,
    pub exp: Exp,
}

impl Item {
    pub fn new(meta: Meta, exp: Exp) -> Item {
        Item {
            meta: meta,
            exp: exp,
        }
    }

    pub fn cons(meta: Meta, car: Item, cdr: Item) -> Item {
        Item {
            meta: meta,
            exp: Exp::Pair(cons(car, cdr)),
        }
    }

    pub fn prim(meta: Meta, fun: Primitive) -> Item {
        Self::new(meta, Exp::Primitive(fun))
    }
}

impl LispCell {
    pub fn new(car: Item, cdr: Item) -> LispCell {
        LispCell {
            car: Box::new(car),
            cdr: Box::new(cdr),
        }
    }

    pub fn set_cdr(&mut self, cdr: Item) {
        self.cdr = Box::new(cdr);
    }

    pub fn append(&mut self, item: Item) -> Result<(), Exn> {
        let mut cell = self;
        loop {
            if let Exp::Nil = cell.cdr.exp {
                cell.set_cdr(Item::cons(
                    cell.cdr.meta,
                    item,
                    Item::new(cell.cdr.meta, Exp::Nil),
                ));
                return Ok(());
            }
            if let Exp::Pair(ref mut x) = cell.cdr.exp {
                cell = x;
            } else {
                return Err(Exn::other_unknown("tried to append to an unproper list"));
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
        let mut s = x.car.exp.format();
        match x.cdr.exp {
            Exp::Pair(cdr) => {
                s.push(' ');
                s.push_str(&cdr.format_naked());
            }
            Exp::Nil => {} // do nothing
            _ => {
                s.push_str(" . ");
                s.push_str(&x.cdr.exp.format());
            }
        }
        s
    }
}

pub struct ListIter {
    pub list: Option<LispCell>,
}

impl Iterator for ListIter {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(list) = self.list.clone() {
            let item = list.car;
            if let Exp::Pair(rest) = list.cdr.exp {
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
    type Item = Item;
    type IntoIter = ListIter;

    fn into_iter(self) -> Self::IntoIter {
        ListIter { list: Some(self) }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.exp.format())
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

    pub fn type_name(&self) -> String {
        match self {
            Exp::Nil => "()",
            Exp::Number(_) => "number",
            Exp::Symbol(_) => "symbol",
            Exp::Primitive(_) => "primitive function",
            Exp::Pair(_) => "pair",
            Exp::Lambda(_) => "lambda function",
            Exp::Boolean(_) => "boolean",
            Exp::Char(_) => "char",
            Exp::Vector(_) => "vector",
            Exp::String(_) => "string",
        }
        .to_string()
    }
}

pub fn cons(car: Item, cdr: Item) -> LispCell {
    LispCell::new(car, cdr)
}
