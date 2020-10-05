use crate::ast::*;
use crate::parser::*;
use crate::runtime::*;
use rustyline::error::ReadlineError;
use rustyline::{Config, EditMode, Editor};

fn parse_eval(expr: String, env: &mut Env) -> Result<Exp, LispErr> {
    let (parsed_exp, _) = parse(&lex(&expr))?;
    Ok(eval(env, &parsed_exp)?)
}

pub fn repl() {
    let config = Config::builder().edit_mode(EditMode::Vi).build();
    let mut env = default_env();
    let mut rl = Editor::<()>::with_config(config);
    // if rl.load_history("history.txt").is_err() {
    //     println!("No previous history.");
    // }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match parse_eval(line, &mut env) {
                    Ok(res) => println!("{}", res),
                    Err(e) => match e {
                        LispErr::UnexpectedToken(msg) | LispErr::Reason(msg) => println!("{}", msg),
                        LispErr::Bug(msg) => println!("Bug! {}", msg),
                    },
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    // rl.save_history("history.txt").unwrap();
}
