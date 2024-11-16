pub mod repl {
    use rustyline::{DefaultEditor, Result};
    use rustyline::error::ReadlineError;
    use crate::engine::database::engine::*;
    use crate::sqlscript::types::types::Val;

    fn pretty_print(v: Val) -> String {
        match v {
            Val::StrVal(s) => "'".to_string() + &s + "'",
            Val::BoolVal(b) => if b { "true".to_string() } else { "false".to_string() },
            Val::NumVal(n) => if f64::is_nan(n) { "NaN".to_string() } else if f64::is_infinite(n) { "inf".to_string() } else { n.to_string() },
            Val::NullVal => "null".to_string(),
            Val::UndefVal => "undefined".to_string(),
            Val::ClosureVal(_, _, _) => "[Function]".to_string(),
            Val::TupVal(v) => {
                let mut inners = Vec::new();
                for val in v { inners.push(pretty_print(val.as_ref().clone())) };
                "[".to_string() + &inners.join(", ") + "]"
            }
        }   
    }

    pub fn repl_main() -> Result<()> {
        // Create database
        let mut db = Database::new();
        // Create editor
        let mut rl = DefaultEditor::new()?;
        loop {
            // Readline
            let readline = rl.readline("> ");
            // Act
            match readline {
                Ok(line) => {
                    let res = db.execute(line);
                    // Handle response
                    match res {
                        QueryResult::Success(s) => println!("{}", s),
                        QueryResult::Table(_) => println!("Table"),
                        QueryResult::Value(v) => println!("{}", pretty_print(v)),
                        QueryResult::Error(s) => println!("Error: {}", s),
                        QueryResult::Exit => break
                    }
                },
                Err(ReadlineError::Interrupted) => {
                    println!("SIGINT");
                    break
                },
                Err(ReadlineError::Eof) => {
                    println!("EOF");
                    break
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                    break
                }
            }
        };
        Ok(())
    }
}