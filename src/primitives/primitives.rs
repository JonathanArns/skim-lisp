use crate::ast::*;
use crate::runtime::*;
use crate::Exception::*;
use std::rc::Rc;

pub fn prim_define(env: &mut Env, meta: Meta, args: Item) -> Result<Item, Exn> {
    let (left, right) = destruct!(env, args, meta; (Item) (Item))?;
    match left.exp {
        Exp::Symbol(key) => {
            let val = eval(env, &right)?;
            env.set(key, val);
            Ok(Item::new(meta, Exp::Nil))
        }
        Exp::Pair(_) => {
            let (key, params) = destruct!(env, left, meta; (Exp::Symbol) (..Exp::Symbol))?;
            let lambda = Exp::Lambda(Lambda {
                params: Rc::new(params),
                body: Rc::new(right),
            });
            env.set(key, Item::new(left.meta, lambda));
            Ok(Item::new(meta, Exp::Nil))
        }
        _ => Err(Exn::typ(
            left.meta,
            "symbol",
            &left.exp.type_name(),
        )),
    }
}

#[allow(unused_mut)]
pub fn prim_lambda(_: &mut Env, meta: Meta, args: Item) -> Result<Item, Exn> {
    let (params, body) = destruct!(env, args, meta; (Item) (Item))?;
    let param_names = destruct!(env, params, meta; (..Exp::Symbol))?;
    Ok(Item::new(meta, Exp::Lambda(Lambda {
        params: Rc::new(param_names),
        body: Rc::new(body),
    })))
}

pub fn prim_if(env: &mut Env, meta: Meta, args: Item) -> Result<Item, Exn> {
    let (test, then, els) = destruct!(env, args, meta; (->Exp) (Item) (Item))?;
    if let Exp::Boolean(false) = test.exp {
        Ok(Item::new(meta, eval(env, &els).map(|i| i.exp)?))
    } else {
        Ok(Item::new(meta, eval(env, &then).map(|i| i.exp)?))
    }
}

#[allow(unused_mut)]
pub fn prim_cond(env: &mut Env, meta: Meta, args: Item) -> Result<Item, Exn> {
    let branches = destruct!(env, args, meta; (..Item))?;
    for branch in branches {
        let (car, cdr) = destruct!(env, branch, meta; (Item) (..Item))?;
        if let Exp::Symbol(ref s) = car.exp {
            if s == "else" {
                if cdr.len() == 0 {
                    return Err(Exn::arity(args.meta, 1, 0));
                }
                let mut result = Exp::Nil;
                for body in cdr {
                    result = eval(env, &body)?.exp;
                }
                return Ok(Item::new(meta, result));
            }
        }
        if let Exp::Boolean(false) = eval(env, &car)?.exp {
        } else {
            let mut result = Exp::Nil;
            for body in cdr {
                result = eval(env, &body)?.exp;
            }
            return Ok(Item::new(meta, result));
        }
    }
    Ok(Item::new(meta, Exp::Nil))
}

#[allow(unused_mut)]
pub fn prim_or(env: &mut Env, meta: Meta, args: Item) -> Result<Item, Exn> {
    let exps = destruct!(env, args, meta; (->..Exp))?;
    for e in exps {
        if let Exp::Boolean(false) = e {
        } else {
            return Ok(Item::new(meta, Exp::Boolean(true)));
        }
    }
    Ok(Item::new(meta, Exp::Boolean(false)))
}

#[allow(unused_mut)]
pub fn prim_and(env: &mut Env, meta: Meta, args: Item) -> Result<Item, Exn> {
    let exps = destruct!(env, args, meta; (->..Exp))?;
    for e in exps {
        if let Exp::Boolean(false) = e {
            return Ok(Item::new(meta, Exp::Boolean(false)));
        }
    }
    Ok(Item::new(meta, Exp::Boolean(true)))
}

#[allow(unused_mut)]
pub fn prim_car(env: &mut Env, meta: Meta, args: Item) -> Result<Item, Exn> {
    let pair = destruct!(env, args, meta; (->Exp::Pair))?;
    Ok(Item::new(meta, eval(env, pair.car.as_ref()).map(|i| i.exp)?))
}

#[allow(unused_mut)]
pub fn prim_cdr(env: &mut Env, meta: Meta, args: Item) -> Result<Item, Exn> {
    let pair = destruct!(env, args, meta; (->Exp::Pair))?;
    Ok(Item::new(meta, eval(env, pair.cdr.as_ref()).map(|i| i.exp)?))
}

pub fn prim_cons(env: &mut Env, meta: Meta, args: Item) -> Result<Item, Exn> {
    let (car, cdr) = destruct!(env, args, meta; (->Exp) (->Exp))?;
    Ok(Item::new(meta, Exp::Pair(cons(car, cdr))))
}

pub fn prim_list(env: &mut Env, meta: Meta, args: Item) -> Result<Item, Exn> {
    if let Exp::Pair(list) = args.exp {
        Ok(Item::new(meta, Exp::Pair(eval_list(env, list)?)))
    } else {
        Ok(Item::new(meta, Exp::Nil))
    }
}

#[allow(unused_mut)]
pub fn prim_quote(_: &mut Env, meta: Meta, args: Item) -> Result<Item, Exn> {
    let datum = destruct!(env, args, meta; (Exp))?;
    Ok(Item::new(meta, datum))
}

#[allow(unused_mut)]
pub fn prim_display(env: &mut Env, meta: Meta, args: Item) -> Result<Item, Exn> {
    let arg = destruct!(env, args, meta; (->Exp))?;
    println!("{}", arg);
    Ok(Item::new(meta, Exp::Nil))
}
