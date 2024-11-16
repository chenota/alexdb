pub mod repl {
    use std::io::{stdin, stdout, Write};
    use crate::engine::database::engine::*;

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
            // Print response
            println!("{}", match res {
                QueryResult::None => "Success",
                QueryResult::Table(_) => "Table",
                QueryResult::Value(_) => "Value"
            })
        }
    }
}