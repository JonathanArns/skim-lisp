use skim::repl::repl;
use skim::run::run;
use clap::{Arg, App};
fn main() {
    let matches = App::new("skim-lisp")
        .version("0.0.1")
        .author("Jonathan Arns")
        .about("a small toy lisp implementation")
        .arg(Arg::with_name("file")
            .index(1)
            .help("The lisp script to execute"))
        .get_matches();
    
    match matches.value_of("file") {
        None => repl(),
        Some(file) => run(file),
    }
}
