pub mod repl {
    use std::io::{stdin, stdout, Write};
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

    pub fn repl_main() {
        // Create database
        let mut db = Database::new();
        loop {
            // Print entry carrot
            print!("> ");
            // Flush stdout buffer
            let _ = stdout().flush();
            // Container for line
            let mut s = String::new();
            // Read line
            stdin().read_line(&mut s).expect("Did not enter a correct string");
            // Query database
            let res = db.execute(s);
            // Handle response
            match res {
                QueryResult::Success(s) => println!("{}", s),
                QueryResult::Table(_) => println!("Table"),
                QueryResult::Value(v) => println!("{}", pretty_print(v)),
                QueryResult::Error(s) => println!("Error: {}", s),
                QueryResult::Exit => break
            }
        }
    }
}