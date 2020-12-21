use crate::ast::*;
use crate::runtime::*;
use crate::Exception::*;
use std::rc::Rc;

pub fn prim_define(env: &mut Env, args: Item) -> Result<Exp, Exn> {
    let (left, right) = destruct!(env, args, "define"; (Item) (Item))?;
    match left.exp {
        Exp::Symbol(key) => {
            let val = eval(env, &right)?;
            env.set(key, val);
            Ok(Exp::Nil)
        }
        Exp::Pair(_) => {
            let (key, params) = destruct!(env, left, "define"; (Exp::Symbol) (..Exp::Symbol))?;
            let lambda = Exp::Lambda(Lambda {
                params: Rc::new(params),
                body: Rc::new(right),
            });
            env.set(key, Item::new(left.meta, lambda));
            Ok(Exp::Nil)
        }
        _ => Err(Exn::typ_unknown("define", "symbol", &left.exp.type_name())),
    }
}

#[allow(unused_mut)]
pub fn prim_lambda(_: &mut Env, args: Item) -> Result<Exp, Exn> {
    let (params, body) = destruct!(env, args, "lambda"; (Item) (Item))?;
    let param_names = destruct!(env, params, "lambda"; (..Exp::Symbol))?;
    Ok(Exp::Lambda(Lambda {
        params: Rc::new(param_names),
        body: Rc::new(body),
    }))
}

pub fn prim_if(env: &mut Env, args: Item) -> Result<Exp, Exn> {
    let (test, then, els) = destruct!(env, args, "if"; (->Exp) (Item) (Item))?;
    if let Exp::Boolean(false) = test.exp {
        eval(env, &els).map(|i| i.exp)
    } else {
        eval(env, &then).map(|i| i.exp)
    }
}

#[allow(unused_mut)]
pub fn prim_cond(env: &mut Env, args: Item) -> Result<Exp, Exn> {
    let branches = destruct!(env, args, "cond"; (..Item))?;
    for branch in branches {
        let (car, cdr) = destruct!(env, branch, "cond"; (Item) (..Item))?;
        if let Exp::Symbol(ref s) = car.exp {
            if s == "else" {
                if cdr.len() == 0 {
                    return Err(Exn::arity_unknown("cond", 1, 0));
                }
                let mut result = Exp::Nil;
                for body in cdr {
                    result = eval(env, &body)?.exp;
                }
                return Ok(result);
            }
        }
        if let Exp::Boolean(false) = eval(env, &car)?.exp {
        } else {
            let mut result = Exp::Nil;
            for body in cdr {
                result = eval(env, &body)?.exp;
            }
            return Ok(result);
        }
    }
    Ok(Exp::Nil)
}

#[allow(unused_mut)]
pub fn prim_or(env: &mut Env, args: Item) -> Result<Exp, Exn> {
    let exps = destruct!(env, args, "or"; (->..Exp))?;
    for e in exps {
        if let Exp::Boolean(false) = e {
        } else {
            return Ok(Exp::Boolean(true));
        }
    }
    Ok(Exp::Boolean(false))
}

#[allow(unused_mut)]
pub fn prim_and(env: &mut Env, args: Item) -> Result<Exp, Exn> {
    let exps = destruct!(env, args, "and"; (->..Exp))?;
    for e in exps {
        if let Exp::Boolean(false) = e {
            return Ok(Exp::Boolean(false));
        }
    }
    Ok(Exp::Boolean(true))
}

#[allow(unused_mut)]
pub fn prim_car(env: &mut Env, args: Item) -> Result<Exp, Exn> {
    let pair = destruct!(env, args, "car"; (->Exp::Pair))?;
    eval(env, pair.car.as_ref()).map(|i| i.exp)
}

#[allow(unused_mut)]
pub fn prim_cdr(env: &mut Env, args: Item) -> Result<Exp, Exn> {
    let pair = destruct!(env, args, "cdr"; (->Exp::Pair))?;
    eval(env, pair.cdr.as_ref()).map(|i| i.exp)
}

pub fn prim_cons(env: &mut Env, args: Item) -> Result<Exp, Exn> {
    let (car, cdr) = destruct!(env, args, "cons"; (->Exp) (->Exp))?;
    Ok(Exp::Pair(cons(car, cdr)))
}

pub fn prim_list(env: &mut Env, args: Item) -> Result<Exp, Exn> {
    if let Exp::Pair(list) = args.exp {
        Ok(Exp::Pair(eval_list(env, list)?))
    } else {
        Ok(Exp::Nil)
    }
}

#[allow(unused_mut)]
pub fn prim_quote(_: &mut Env, args: Item) -> Result<Exp, Exn> {
    let datum = destruct!(env, args, ""; (Exp))?;
    Ok(datum)
}

#[allow(unused_mut)]
pub fn prim_display(env: &mut Env, args: Item) -> Result<Exp, Exn> {
    let arg = destruct!(env, args, ""; (->Exp))?;
    println!("{}", arg);
    Ok(Exp::Nil)
}
