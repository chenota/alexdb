mod sqlscript;
mod storage;
mod engine;
mod repl;

use crate::repl::repl::repl::*;

fn main() {
    match repl_main() {
        Ok(_) => std::process::exit(0),
        _ => std::process::exit(1)
    };
}
