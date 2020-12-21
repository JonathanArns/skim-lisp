use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct Meta {
    pub location: usize,
}

pub enum Condition {
    Syntax(String),
    Arity(String, usize, usize),
    Type(String, String, String),
    Other(String), // TODO: replace this with meaningful variants
}

pub struct Exn {
    meta: Option<Meta>,
    condition: Condition,
}

impl Meta {
    pub fn new(loc: usize) -> Meta {
        Meta { location: loc }
    }
}

impl Exn {
    pub fn new(meta: Meta, cond: Condition) -> Exn {
        Exn {
            meta: Some(meta),
            condition: cond,
        }
    }
    pub fn new_unknown(cond: Condition) -> Exn {
        Exn {
            meta: None,
            condition: cond,
        }
    }

    pub fn syntax_unknown(msg: &str) -> Exn {
        Exn::new_unknown(Condition::Syntax(msg.to_string()))
    }

    pub fn arity(loc: Meta, name: &str, expected: usize, found: usize) -> Exn {
        Exn::new(loc, Condition::Arity(name.to_string(), expected, found))
    }

    pub fn arity_unknown(name: &str, expected: usize, found: usize) -> Exn {
        Exn::new_unknown(Condition::Arity(name.to_string(), expected, found))
    }

    pub fn typ(loc: Meta, name: &str, expected: &str, found: &str) -> Exn {
        Exn::new(
            loc,
            Condition::Type(name.to_string(), expected.to_string(), found.to_string()),
        )
    }

    pub fn typ_unknown(name: &str, expected: &str, found: &str) -> Exn {
        Exn::new_unknown(Condition::Type(
            name.to_string(),
            expected.to_string(),
            found.to_string(),
        ))
    }

    pub fn other_unknown(msg: &str) -> Exn {
        Exn::new_unknown(Condition::Other(msg.to_string()))
    }
}

impl Display for Exn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let s = if let Some(meta) = self.meta {
            format!(
                "--------------------------------------\n   Exception occurred at {}",
                meta.location
            )
        } else {
            "--------------------------------------\n   Exception occured at unknown location"
                .to_string()
        };
        write!(f, "{}", s)
    }
}
