use crate::ast::*;
use crate::primitives::*;
use crate::Exception::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Env<'a> {
    map: HashMap<String, Item>,
    outer: Option<&'a Env<'a>>,
}

impl<'a> Env<'a> {
    pub fn set(&mut self, key: String, val: Item) {
        self.map.insert(key, val);
    }

    pub fn get(&self, key: &str) -> Option<&Item> {
        let mut env = self;
        loop {
            let x = env.map.get(key);
            if let Some(_) = x {
                return x;
            }
            if let Some(outer) = env.outer {
                env = outer;
            } else {
                return None;
            }
        }
    }

    pub fn new_scope(&self) -> Env {
        Env {
            map: HashMap::new(),
            outer: Some(self),
        }
    }
}

pub fn default_env() -> Env<'static> {
    let mut env = Env {
        map: HashMap::new(),
        outer: None,
    };
    env.set("+".to_string(), Item::prim(Meta::empty(), prim_plus));
    env.set("-".to_string(), Item::prim(Meta::empty(), prim_minus));
    env.set("define".to_string(), Item::prim(Meta::empty(), prim_define));
    env.set("lambda".to_string(), Item::prim(Meta::empty(), prim_lambda));
    env.set("if".to_string(), Item::prim(Meta::empty(), prim_if));
    env.set("cond".to_string(), Item::prim(Meta::empty(), prim_cond));
    env.set("or".to_string(), Item::prim(Meta::empty(), prim_or));
    env.set("and".to_string(), Item::prim(Meta::empty(), prim_and));
    env.set("car".to_string(), Item::prim(Meta::empty(), prim_car));
    env.set("cdr".to_string(), Item::prim(Meta::empty(), prim_cdr));
    env.set("cons".to_string(), Item::prim(Meta::empty(), prim_cons));
    env.set("list".to_string(), Item::prim(Meta::empty(), prim_list));
    env.set("quote".to_string(), Item::prim(Meta::empty(), prim_quote));
    env.set(
        "display".to_string(),
        Item::prim(Meta::empty(), prim_display),
    );
    env
}

pub fn eval<'a>(env: &'a mut Env, item: &Item) -> Result<Item, Exn> {
    match item.exp.to_owned() {
        Exp::Pair(x) => {
            if let (Exp::Nil, Exp::Nil) = (&x.car.exp, &x.cdr.exp) {
                Ok(Item::new(item.meta.clone(), Exp::Nil))
            } else {
                apply_function(env, x.clone(), item.meta.clone())
            }
        }
        Exp::Symbol(s) => lookup_symbol(env, &s).ok_or(Exn::other(
            item.meta.clone(),
            "tried to look up undefined symbol",
        )),
        Exp::Nil
        | Exp::Number(_)
        | Exp::Lambda(_)
        | Exp::Primitive(_)
        | Exp::String(_)
        | Exp::Char(_)
        | Exp::Vector(_)
        | Exp::Boolean(_) => Ok(item.clone()), // self evaluating
    }
}

fn apply_function(env: &mut Env, list: LispCell, meta: Meta) -> Result<Item, Exn> {
    match eval(env, &list.car)?.exp {
        Exp::Primitive(prim) => Ok(prim(env, meta, *list.cdr)?),
        Exp::Lambda(lambda) => {
            let mut args_iter = if let Exp::Pair(args) = list.cdr.exp {
                eval_list(env, args)?.into_iter()
            } else {
                ListIter { list: None }
            };
            let mut scope = env.new_scope();
            let mut num_args_found = 0;
            for param in lambda.params.as_ref() {
                if let Some(arg) = args_iter.next() {
                    scope.set(param.clone(), arg);
                    num_args_found += 1;
                } else {
                    return Err(Exn::arity(
                        meta,
                        lambda.params.len(),
                        num_args_found,
                    ));
                }
            }
            if let Some(_) = args_iter.next() {
                return Err(Exn::arity(
                    meta,
                    lambda.params.len(),
                    num_args_found,
                ));
            }
            eval(&mut scope, lambda.body.as_ref())
        }
        x => Err(Exn::typ(meta, "procedure", &x.type_name())),
    }
}

pub fn eval_list(env: &mut Env, list: LispCell) -> Result<LispCell, Exn> {
    let mut res = list;
    let mut rest = &mut res;
    loop {
        rest.car = Box::new(eval(env, &mut rest.car.clone())?);
        if let Exp::Pair(ref mut next) = rest.cdr.exp {
            rest = next;
        } else {
            break;
        }
    }
    Ok(res)
}

fn lookup_symbol<'a>(env: &'a Env, sym: &str) -> Option<Item> {
    env.get(sym).cloned()
}
