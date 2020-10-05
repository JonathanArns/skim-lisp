use crate::ast::*;
use regex::Regex;

pub fn lex(expr: &str) -> Vec<String> {
    Regex::new(r";.*?(\n|\r|\r\n)")
        .unwrap()
        .replace_all(expr, "\n")
        .replace("(", " ( ")
        .replace(")", " ) ")
        .replace("'", " ' ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

pub fn parse<'a>(tokens: &'a [String]) -> Result<(Exp, &'a [String]), LispErr> {
    let (token, rest) = tokens
        .split_first()
        .ok_or(LispErr::Reason("Could not get next token".to_string()))?;
    match &token[..] {
        "(" => parse_list(rest),
        ")" => Err(LispErr::UnexpectedToken(")".to_string())),
        _ => Ok((parse_atom(token), rest)),
    }
}

fn parse_list<'a>(tokens: &'a [String]) -> Result<(Exp, &'a [String]), LispErr> {
    let mut list = cons(Exp::Nil, Exp::Nil);
    let mut toks = tokens;
    loop {
        let (next, rest) = toks
            .split_first()
            .ok_or(LispErr::Reason("Could not get next token".to_string()))?;
        if next == ")" {
            if let (&Exp::Nil, &Exp::Nil) = (&*list.car, &*list.cdr) {
                return Ok((Exp::Pair(list), rest));
            } else if let Exp::Pair(res) = *list.cdr {
                return Ok((Exp::Pair(res), rest));
            } else {
                return Err(LispErr::Bug("Failed to parse list".to_string()));
            }
        }
        let (exp, new_toks) = parse(toks)?;
        list.append(exp)?;
        toks = new_toks;
    }
}

fn parse_atom(token: &str) -> Exp {
    let potential_float: Result<f64, std::num::ParseFloatError> = token.parse();
    match potential_float {
        Ok(x) => Exp::Number(x),
        Err(_) => Exp::Symbol(token.to_string()),
    }
}
