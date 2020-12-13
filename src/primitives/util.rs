#[macro_export]
macro_rules! destruct {
    (@void $tt:tt) => {};
    (@type_name Exp::Number) => { "number" };
    (@type_name Exp::Boolean) => { "boolean" };
    (@type_name Exp::Pair) => { "pair" };
    (@arg($env:ident, $list:ident) (->Exp)) => { // (->Exp)
        eval($env, &*$list.car)?
    };
    (@arg($env:ident, $list:ident) (Exp)) => { // (Exp)
        *$list.car
    };
    (@arg($env:ident, $list:ident) (->..Exp)) => { // (->..Exp)
        {
            let mut vec: Vec<Exp> = Vec::new();
            vec.push(eval($env, &*$list.car)?);
            while let Exp::Pair(cdr) = *$list.cdr {
                $list = cdr;
                vec.push(eval($env, &*$list.car)?);
            }
            vec
        }
    };
    (@arg($env:ident, $list:ident) (..Exp)) => { // (..Exp)
        {
            let mut vec: Vec<Exp> = Vec::new();
            vec.push(*$list.car);
            while let Exp::Pair(cdr) = *$list.cdr {
                $list = cdr;
                vec.push(*$list.car);
            }
            vec
        }
    };
    (@arg($env:ident, $list:ident) (->..$pat:path)) => { // (->..Exp::Pair)
        {
            let mut vec = Vec::new();
            if let $pat(x) = eval($env, &*$list.car)? {
                vec.push(x);
            } else {
                todo!()
            }
            while let Exp::Pair(cdr) = *$list.cdr {
                $list = cdr;
                if let $pat(x) = eval($env, &*$list.car)? {
                    vec.push(x);
                } else {
                    todo!()
                }
            }
            vec
        }
    };
    (@arg($env:ident, $list:ident) (..$pat:path)) => { // (..Exp::Pair)
        {
            let mut vec = Vec::new();
            if let $pat(x) = e {
                vec.push(x);
            } else {
                todo!()
            }
            while let Exp::Pair(cdr) = *$list.cdr {
                $list = cdr;
                if let $pat(x) = e {
                    vec.push(x);
                } else {
                    todo!()
                }
            }
            vec
        }
    };
    (@arg($env:ident, $list:ident) (->$pat:path)) => { // (->Exp::Pair)
        if let $pat(x) = eval($env, &*$list.car)? {
            x
        } else {
            todo!()
        }
    };
    (@arg($env:ident, $list:ident) ($pat:path)) => { // (Exp::Pair)
        if let $pat(x) = *$list.car {
            x
        } else {
            todo!()
        }
    };
    ($env:ident, $ex:expr, $err:literal; $first:tt $($arg:tt)*) => {
        if let Exp::Pair(mut list) = $ex {
            let (mut expected_args, mut received_args) = (1, 1);
            $(destruct!(@void $arg); expected_args += 1usize;)* // count expected number of arguments
            let result = (
                destruct!(@arg($env, list) $first)
                $(
                    ,{
                        received_args += 1usize;
                        list = if let Exp::Pair(cdr) = *list.cdr {
                            Ok(cdr)
                        } else {
                            Err(LispErr::Reason(format!("Expected {} arguments and got {}", expected_args, received_args)))
                        }?;
                        destruct!(@arg($env, list) $arg)
                    }
                )*
            );
            if let Exp::Pair(cdr) = *list.cdr {
                list = cdr;
                while let Exp::Pair(cdr) = *list.cdr {
                    list = cdr;
                    received_args += 1;
                }
                Err(LispErr::Reason(format!("Expected {} arguments and got {}", expected_args, received_args)))
            } else { // got too many arguments
                Ok(result)
            }
        } else {
            Err(LispErr::Reason("primitive procedure did not receive a list as input".to_string()))
        }
    };
}
