use crate::ast::*;
use crate::parser::*;
use crate::runtime::*;
use crate::Exception::*;
use rustyline::error::ReadlineError;
use rustyline::{Config, EditMode, Editor};

fn parse_eval(code: String, env: &mut Env) -> Result<Item, Exn> {
    let (parsed_exp, _) = parse(&lex(&code, None))?;
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
                    Err(e) => println!("{}", e),
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
