use crate::parser::*;
use crate::runtime::*;
use crate::Exception::*;
use std::fs;

pub fn run(file_name: &str) {
    let mut env = default_env();
    match exec_file(&mut env, file_name) {
        Ok(()) => {}
        Err(e) => println!("{}", e),
    }
}

pub(crate) fn exec_file(env: &mut Env, file_name: &str) -> Result<(), Exn> {
    let code = fs::read_to_string(file_name).expect(&format!("Could not read file: {}", file_name));
    let tokens = lex(&code, Some(file_name.to_owned()));
    let mut exp_and_rest = parse(&tokens)?;
    loop {
        eval(env, &exp_and_rest.0)?;
        if exp_and_rest.1.len() == 0 {
            return Ok(());
        }
        exp_and_rest = parse(exp_and_rest.1)?;
    }
}
