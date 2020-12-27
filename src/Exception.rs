use std::fmt::Display;


#[derive(Clone)]
pub struct Meta {
    pub line: usize,
    pub position: usize,
    pub token_length: usize,
    pub file_name: Option<String>,
    pub code: String,
}

#[derive(Clone)]
pub enum Arity {
    Exact(usize),
    AtLeast(usize),
}

#[derive(Clone)]
pub enum Condition {
    Syntax(String),
    Arity(Arity, usize),
    Type(String, String),
    Other(String), // TODO: replace this with meaningful variants
}

pub struct Exn {
    meta: Meta,
    condition: Condition,
}

impl Meta {
    pub fn new(line: usize, pos: usize, length: usize, file: Option<String>, code: String) -> Meta {
        Meta {
            line: line,
            position: pos,
            token_length: length,
            file_name: file,
            code: code,
        }
    }

    pub fn empty() -> Meta {
        Meta {
            line: 0,
            position: 0,
            token_length: 0,
            file_name: None,
            code: "".to_owned(),
        }
    }
}

impl Exn {
    pub fn new(meta: Meta, cond: Condition) -> Exn {
        Exn {
            meta: meta,
            condition: cond,
        }
    }

    pub fn syntax(meta: Meta, msg: &str) -> Exn {
        Exn::new(meta, Condition::Syntax(msg.to_string()))
    }

    pub fn arity(meta: Meta, expected: usize, found: usize) -> Exn {
        Exn::new(
            meta,
            Condition::Arity(Arity::Exact(expected), found),
        )
    }

    pub fn typ(meta: Meta, expected: &str, found: &str) -> Exn {
        Exn::new(
            meta,
            Condition::Type(expected.to_string(), found.to_string()),
        )
    }

    pub fn other(meta: Meta, msg: &str) -> Exn {
        Exn::new(meta, Condition::Other(msg.to_string()))
    }
}

impl Display for Exn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let (title, msg) = match self.condition.clone() {
            Condition::Arity(expected, found) => (
                "wrong number of arguments",
                match expected {
                    Arity::Exact(a) => format!("expected {} arguments, found {}", a, found),
                    Arity::AtLeast(a) => format!("expected at least {} arguments, found {}", a, found),
                },
            ),
            Condition::Syntax(msg) => ("wrong syntax", msg),
            Condition::Type(found, expected) => (
                "mismatched types",
                format!("expected {}, found {}", expected, found),
            ),
            Condition::Other(msg) => ("unknown", msg),
        };
        let mut file = true;
        let mut space = String::new();
        let header = if let Some(file_name) = self.meta.file_name.clone() {
            for _ in 0..self.meta.line.to_string().len() {
                space.push(' ');
            }
            let location = format!("{}:{}:{}", file_name, self.meta.line, self.meta.position);
            format!("exception: {}\n{}--> {}", title, space, location)
        } else {
            file = false;
            format!("exception: {}", title)
        };
        let body = if file {
            let mut body = format!("{} |\n{} | {}\n{} |", space, self.meta.line, self.meta.code, space);
            let mut pointer = String::new();
            for _ in 0..self.meta.position { pointer.push(' '); }
            for _ in 0..self.meta.token_length { pointer.push('^'); }
            pointer.push(' ');
            pointer.push_str(&msg);
            pointer.push('\n');
            body.push_str(&pointer);
            body
        } else {
            todo!()
        };
        write!(f, "{}\n{}", header, body)
    }
}
