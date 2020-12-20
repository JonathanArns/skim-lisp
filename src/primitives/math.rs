use crate::ast::*;
use crate::Exception::*;
use crate::runtime::*;

#[allow(unused_mut)]
pub fn prim_plus(env: &mut Env, args: Exp) -> Result<Exp, Exn> {
    let mut res = 0.0;
    let list = destruct!(env, args, ""; (->..Exp::Number))?;
    for x in list {
        res += x;
    }
    Ok(Exp::Number(res))
}

pub fn prim_minus(env: &mut Env, args: Exp) -> Result<Exp, Exn> {
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
