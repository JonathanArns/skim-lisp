use crate::ast::*;
use crate::Exception::*;
use crate::runtime::*;
use std::rc::Rc;

pub fn prim_define(env: &mut Env, args: Exp) -> Result<Exp, Exn> {
    let (left, right) = destruct!(env, args, "define"; (Exp) (Exp))?;
    match left {
        Exp::Symbol(key) => {
            let val = eval(env, &right)?;
            env.set(key, val);
            Ok(Exp::Nil)
        }
        Exp::Pair(_) => {
            let (key , params) = destruct!(env, left, "define"; (Exp::Symbol) (..Exp::Symbol))?;
            let lambda = Exp::Lambda(Lambda {
                params: Rc::new(params),
                body: Rc::new(right),
            });
            env.set(key, lambda);
            Ok(Exp::Nil)
        }
        _ => Err(Exn::typ_unknown("define", "symbol", &left.type_name())),
    }
}

#[allow(unused_mut)]
pub fn prim_lambda(_: &mut Env, args: Exp) -> Result<Exp, Exn> {
    let (params, body) = destruct!(env, args, "lambda"; (Exp) (Exp))?;
    let param_names = destruct!(env, params, "lambda"; (..Exp::Symbol))?;
    Ok(Exp::Lambda(Lambda {
        params: Rc::new(param_names),
        body: Rc::new(body),
    }))
}

pub fn prim_if(env: &mut Env, args: Exp) -> Result<Exp, Exn> {
    let (test, then, els) = destruct!(env, args, "if"; (->Exp) (Exp) (Exp))?;
    if let Exp::Boolean(false) = test {
        eval(env, &els)
    } else {
        eval(env, &then)
    }
}

#[allow(unused_mut)]
pub fn prim_cond(env: &mut Env, args: Exp) -> Result<Exp, Exn> {
    let branches = destruct!(env, args, "cond"; (..Exp))?;
    for branch in branches {
        let (car, cdr) = destruct!(env, branch, "cond"; (Exp) (..Exp))?;
        if let Exp::Symbol(ref s) = car {
            if s == "else" {
                if cdr.len() == 0 {
                    return Err(Exn::arity_unknown("cond", 1, 0))
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
pub fn prim_or(env: &mut Env, args: Exp) -> Result<Exp, Exn> {
    let exps = destruct!(env, args, "or"; (->..Exp))?;
    for e in exps {
        if let Exp::Boolean(false) = e {} else {
            return Ok(Exp::Boolean(true))
        }
    }
    Ok(Exp::Boolean(false))
}

#[allow(unused_mut)]
pub fn prim_and(env: &mut Env, args: Exp) -> Result<Exp, Exn> {
    let exps = destruct!(env, args, "and"; (->..Exp))?;
    for e in exps {
        if let Exp::Boolean(false) = e {
            return Ok(Exp::Boolean(false))
        }
    }
    Ok(Exp::Boolean(true))
}

#[allow(unused_mut)]
pub fn prim_car(env: &mut Env, args: Exp) -> Result<Exp, Exn> {
    let pair = destruct!(env, args, "car"; (->Exp::Pair))?;
    eval(env, pair.car.as_ref())
}

#[allow(unused_mut)]
pub fn prim_cdr(env: &mut Env, args: Exp) -> Result<Exp, Exn> {
    let pair = destruct!(env, args, "cdr"; (->Exp::Pair))?;
    eval(env, pair.cdr.as_ref())
}

pub fn prim_cons(env: &mut Env, args: Exp) -> Result<Exp, Exn> {
    let (car, cdr) = destruct!(env, args, "cons"; (->Exp) (->Exp))?;
    Ok(Exp::Pair(cons(car, cdr)))
}

pub fn prim_list(env: &mut Env, args: Exp) -> Result<Exp, Exn> {
    if let Exp::Pair(list) = args {
        Ok(Exp::Pair(eval_list(env, list)?))
    } else {
        Ok(Exp::Nil)
    }
}

#[allow(unused_mut)]
pub fn prim_quote(_: &mut Env, args: Exp) -> Result<Exp, Exn> {
    let datum = destruct!(env, args, ""; (Exp))?;
    Ok(datum)
}

#[allow(unused_mut)]
pub fn prim_display(env: &mut Env, args: Exp) -> Result<Exp, Exn> {
    let arg = destruct!(env, args, ""; (->Exp))?;
    println!("{}", arg);
    Ok(Exp::Nil)
}