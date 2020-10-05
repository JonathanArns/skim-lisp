use crate::ast::*;
use crate::runtime::*;
use std::rc::Rc;

pub fn prim_plus(env: &mut Env, args: Exp) -> Result<Exp, LispErr> {
    let mut res = 0.0;
    if let Exp::Pair(mut list) = args {
        list = eval_list(env, list)?;
        for item in list {
            if let Exp::Number(x) = item {
                res += x;
            } else {
                return Err(LispErr::Reason(
                    "Expected number, got something else".to_string(),
                ));
            }
        }
    }
    Ok(Exp::Number(res))
}

pub fn prim_minus(env: &mut Env, args: Exp) -> Result<Exp, LispErr> {
    let mut res = 0.0;
    let mut arg0 = 0.0;
    if let Exp::Pair(list) = args {
        let mut list_iter = eval_list(env, list)?.into_iter();
        if let Some(Exp::Number(n)) = list_iter.next() {
            res = n;
            arg0 = n;
        }
        for item in list_iter {
            if let Exp::Number(x) = item {
                res -= x;
            } else {
                return Err(LispErr::Reason(
                    "Expected number, got something else".to_string(),
                ));
            }
        }
        if arg0 == res {
            return Ok(Exp::Number(-res));
        } else {
            return Ok(Exp::Number(res));
        }
    }
    Err(LispErr::Reason("Too few arguments".to_string()))
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
