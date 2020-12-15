use crate::ast::*;
use crate::runtime::*;
use std::rc::Rc;
use crate::destruct;

#[allow(unused_mut)]
pub fn prim_plus(env: &mut Env, args: Exp) -> Result<Exp, LispErr> {
    let mut res = 0.0;
    let list = destruct!(env, args, ""; (->..Exp::Number))?;
    for x in list {
        res += x;
    }
    Ok(Exp::Number(res))
}

pub fn prim_minus(env: &mut Env, args: Exp) -> Result<Exp, LispErr> {
    let (arg0, rest) = destruct!(env, args, ""; (->Exp::Number) (->..Exp::Number))?;
    let mut res = arg0;
    for x in rest {
        res -= x;
    }
    if arg0 == res {
        Ok(Exp::Number(-res))
    } else {
        Ok(Exp::Number(res))
    }
}

pub fn prim_define(env: &mut Env, args: Exp) -> Result<Exp, LispErr> {
    if let Exp::Pair(list) = args {
        match *list.car {
            Exp::Symbol(key) => {
                let val = if let Exp::Pair(args_cdr) = *list.cdr {
                    eval(env, &*args_cdr.car)?
                } else {
                    Exp::Nil
                };
                env.set(key, val);
                Ok(Exp::Nil)
            }
            Exp::Pair(fn_signature) => {
                if let Exp::Symbol(key) = *fn_signature.car {
                    let lambda = prim_lambda(env, Exp::Pair(cons(*fn_signature.cdr, *list.cdr)))?;
                    env.set(key, lambda);
                    Ok(Exp::Nil)
                } else {
                    Err(LispErr::Reason(
                        "Expected a Symbol as the first argument to (define)".to_string(),
                    ))
                }
            }
            _ => Err(LispErr::Reason(
                "Expected a Symbol as the first argument to (define)".to_string(),
            )),
        }
    } else {
        Err(LispErr::Reason("Too few arguments to (define)".to_string()))
    }
}

pub fn prim_lambda(_: &mut Env, args: Exp) -> Result<Exp, LispErr> {
    if let Exp::Pair(list) = args {
        if let Exp::Pair(params) = *list.car {
            if let Exp::Pair(rest) = *list.cdr {
                return Ok(Exp::Lambda(Lambda {
                    params: Rc::new(Exp::Pair(params)),
                    body: Rc::new(*rest.car),
                }));
            }
        }
    }
    Err(LispErr::Reason("invalid lambda definition".to_string()))
}

pub fn prim_if(env: &mut Env, args: Exp) -> Result<Exp, LispErr> {
    let (test, then, els) = destruct!(env, args, "if"; (->Exp) (Exp) (Exp))?;
    if let Exp::Boolean(false) = test {
        eval(env, &els)
    } else {
        eval(env, &then)
    }
}

#[allow(unused_mut)]
pub fn prim_cond(env: &mut Env, args: Exp) -> Result<Exp, LispErr> {
    let branches = destruct!(env, args, "cond"; (..Exp))?;
    for branch in branches {
        let (car, cdr) = destruct!(env, branch, "cond"; (Exp) (..Exp))?;
        if let Exp::Symbol(ref s) = car {
            if s == "else" {
                if cdr.len() == 0 {
                    return Err(LispErr::Reason("(cond) got an else branch with 0 expressions, expected at least 1".to_string()))
                }
                let mut result = Exp::Nil;
                for body in cdr {
                    result = eval(env, &body)?;
                }
                return Ok(result)
            }
        }
        if let Exp::Boolean(false) = eval(env, &car)? {} else {
            let mut result = Exp::Nil;
            for body in cdr {
                result = eval(env, &body)?;
            }
            return Ok(result)
        }
    }
    Ok(Exp::Nil)
}

#[allow(unused_mut)]
pub fn prim_or(env: &mut Env, args: Exp) -> Result<Exp, LispErr> {
    let exps = destruct!(env, args, "or"; (->..Exp))?;
    for e in exps {
        if let Exp::Boolean(false) = e {} else {
            return Ok(Exp::Boolean(true))
        }
    }
    Ok(Exp::Boolean(false))
}

#[allow(unused_mut)]
pub fn prim_and(env: &mut Env, args: Exp) -> Result<Exp, LispErr> {
    let exps = destruct!(env, args, "and"; (->..Exp))?;
    for e in exps {
        if let Exp::Boolean(false) = e {
            return Ok(Exp::Boolean(false))
        }
    }
    Ok(Exp::Boolean(true))
}

#[allow(unused_mut)]
pub fn prim_car(env: &mut Env, args: Exp) -> Result<Exp, LispErr> {
    let pair = destruct!(env, args, "car"; (->Exp::Pair))?;
    eval(env, pair.car.as_ref())
}

#[allow(unused_mut)]
pub fn prim_cdr(env: &mut Env, args: Exp) -> Result<Exp, LispErr> {
    let pair = destruct!(env, args, "cdr"; (->Exp::Pair))?;
    eval(env, pair.cdr.as_ref())
}

pub fn prim_cons(env: &mut Env, args: Exp) -> Result<Exp, LispErr> {
    let (car, cdr) = destruct!(env, args, "cons"; (->Exp) (->Exp))?;
    Ok(Exp::Pair(cons(car, cdr)))
}

pub fn prim_list(env: &mut Env, args: Exp) -> Result<Exp, LispErr> {
    if let Exp::Pair(list) = args {
        Ok(Exp::Pair(eval_list(env, list)?))
    } else {
        Ok(Exp::Nil)
    }
}

#[allow(unused_mut)]
pub fn prim_quote(_: &mut Env, args: Exp) -> Result<Exp, LispErr> {
    let datum = destruct!(env, args, ""; (Exp))?;
    Ok(datum)
}

#[allow(unused_mut)]
pub fn prim_display(env: &mut Env, args: Exp) -> Result<Exp, LispErr> {
    let arg = destruct!(env, args, ""; (->Exp))?;
    println!("{}", arg);
    Ok(Exp::Nil)
}