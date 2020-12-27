use crate::ast::*;
use crate::runtime::*;
use crate::Exception::*;

#[allow(unused_mut)]
pub fn prim_plus(env: &mut Env, meta: Meta, args: Item) -> Result<Item, Exn> {
    let mut res = 0.0;
    let list = destruct!(env, args, meta; (->..Exp::Number))?;
    for x in list {
        res += x;
    }
    Ok(Item::new(meta, Exp::Number(res)))
}

pub fn prim_minus(env: &mut Env, meta: Meta, args: Item) -> Result<Item, Exn> {
    let (arg0, rest) = destruct!(env, args, meta; (->Exp::Number) (->..Exp::Number))?;
    let mut res = arg0;
    for x in rest {
        res -= x;
    }
    if arg0 == res {
        Ok(Item::new(meta, Exp::Number(-res)))
    } else {
        Ok(Item::new(meta, Exp::Number(res)))
    }
}
