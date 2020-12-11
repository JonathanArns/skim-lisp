#[macro_export]
macro_rules! destruct {
    // (@step($list:ident) ()) => {
    //     if let Exp::Nil = *$list.cdr {
            
    //     }
    // };
    (@type_name Exp::Number) => { "number" };
    (@type_name Exp::Boolean) => { "boolean" };
    (@type_name Exp::Pair) => { "pair" };
    (@step($env:ident, $list:ident) (->Exp)) => { // ...(->Exp)...
        {
            $list = if let Exp::Pair(cdr) = *$list.cdr {
                cdr
            } else {
                todo!()
            };
            eval($env, &*$list.car)?
        }
    };
    (@step($env:ident, $list:ident) (Exp)) => { // ...(Exp)...
        {
            $list = if let Exp::Pair(cdr) = *$list.cdr {
                cdr
            } else {
                todo!()
            };
            *$list.car
        }
    };
    (@first($env:ident, $list:ident) (->Exp)) => { // (->Exp)...
        {
            eval($env, &*$list.car)?
        }
    };
    (@first($env:ident, $list:ident) (Exp)) => { // (Exp)...
        {
            *$list.car
        }
    };
    (@first($env:ident, $list:ident) (->..Exp)) => { // (->..Exp)
        {
            let mut vec: Vec<Exp> = Vec::new();
            for e in $list {
                vec.push(eval($env, &e)?);
            }
            vec
        }
    };
    (@first($env:ident, $list:ident) (..Exp)) => { // (..Exp)
        {
            let mut vec: Vec<Exp> = Vec::new();
            for e in $list {
                vec.push(e);
            }
            vec
        }
    };
    (@step($env:ident, $list:ident) (->..Exp)) => { // ...(->..Exp)
        {
            $list = if let Exp::Pair(cdr) = *$list.cdr {
                cdr
            } else {
                todo!()
            };
            let mut vec: Vec<Exp> = Vec::new();
            for e in $list {
                vec.push(eval($env, &e)?);
            }
            vec
        }
    };
    (@step($env:ident, $list:ident) (..Exp)) => { // ...(..Exp)
        {
            $list = if let Exp::Pair(cdr) = *$list.cdr {
                cdr
            } else {
                todo!()
            };
            let mut vec: Vec<Exp> = Vec::new();
            for e in $list {
                vec.push(e);
            }
            vec
        }
    };
    (@first($env:ident, $list:ident) (->..$pat:path)) => { // (->..Exp::Pair)
        {
            let mut vec = Vec::new();
            for e in $list {
                if let $pat(x) = eval($env, &e)? {
                    vec.push(x);
                } else {
                    todo!()
                }
            }
            vec
        }
    };
    (@first($env:ident, $list:ident) (..$pat:path)) => { // (..Exp::Pair)
        {
            let mut vec = Vec::new();
            for e in $list {
                if let $pat(x) = e {
                    vec.push(x);
                } else {
                    todo!()
                }
            }
            vec
        }
    };
    (@step($env:ident, $list:ident) (->..$pat:path)) => { // ...(->..Exp::Pair)
        {
            $list = if let Exp::Pair(cdr) = *$list.cdr {
                cdr
            } else {
                todo!()
            };
            let mut vec = Vec::new();
            for e in $list {
                if let $pat(x) = eval($env, &e)? {
                    vec.push(x);
                } else {
                    todo!()
                }
            }
            vec
        }
    };
    (@step($env:ident, $list:ident) (..$pat:path)) => { // ...(..Exp::Pair)
        {
            $list = if let Exp::Pair(cdr) = *$list.cdr {
                cdr
            } else {
                todo!()
            };
            let mut vec = Vec::new();
            for e in $list {
                if let $pat(x) = e {
                    vec.push(x);
                } else {
                    todo!()
                }
            }
            vec
        }
    };
    (@step($env:ident, $list:ident) (->$pat:path)) => { // ...(->Exp::Pair)...
        {
            $list = if let Exp::Pair(cdr) = *$list.cdr {
                cdr
            } else {
                todo!()
            };
            if let $pat(x) = eval($env, &*$list.car)? {
                x
            } else {
                todo!()
            }
        }
    };
    (@step($env:ident, $list:ident) ($pat:path)) => { // ...(Exp::Pair)...
        {
            $list = if let Exp::Pair(cdr) = *$list.cdr {
                cdr
            } else {
                todo!()
            };
            if let $pat(x) = *$list.car {
                x
            } else {
                todo!()
            }
        }
    };
    (@first($env:ident, $list:ident) (->$pat:path)) => { // (->Exp::Pair)...
        {
            if let $pat(x) = eval($env, &*$list.car)? {
                x
            } else {
                todo!()
            }
        }
    };
    (@first($env:ident, $list:ident) ($pat:path)) => { // (Exp::Pair)...
        {
            if let $pat(x) = *$list.car {
                x
            } else {
                todo!()
            }
        }
    };
    ($env:ident, $ex:expr, $err:literal; $first:tt $($arg:tt)*) => {
        if let Exp::Pair(mut list) = $ex {
            Ok((
                destruct!(@first($env, list) $first)
                $(
                    ,destruct!(@step($env, list) $arg)
                )*
            ))
        } else {
            Err(LispErr::Reason("primitive procedure did not receive a list as input".to_string()))
        }
    };
}
