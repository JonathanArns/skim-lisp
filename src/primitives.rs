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

pub fn prim_cond(env: &mut Env, args: Exp) -> Result<Exp, LispErr> {
    let num_args_err = Err(LispErr::Reason(
        "Expected exactly 3 arguments to (cond)".to_string(),
    ));
    if let Exp::Pair(list) = args {
        let mut iter = list.into_iter();
        let condition = if let Some(cond) = iter.next() {
            cond
        } else {
            return num_args_err;
        };
        let true_branch = if let Some(branch) = iter.next() {
            branch
        } else {
            return num_args_err;
        };
        let false_branch = if let Some(branch) = iter.next() {
            branch
        } else {
            return num_args_err;
        };
        if let Some(_) = iter.next() {
            return num_args_err;
        }

        match eval(env, &condition)? {
            Exp::Boolean(false) | Exp::Nil => eval(env, &false_branch),
            _ => eval(env, &true_branch),
        }
    } else {
        num_args_err
    }
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