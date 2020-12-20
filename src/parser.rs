use crate::ast::*;
use crate::Exception::*;
use regex::Regex;

pub fn lex(code: &str) -> Vec<String> {
    Regex::new(r";.*?(\n|\r|\r\n)")
        .unwrap()
        .replace_all(code, "\n")
        .replace("(", " ( ")
        .replace(")", " ) ")
        .replace("'", " ' ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

pub fn parse<'a>(tokens: &'a [String]) -> Result<(Exp, &'a [String]), Exn> {
    let (token, rest) = tokens
        .split_first()
        .ok_or(Exn::other_unknown("Could not get next token"))?;
    match &token[..] {
        "(" => parse_list(rest),
        ")" => Err(Exn::syntax_unknown("Found unexpected \")\"")),
        "'" => parse_quote(rest),
        _ => Ok((parse_atom(token)?, rest)),
    }
}

fn parse_quote<'a>(tokens: &'a [String]) -> Result<(Exp, &'a [String]), Exn> {
    let (datum, rest) = parse(tokens)?;
    Ok((Exp::Pair(cons(Exp::Symbol("quote".to_string()), Exp::Pair(cons(datum, Exp::Nil)))), rest))
}

fn parse_list<'a>(tokens: &'a [String]) -> Result<(Exp, &'a [String]), Exn> {
    let mut list = cons(Exp::Nil, Exp::Nil);
    let mut toks = tokens;
    loop {
        let (next, rest) = toks
            .split_first()
            .ok_or(Exn::other_unknown("Could not get next token"))?;
        if next == ")" {
            if let (&Exp::Nil, &Exp::Nil) = (&*list.car, &*list.cdr) {
                return Ok((Exp::Pair(list), rest));
            } else if let Exp::Pair(res) = *list.cdr {
                return Ok((Exp::Pair(res), rest));
            } else {
                return Err(Exn::syntax_unknown("Could not parse list"));
            }
        }
        let (exp, new_toks) = parse(toks)?;
        list.append(exp)?;
        toks = new_toks;
    }
}

fn parse_atom(token: &str) -> Result<Exp, Exn> {
    let s = token.to_string();
    let mut iter = s.chars();
    let first = iter.next().ok_or(Exn::other_unknown(&format!("Could not get first char of token {}", token)))?;
    if let Ok(x) = token.parse() {
        // float 64
        Ok(Exp::Number(x))
    } else if first == '#' {
        if token == "#t" {
            Ok(Exp::Boolean(true))
        } else if token == "#f" {
            Ok(Exp::Boolean(false))
        } else {
            Err(Exn::syntax_unknown(&format!("Unexpected token: {}", token)))
        }
    // } else if first == '"' {
    //     // string
    //     let mut string = String::new();
    //     for c in iter {
    //         if c != '"' || string.ends_with("\"") {
    //             string.push(c);
    //         } else {
    //             return Ok(Exp::String(string));
    //         }
    //     }
    //     Err(LispErr::UnexpectedToken(
    //         "Expected \" to finish string literal".to_string(),
    //     ))
    } else {
        Ok(Exp::Symbol(token.to_string()))
    }
}
