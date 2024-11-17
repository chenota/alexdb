pub mod repl {
    use rustyline::{DefaultEditor, Result};
    use rustyline::error::ReadlineError;
    use crate::engine::database::engine::*;
    use crate::sqlscript::types::types::Val;
    use crate::storage::table::table::*;

    const MAX_PRINT_ROWS: usize = 10;
    const COL_PADDING: usize = 3;

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

    fn print_table(t: Table) -> () {
        // Columns to print
        let mut t_rows = Vec::new();
        // Number of cols
        let num_cols = t.get_headers().len();
        // Longest item in each col
        let mut longest_cols: Vec<usize> = Vec::new();
        // Push space for each column and max width counter
        for h in t.get_headers() { 
            t_rows.push(h.clone()); 
            longest_cols.push(h.len())
        }
        // Iterate through rows
        let mut num_rows: usize = 0;
        for row in t.iter() {
            // Don't print more than max
            if num_rows > MAX_PRINT_ROWS { break }
            // Iterate through values
            for j in 0..num_cols {
                // Convert value to string
                let val_str = pretty_print(row[j].clone());
                // If new value has more characters, replace longest string
                if val_str.len() > longest_cols[j] { longest_cols[j] = val_str.len() }
                // Store value as string
                t_rows.push(val_str)
            };
            // Increment num rows
            num_rows += 1;
        };
        // Print each row...
        let mut col_num = 0;
        for s in t_rows {
            // Print string
            print!("{}", s);
            // Print padding spaces
            if col_num + 1 < num_cols {
                for _ in 0..(COL_PADDING + longest_cols[col_num] - s.len()) { print!(" ") }
            }
            // Determine next column
            col_num = (col_num + 1) % num_cols;
            if col_num == 0 { println!("") }
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
                        QueryResult::Table(t) => print_table(t),
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