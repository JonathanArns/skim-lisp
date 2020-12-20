use std::fmt::Display;

pub struct Location {
    file: String,
    index: usize,
}

pub enum Condition {
    Syntax(String),
    Arity(String, usize, usize),
    Type(String, String, String),
    Other(String), // TODO: replace this with meaningful variants
}

pub struct Exn {
    location: Option<Location>,
    condition: Condition
}

impl Exn {
    pub fn new(loc: Location, cond: Condition) -> Exn{
        Exn {
            location: Some(loc),
            condition: cond,
        }
    }
    pub fn new_unknown(cond: Condition) -> Exn{
        Exn {
            location: None,
            condition: cond,
        }
    }

    pub fn syntax_unknown(msg: &str) -> Exn {
        Exn::new_unknown(Condition::Syntax(msg.to_string()))
    }
    
    pub fn arity(loc: Location, name: &str, expected: usize, found: usize) -> Exn {
        Exn::new(loc, Condition::Arity(name.to_string(), expected, found))
    }

    pub fn arity_unknown(name: &str, expected: usize, found: usize) -> Exn {
        Exn::new_unknown(Condition::Arity(name.to_string(), expected, found))
    }

    pub fn typ(loc: Location, name: &str, expected: &str, found: &str) -> Exn {
        Exn::new(loc, Condition::Type(name.to_string(), expected.to_string(), found.to_string()))
    }

    pub fn typ_unknown(name: &str, expected: &str, found: &str) -> Exn {
        Exn::new_unknown(Condition::Type(name.to_string(), expected.to_string(), found.to_string()))
    }

    pub fn other_unknown(msg: &str) -> Exn {
        Exn::new_unknown(Condition::Other(msg.to_string()))
    }
}

impl Display for Exn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        // write!(f, "{}", self.format())
        todo!()
    }
}