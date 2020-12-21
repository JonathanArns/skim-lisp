use crate::ast::*;
use crate::Exception::*;

#[derive(Clone)]
pub struct Token {
    meta: Meta,
    string: String,
}

impl Token {
    fn new(loc: usize, c: char) -> Token {
        let mut s = String::new();
        s.push(c);
        Token {
            meta: Meta::new(loc),
            string: s,
        }
    }
}

pub fn lex(code: &str) -> Vec<Token> {
    let mut res: Vec<Token> = Vec::new();
    let mut comment = false;
    let mut token = None;
    for (i, c) in code.char_indices() {
        if comment && (c != '\n' || c != '\r') {
            continue;
        }
        match c {
            ';' => comment = true,
            '\n' | '\r' => comment = false,
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
                res.push(Token::new(i, c));
            }
            _ => {
                if let Some(ref mut t) = token {
                    t.string.push(c)
                } else {
                    token = Some(Token::new(i, c))
                }
            }
        }
    }
    res
}

pub fn parse<'a>(tokens: &'a [Token]) -> Result<(Item, &'a [Token]), Exn> {
    let (token, rest) = tokens
        .split_first()
        .ok_or(Exn::other_unknown("Could not get next token"))?;
    match &token.string[..] {
        "(" => parse_list(rest, token.meta),
        ")" => Err(Exn::syntax_unknown("Found unexpected \")\"")),
        "'" => parse_quote(rest, token.meta),
        _ => Ok((parse_atom(token.to_owned())?, rest)),
    }
}

fn parse_quote<'a>(tokens: &'a [Token], meta: Meta) -> Result<(Item, &'a [Token]), Exn> {
    let (datum, rest) = parse(tokens)?;
    let datum_meta = datum.meta;
    Ok((
        Item::cons(
            meta,
            Item::new(meta, Exp::Symbol("quote".to_string())),
            Item::cons(datum_meta, datum, Item::new(datum_meta, Exp::Nil)),
        ),
        rest,
    ))
}

fn parse_list<'a>(tokens: &'a [Token], meta: Meta) -> Result<(Item, &'a [Token]), Exn> {
    let mut list = cons(Item::new(meta, Exp::Nil), Item::new(meta, Exp::Nil));
    let mut toks = tokens;
    loop {
        let (next, rest) = toks
            .split_first()
            .ok_or(Exn::other_unknown("Could not get next token"))?;
        if next.string == ")" {
            if let (&Exp::Nil, &Exp::Nil) = (&list.car.exp, &list.cdr.exp) {
                return Ok((Item::new(meta, Exp::Pair(list)), rest));
            } else if let Exp::Pair(res) = list.cdr.exp {
                return Ok((Item::new(meta, Exp::Pair(res)), rest));
            } else {
                return Err(Exn::syntax_unknown("Could not parse list"));
            }
        }
        let (exp, new_toks) = parse(toks)?;
        list.append(exp)?;
        toks = new_toks;
    }
}

fn parse_atom(token: Token) -> Result<Item, Exn> {
    let mut iter = token.string.chars();
    let first = iter.next().ok_or(Exn::other_unknown(&format!(
        "Could not get first char of token {}",
        token.string
    )))?;
    if let Ok(x) = token.string.parse() {
        // float 64
        Ok(Item::new(token.meta, Exp::Number(x)))
    } else if first == '#' {
        if token.string == "#t" {
            Ok(Item::new(token.meta, Exp::Boolean(true)))
        } else if token.string == "#f" {
            Ok(Item::new(token.meta, Exp::Boolean(false)))
        } else {
            Err(Exn::syntax_unknown(&format!(
                "Unexpected token: {}",
                token.string
            )))
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
