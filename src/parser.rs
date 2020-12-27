use crate::ast::*;
use crate::Exception::*;

#[derive(Clone)]
pub struct Token {
    meta: Meta,
    string: String,
}

impl Token {
    fn new(line: usize, position: usize, c: char, file: Option<String>, code: String) -> Token {
        let mut s = String::new();
        s.push(c);
        Token {
            meta: Meta::new(line, position, 1, file, code),
            string: s,
        }
    }
}

pub fn lex(code: &str, file: Option<String>) -> Vec<Token> {
    let mut res: Vec<Token> = Vec::new();
    let mut token = None;
    let mut line = 0;
    let mut position;
    for l in code.split_terminator("\n") {
        line += 1;
        position = 0;
        for c in l.chars() {
            position += 1;
            match c {
                ';' => break,
                ' ' => {
                    if let Some(t) = token {
                        res.push(t);
                        token = None;
                    }
                }
                '(' | ')' | '\'' => {
                    if let Some(t) = token {
                        res.push(t);
                        token = None;
                    }
                    res.push(Token::new(line, position, c, file.clone(), l.to_owned()));
                }
                _ => {
                    if let Some(ref mut t) = token {
                        t.string.push(c);
                        t.meta.token_length += 1;
                    } else {
                        token = Some(Token::new(line, position, c, file.clone(), l.to_owned()));
                    }
                }
            }
        }
    }
    res
}

pub fn parse<'a>(tokens: &'a [Token]) -> Result<(Item, &'a [Token]), Exn> {
    let (token, rest) = tokens
        .split_first()
        .ok_or(Exn::other(Meta::empty(), "Could not get next token"))?;
    match &token.string[..] {
        "(" => parse_list(rest, token.meta.clone()),
        ")" => Err(Exn::syntax(token.meta.clone(), "Found unexpected \")\"")),
        "'" => parse_quote(rest, token.meta.clone()),
        _ => Ok((parse_atom(token.to_owned())?, rest)),
    }
}

fn parse_quote<'a>(tokens: &'a [Token], meta: Meta) -> Result<(Item, &'a [Token]), Exn> {
    let (datum, rest) = parse(tokens)?;
    let datum_meta = datum.meta.clone();
    Ok((
        Item::cons(
            meta.clone(),
            Item::new(meta, Exp::Symbol("quote".to_string())),
            Item::cons(datum_meta.clone(), datum, Item::new(datum_meta, Exp::Nil)),
        ),
        rest,
    ))
}

fn parse_list<'a>(tokens: &'a [Token], mut meta: Meta) -> Result<(Item, &'a [Token]), Exn> {
    let mut list = cons(
        Item::new(meta.clone(), Exp::Nil),
        Item::new(meta.clone(), Exp::Nil),
    );
    let mut toks = tokens;
    loop {
        let (next, rest) = toks
            .split_first()
            .ok_or(Exn::other(Meta::empty(), "Could not get next token"))?;
        if next.string == ")" {
            meta.token_length = 1 + next.meta.position - meta.position;
            if let (&Exp::Nil, &Exp::Nil) = (&list.car.exp, &list.cdr.exp) {
                return Ok((Item::new(meta, Exp::Pair(list)), rest));
            } else if let Exp::Pair(res) = list.cdr.exp {
                return Ok((Item::new(meta, Exp::Pair(res)), rest));
            } else {
                return Err(Exn::syntax(next.meta.clone(), "Could not parse list"));
            }
        }
        let (exp, new_toks) = parse(toks)?;
        list.append(exp)?;
        toks = new_toks;
    }
}

fn parse_atom(token: Token) -> Result<Item, Exn> {
    let mut iter = token.string.chars();
    let first = iter.next().ok_or(Exn::other(
        token.meta.clone(),
        &format!("Could not get first char of token {}", token.string),
    ))?;
    if let Ok(x) = token.string.parse() {
        // float 64
        Ok(Item::new(token.meta, Exp::Number(x)))
    } else if first == '#' {
        if token.string == "#t" {
            Ok(Item::new(token.meta, Exp::Boolean(true)))
        } else if token.string == "#f" {
            Ok(Item::new(token.meta, Exp::Boolean(false)))
        } else {
            Err(Exn::syntax(
                token.meta,
                &format!("Unexpected token: {}", token.string),
            ))
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
        Ok(Item::new(token.meta, Exp::Symbol(token.string)))
    }
}
