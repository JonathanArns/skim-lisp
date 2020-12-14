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

pub fn prim_cond(env: &mut Env, args: Exp) -> Result<Exp, LispErr> {
    let num_args_err = Err(LispErr::Reason(
        "Expected exactly 3 arguments to (cond)".to_string(),
    ));
    if let Exp::Pair(list) = args {
        let mut clause_iter = list.into_iter();
        let x = loop {
            match clause_iter.next() {
                Some(Exp::Pair(clause)) => {
                    todo!()
                },
                None => break Ok(Exp::Nil),
                _ => break Err(LispErr::Reason("(cond) only takes lists as arguments".to_string())),
            }
        };
        ///////////////
        let condition = if let Some(cond) = clause_iter.next() {
            cond
        } else {
            return num_args_err;
        };
        let true_branch = if let Some(branch) = clause_iter.next() {
            branch
        } else {
            return num_args_err;
        };
        let false_branch = if let Some(branch) = clause_iter.next() {
            branch
        } else {
            return num_args_err;
        };

        match eval(env, &condition)? {
            Exp::Boolean(false) | Exp::Nil => eval(env, &false_branch),
            _ => eval(env, &true_branch),
        }
    } else {
        num_args_err

pub fn prim_or(env: &mut Env, args: Exp) -> Result<Exp, LispErr> {
    let (left, right) = destruct!(env, args, "or"; (->Exp) (->Exp))?;
    if let Exp::Boolean(true) = left {
        Ok(left)
    } else if let Exp::Boolean(true) = right {
        Ok(right)
    } else {
        Ok(Exp::Boolean(false))
    }
}

pub fn prim_and(env: &mut Env, args: Exp) -> Result<Exp, LispErr> {
    let (left, right) = destruct!(env, args, "or"; (->Exp) (->Exp))?;
    if let Exp::Boolean(true) = left {
        if let Exp::Boolean(true) = right {
            return Ok(right)
        }
    }
    Ok(Exp::Boolean(false))
}

pub fn prim_car(env: &mut Env, args: Exp) -> Result<Exp, LispErr> {
    let arg_err = Err(LispErr::Reason("Expected exactly one argument of type list to (car)".to_string()));
    if let Exp::Pair(LispCell {car, cdr}) = args {
        if let (Exp::Pair(LispCell {car, ..}), Exp::Nil) = (eval(env, &*car)?, *cdr) {
            Ok(*car)
        } else {
            arg_err
        }
    } else {
        arg_err
    }
}

pub fn prim_cdr(env: &mut Env, args: Exp) -> Result<Exp, LispErr> {
    let arg_err = Err(LispErr::Reason("Expected exactly one argument of type list to (cdr)".to_string()));
    if let Exp::Pair(LispCell {car, cdr}) = args {
        if let (Exp::Pair(LispCell {cdr, ..}), Exp::Nil) = (eval(env, &*car)?, *cdr) {
            Ok(*cdr)
        } else {
            arg_err
        }
    } else {
        arg_err
    }
}

pub fn prim_cons(env: &mut Env, args: Exp) -> Result<Exp, LispErr> {
    let num_args_err = Err(LispErr::Reason(
        "Expected exactly 2 arguments to (cons)".to_string(),
    ));
    if let Exp::Pair(list) = args {
        let mut iter = eval_list(env, list)?.into_iter();
        let car = if let Some(car) = iter.next() {
            car
        } else {
            return num_args_err;
        };
        let cdr = if let Some(cdr) = iter.next() {
            cdr
        } else {
            return num_args_err;
        };
        if let Some(_) = iter.next() {
            return num_args_err;
        }
        Ok(Exp::Pair(cons(car, cdr)))
    } else {
        num_args_err
    }
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