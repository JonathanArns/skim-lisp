use crate::ast::*;
use crate::primitives::primitives::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Env<'a> {
    map: HashMap<String, Exp>,
    outer: Option<&'a Env<'a>>,
}

impl<'a> Env<'a> {
    pub fn set(&mut self, key: String, val: Exp) {
        self.map.insert(key, val);
    }

    pub fn get(&self, key: &str) -> Option<&Exp> {
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
    env.set("+".to_string(), Exp::Primitive(prim_plus));
    env.set("-".to_string(), Exp::Primitive(prim_minus));
    env.set("define".to_string(), Exp::Primitive(prim_define));
    env.set("lambda".to_string(), Exp::Primitive(prim_lambda));
    env.set("cond".to_string(), Exp::Primitive(prim_cond));
    env.set("car".to_string(), Exp::Primitive(prim_car));
    env.set("cdr".to_string(), Exp::Primitive(prim_cdr));
    env.set("cons".to_string(), Exp::Primitive(prim_cons));
    env.set("list".to_string(), Exp::Primitive(prim_list));
    env.set("quote".to_string(), Exp::Primitive(prim_quote));
    env
}

pub fn eval<'a>(env: &'a mut Env, exp: &Exp) -> Result<Exp, LispErr> {
    match exp {
        Exp::Pair(x) => {
            if let (Exp::Nil, Exp::Nil) = (&*x.car, &*x.cdr) {
                Ok(Exp::Nil)
            } else {
                apply_function(env, x.clone())
            }
        }
        Exp::Symbol(s) => lookup_symbol(env, &s),
        Exp::Nil
        | Exp::Number(_)
        | Exp::Lambda(_)
        | Exp::Primitive(_)
        | Exp::String(_)
        | Exp::Char(_)
        | Exp::Vector(_)
        | Exp::Boolean(_) => Ok(exp.clone()), // self evaluating
    }
}

fn apply_function(env: &mut Env, list: LispCell) -> Result<Exp, LispErr> {
    match eval(env, &*list.car)? {
        Exp::Primitive(prim) => prim(env, *list.cdr),
        Exp::Lambda(lambda) => {
            let mut scope = if let Exp::Pair(param_names) = lambda.params.as_ref() {
                let mut args_iter = if let Exp::Pair(args) = *list.cdr {
                    eval_list(env, args)?.into_iter()
                } else {
                    ListIter { list: None }
                };
                let mut scope = env.new_scope();
                // TODO: this might violate function call semantics
                // in terms of calling with too few or too many arguments
                for param in param_names.clone().into_iter() {
                    if let Exp::Symbol(sym) = param {
                        if let Some(arg) = args_iter.next() {
                            scope.set(sym, arg);
                        } else {
                            return Err(LispErr::Reason(format!(
                                "Function {} called with too few arguments",
                                list.car
                            )));
                        }
                    }
                }
                if let Some(_) = args_iter.next() {
                    return Err(LispErr::Reason(format!(
                        "Function {} called with too many arguments",
                        list.car
                    )));
                }
                scope
            } else {
                return Err(LispErr::Bug(
                    "Lambda does not have a params list".to_string(),
                ));
            };
            eval(&mut scope, lambda.body.as_ref())
        }
        x => Err(LispErr::Reason(format!("{} is not a callable function", x))),
    }
}

pub fn eval_list(env: &mut Env, list: LispCell) -> Result<LispCell, LispErr> {
    let mut res = list;
    let mut rest = &mut res;
    loop {
        rest.car = Box::new(eval(env, &mut rest.car.clone())?);
        if let Exp::Pair(ref mut next) = &mut *rest.cdr {
            rest = next;
        } else {
            break;
        }
    }
    Ok(res)
}

fn lookup_symbol<'a>(env: &'a Env, sym: &str) -> Result<Exp, LispErr> {
    Ok(env
        .get(sym)
        .ok_or(LispErr::Reason(format!("undefined symbol: {}", sym)))?
        .clone())
}
