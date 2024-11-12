pub mod engine {
    use crate::storage::column::generic::{Uncompressed, ColumnInterace, Column};
    use crate::storage::table::table::*;
    use crate::sqlscript::parser::parser::*;
    use crate::sqlscript::types::types::*;
    use super::super::script::env::*;
    use super::super::script::engine::*;
    use std::rc::Rc;

    pub enum ExecutionResult {
        TableResult(Table),
        ValueResult(Val),
        None
    }

    pub struct Database {
        tables: Vec<Table>,
        table_names: Vec<String>,
        constants: Vec<(String, Val)>,
        calculated: Vec<Vec<Option<Expr>>>
    }
    impl Database {
        fn insert(&mut self, table_name: &String, fields: &Option<Vec<String>>, values: &Vec<Rc<Expr>>) -> ExecutionResult {
            // Get referenced table
            let table_idx = self.table_names.iter().position(|r| *r == *table_name).unwrap();
            let table = &self.tables[table_idx];
            // Create environment
            let mut default_env = Environment::new();
            // Evaluate given values
            let mut values_insert: Vec<Val> = Vec::new();
            let mut i: usize = 0;
            for field_name in table.get_headers() {
                // Check if is calculated
                match &self.calculated[table_idx][i] {
                    Some(expr) => {
                        // Environment
                        let mut env = self.default_environment();
                        // Push already-added values onto environment
                        for j in 0..i {
                            env.push(&table.get_headers()[j], &values_insert[j]);
                        }
                        // Evaluate
                        let val = eval(expr, &mut env);
                        // Insert val
                        values_insert.push(val);
                    },
                    None => {
                        match fields {
                            Some(inserted_fields) => {
                                // Get index of field name in inserted fields
                                match inserted_fields.iter().position(|r| { r == field_name }) {
                                    Some(idx) => {
                                        // Evaluate next value
                                        let val = eval(values[idx].as_ref(), &mut default_env);
                                        // Push onto values_insert
                                        values_insert.push(val);
                                    },
                                    None => values_insert.push(Val::NullVal)
                                }
                            },
                            None => {
                                // Evaluate next value
                                let val = eval(values[i].as_ref(), &mut default_env);
                                // Push onto values_insert
                                values_insert.push(val);
                            }
                        }
                    }
                }
                // Increment i
                i += 1;
            }
            // Is this the first row?
            let first_row = table.len() == 0;
            // Calculate aggregates
            let mut ag_vals = Vec::new();
            for ag in table.get_aggregates() {
                // Are you given an initial aggregate?
                let has_init = match &ag.3 {
                    Some(_) => true,
                    _ => false,
                };
                // Environment
                let mut env = self.default_environment();
                // Add new row to environment
                for i in 0..values_insert.len() {
                    env.push(&table.get_headers()[i], &values_insert[i]);
                }
                // Add current value to environment, unless is first row and has init
                if !(first_row && has_init) {
                    env.push(&"current".to_string(), &ag.1);
                }
                // Evaluate
                let val = match first_row {
                    true => match &ag.3 {
                        Some(e1) => eval(e1, &mut env),
                        None => eval(&ag.2, &mut env)
                    },
                    false => eval(&ag.2, &mut env)
                };
                // Push
                ag_vals.push(val);
            }
            // Calculate computations
            let mut cmp_vals = Vec::new();
            for cmp in table.get_computations() {
                // Environment
                let mut env = self.default_environment();
                let mut i = 0;
                for ag in table.get_aggregates() {
                    env.push(&ag.0, &ag_vals[i]);
                    i += 1;
                }
                // Evaluate
                cmp_vals.push(eval(&cmp.2, &mut env));
            }
            // Borrow table as mutable
            let table = &mut self.tables[table_idx];
            // Add row to table
            table.add_row(values_insert);
            // Add aggregates
            table.update_aggregates(&ag_vals);
            // Add computations
            table.update_computations(&cmp_vals);
            // Return
            ExecutionResult::None
        }
        fn create_table(&mut self, table_name: &String, schema: &Vec<(String, ColType)>) -> ExecutionResult {
            // Check that table doesn't already exist
            if self.table_names.contains(table_name) { panic!("Table already exists") }
            // Push table name
            self.table_names.push(table_name.clone());
            // Create new table
            self.tables.push(Table::new());
            // Get index of new table
            let idx: usize = self.tables.len() - 1;
            // Create new calculated vector
            self.calculated.push(Vec::new());
            // Add schema to new table, mark all columns as not calculated
            for schema_item in schema {
                self.tables[idx].add_column(&schema_item.0, schema_item.1);
                self.calculated[idx].push(None)
            }
            ExecutionResult::None
        }
        fn select(&mut self, fields: &Option<Vec<String>>, table_name: &String, where_: &Option<Expr>, sort_by: &Option<(String, SortType)>, limit: &Option<Expr>) -> ExecutionResult {
            // Get referenced table
            let table_idx = self.table_names.iter().position(|r| *r == *table_name).unwrap();
            let table = &self.tables[table_idx];
            // Vector of added rows
            let mut added_rows: Vec<Vec<Val>> = Vec::new();
            let mut full_rows: Vec<(Vec<Val>, usize)> = Vec::new();
            // Iterate through each row in the table
            let mut i: usize = 0;
            for row in table.iter() {
                // New row
                let mut new_row: Vec<Val> = Vec::new();
                // Environment in which to evaluate row
                let mut env = self.default_environment();
                // Add aggregates into environment
                table.push_aggregates(&mut env);
                // Add all fields to environment
                for field in table.get_headers() {
                    let idx = table.header_idx(field);
                    env.push(field, &row[idx]);
                }
                // Add items to new row
                match fields {
                    Some(v) => {
                        for field in v {
                            let idx = table.header_idx(field);
                            new_row.push(row[idx].clone());
                        }
                    },
                    None => {
                        for field in table.get_headers() {
                            let idx = table.header_idx(field);
                            new_row.push(row[idx].clone());
                        }
                    }
                }
                // Evaluate where clause, convert to bool
                let should_add = match where_ {
                    Some(expr) => eval_bool(expr, &mut env),
                    None => true
                };
                // Push to new table if should add
                if should_add {
                    added_rows.push(new_row);
                    // Only add to full rows if sorting
                    match sort_by {
                        Some(_) => {
                            full_rows.push((row.clone(), i));
                            // Increment i
                            i += 1;
                        },
                        None => ()
                    };
                }
                
            };
            // If sorting, sort full rows by sort_by header
            match sort_by {
                Some(s) => {
                    let col_idx = table.header_idx(&s.0);
                    match &s.1 {
                        SortType::Ascending => full_rows.sort_by(|a, b| eval_ordering(&(a.0)[col_idx], &(b.0)[col_idx])),
                        SortType::Descending => full_rows.sort_by(|a, b| eval_ordering_desc(&(a.0)[col_idx], &(b.0)[col_idx]))
                    }
                },
                None => ()
            };
            // Create empty projected table
            let mut table_project = Table::new();
            // Setup table
            match fields {
                Some(v) => {
                    for field in v {
                        match table.get_column(field) {
                            Column::Boolean(_) => table_project.add_column(field, ColType::Boolean),
                            Column::Number(_) => table_project.add_column(field, ColType::Number),
                            Column::String(_) => table_project.add_column(field, ColType::String)
                        }
                    }
                },
                None => {
                    for field in table.get_headers() {
                        match table.get_column(field) {
                            Column::Boolean(_) => table_project.add_column(field, ColType::Boolean),
                            Column::Number(_) => table_project.add_column(field, ColType::Number),
                            Column::String(_) => table_project.add_column(field, ColType::String)
                        }
                    }
                }
            };
            // Get limit of rows to add to table
            let lim_usize = match limit {
                Some(expr) => {
                    // Evaluate expression
                    let mut env = self.default_environment();
                    // Add aggregates into environment
                    table.push_aggregates(&mut env);
                    Some(eval_num(expr, &mut env) as usize)
                },
                None => None
            };
            match sort_by {
                Some(_) => {
                    // Add rows to new table
                    for i in 0..full_rows.len() {
                        // Stop adding if reached limit
                        match lim_usize {
                            Some(x) => if i >= x {break},
                            None => ()
                        };
                        // Matching index of added row
                        let matching_index = full_rows[i].1;
                        // Add row to table
                        table_project.add_row(added_rows[matching_index].clone())
                    };
                },
                None => {
                    // Reverse added rows (need to do this since popping)
                    added_rows.reverse();
                    // Add rows to new table
                    for i in 0..added_rows.len() {
                        // Stop adding if reached limit
                        match lim_usize {
                            Some(x) => if i >= x {break},
                            None => ()
                        };
                        // Add row to table
                        table_project.add_row(added_rows.pop().unwrap())
                    };
                }
            }
            // Return new table
            ExecutionResult::TableResult(table_project)
        }
        fn create_const(&mut self, name: &String, expr: &Expr) -> ExecutionResult {
            // Evaluate expr
            let mut env = self.default_environment();
            let val = eval(expr, &mut env);
            // Check if name already in constants
            let pos = self.constants.iter().position(|r| r.0 == *name);
            match pos {
                Some(idx) => self.constants[idx] = (name.clone(), val),
                None => self.constants.push((name.clone(), val))
            };
            // Return nothing
            ExecutionResult::None
        }
        fn create_column(&mut self, t: &ColType, col_name: &String, expr: &Expr, table_name: &String) -> ExecutionResult {
            // Get index of table
            let table_idx = self.get_table_index(table_name).unwrap();
            let table = &self.tables[table_idx];
            // Calculate values for existing rows
            let col = match t {
                ColType::Boolean => {
                    // Data object
                    let mut col_data: Uncompressed<bool> = Uncompressed::new();
                    // Iterate through table rows
                    for row in table.iter() {
                        // Environment
                        let mut env = self.default_environment();
                        // Push row to environment
                        for field in table.get_headers() {
                            let idx = table.header_idx(field);
                            env.push(field, &row[idx]);
                        }
                        // Get column value
                        let val = eval_bool_option(expr, &mut env);
                        // Push value to data container
                        col_data.insert(val);
                    }
                    // Return column
                    Column::Boolean(Box::new(col_data))
                },
                ColType::Number => {
                    // Data object
                    let mut col_data: Uncompressed<f64> = Uncompressed::new();
                    // Iterate through table rows
                    for row in table.iter() {
                        // Environment
                        let mut env = self.default_environment();
                        // Push row to environment
                        for field in table.get_headers() {
                            let idx = table.header_idx(field);
                            env.push(field, &row[idx]);
                        }
                        // Get column value
                        let val = eval_num_option(expr, &mut env);
                        // Push value to data container
                        col_data.insert(val);
                    }
                    // Return column
                    Column::Number(Box::new(col_data))
                },
                ColType::String => {
                    // Data object
                    let mut col_data: Uncompressed<String> = Uncompressed::new();
                    // Iterate through table rows
                    for row in table.iter() {
                        // Environment
                        let mut env = self.default_environment();
                        // Push row to environment
                        for field in table.get_headers() {
                            let idx = table.header_idx(field);
                            env.push(field, &row[idx]);
                        }
                        // Get column value
                        let val = eval_str_option(expr, &mut env);
                        // Push value to data container
                        col_data.insert(val);
                    }
                    // Return column
                    Column::String(Box::new(col_data))
                }
            };
            // Borrow table as mutable
            let table = &mut self.tables[table_idx];
            // Insert column into table
            table.add_populated_column(col_name, col);
            // Mark column as calculated
            self.calculated[table_idx].push(Some(expr.clone()));
            // Return nothing
            ExecutionResult::None
        }
        fn create_aggregate(&mut self, ag_name: &String, expr: &Expr, init: &Option<Expr>, table_name: &String) -> ExecutionResult {
            // Get index of table
            let table_idx = self.get_table_index(table_name).unwrap();
            let table = &self.tables[table_idx];
            // Value of aggregate
            let mut ag_val = Val::NullVal;
            // Does this have an init?
            let has_init = match init {
                Some(_) => true,
                None => false
            };
            // Calculate aggregate for existing rows
            let mut i: usize = 0;
            for row in table.iter() {
                // Is this the first row?
                let first_row = i == 0;
                // Environment
                let mut env = self.default_environment();
                // Add row to environment
                for i in 0..row.len() {
                    env.push(&table.get_headers()[i], &row[i]);
                }
                // Add current value to environment, unless has init and is first row
                if !(has_init && first_row) {
                    env.push(&"current".to_string(), &ag_val);
                }
                // Evaluate
                ag_val = match first_row {
                    true => match init {
                        Some(e1) => eval(e1, &mut env),
                        None => eval(expr, &mut env)
                    },
                    false => eval(expr, &mut env)
                };
                // Increment i
                i += 1;
            };
            // Register aggregate into table
            let table = &mut self.tables[table_idx];
            table.add_aggregate(ag_name, &ag_val, expr, init);
            // Finished
            ExecutionResult::None
        }
        fn select_aggregate(&mut self, ag_name: &String, table_name: &String) -> ExecutionResult {
            // Get index of table
            let table_idx = self.get_table_index(table_name).unwrap();
            let table = &self.tables[table_idx];
            // Return aggregate
            ExecutionResult::ValueResult(table.get_aggregate(ag_name))
        }
        fn create_computation(&mut self, cmp_name: &String, expr: &Expr, table_name: &String) -> ExecutionResult {
            // Get index of table
            let table_idx = self.get_table_index(table_name).unwrap();
            let table = &self.tables[table_idx]; 
            // Get value of computation (null if table is empty)
            let comp_val = match table.len() == 0 {
                true => Val::NullVal,
                false => {
                    // Environment
                    let mut env = self.default_environment();
                    table.push_aggregates(&mut env);
                    // Evaluate
                    eval(expr, &mut env)
                }
            };
            // Register computation into table
            let table = &mut self.tables[table_idx];
            table.add_computation(cmp_name, &comp_val, expr);
            // Return nothing
            ExecutionResult::None
        }
        pub fn execute(&mut self, q: String) -> ExecutionResult {
            // Parse given query
            let mut query_parser = Parser::new(q);
            let parsed_query = query_parser.parse();
            // Execute query
            match &parsed_query {
                Query::CreateTable(table_name, schema) => self.create_table(table_name, schema),
                Query::Insert(table_name, fields, values) => self.insert(table_name, fields, values),
                Query::Select(fields, table_name, where_, sort_by, limit) => self.select(fields, table_name, where_, sort_by, limit),
                Query::Const(name, expr) => self.create_const(name, expr),
                Query::Column(t, col_name, expr, table_name) => self.create_column(t, col_name, expr, table_name),
                Query::Aggregate(ag_name, expr, init, table_name) => self.create_aggregate(ag_name, expr, init, table_name),
                Query::SelectAggregate(ag_name, table_name) => self.select_aggregate(ag_name, table_name),
                Query::Comp(cmp_name, expr, table_name) => self.create_computation(cmp_name, expr, table_name),
                _ => panic!("Unimplemented")
            }
        }
        pub fn new() -> Database {
            Database {
                tables: Vec::new(),
                table_names: Vec::new(),
                constants: Vec::new(),
                calculated: Vec::new()
            }
        }
        pub fn get_table_names(&self) -> &Vec<String> { &self.table_names }
        pub fn get_table_index(&self, name: &String) -> Option<usize> { self.table_names.iter().position(|r| *r == *name) }
        pub fn default_environment(&self) -> Environment {  
            // New environment
            let mut def_env = Environment::new();
            // Add constants
            for c in self.constants.iter() {
                def_env.push(&c.0, &c.1)
            };
            // Return
            def_env
        }
    }
}

#[cfg(test)]
mod test_database {
    use super::engine::*;
    use crate::sqlscript::types::types::Val;
    #[test]
    fn create_table_single() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (col1 num)".to_string());
        // Get table names
        let table_names = db.get_table_names();
        // Check values
        assert_eq!(table_names[0], "test_table");
        Ok(())
    }
    #[test]
    fn create_table_multi() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (col1 num)".to_string());
        db.execute("CREATE TABLE test_table2 (col1 num)".to_string());
        // Get table names
        let table_names = db.get_table_names();
        // Check values
        assert_eq!(table_names[0], "test_table");
        assert_eq!(table_names[1], "test_table2");
        Ok(())
    }
    #[test]
    fn insert_single() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1)".to_string());
        db.execute("INSERT INTO test_table VALUES (2)".to_string());
        Ok(())
    }
    #[test]
    fn insert_multi() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 str)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, 'testval')".to_string());
        db.execute("INSERT INTO test_table VALUES (2, 'hello, world')".to_string());
        Ok(())
    }
    #[test]
    fn insert_specific() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 str)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table (field1) VALUES (1)".to_string());
        db.execute("INSERT INTO test_table (field2) VALUES ('hello, world')".to_string());
        db.execute("INSERT INTO test_table (field2, field1) VALUES ('hello, world', 2)".to_string());
        Ok(())
    }
    #[test]
    fn select_basic_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 num)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, 2)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, 4)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, 6)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 3);
                let mut i = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(1.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_mix_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 num)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, 2)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, 4)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, 6)".to_string());
        // Perform select query
        let result = db.execute("SELECT field2, field1 FROM test_table".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 3);
                let mut i = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[1] {
                            Val::NumVal(1.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_mix_types() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 3);
                let mut i = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[2] {
                            Val::BoolVal(false) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_where_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table WHERE field2 == true".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 2);
                let mut i = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[2] {
                            Val::BoolVal(false) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    if i == 1 {
                        match row[2] {
                            Val::BoolVal(true) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_where_2() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table WHERE field1 > 4".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 1);
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_where_3() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table WHERE field1 > 1000".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 0);
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_where_4() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table WHERE field1 == 3 && field2 == false && field3 == false".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 1);
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_where_5() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table WHERE {isbig = fun x -> x >= 5; isbig(field1)}".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 1);
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_limit_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table LIMIT 1".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(1.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_limit_2() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table LIMIT 2 - 1".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(1.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_limit_3() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table LIMIT '1'".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(1.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_limit_4() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table LIMIT (fun -> 1)()".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(1.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_limit_5() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table LIMIT 0".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 0);
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_limit_where_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table WHERE field1 > 2 LIMIT (fun -> 1)()".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(3.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_order_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table ORDER BY field1".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 3);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(1.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    if i == 1 {
                        match row[0] {
                            Val::NumVal(3.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    if i == 2 {
                        match row[0] {
                            Val::NumVal(5.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_order_asc() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table ORDER BY field1 ASC".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 3);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(1.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    if i == 1 {
                        match row[0] {
                            Val::NumVal(3.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    if i == 2 {
                        match row[0] {
                            Val::NumVal(5.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_order_desc() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table ORDER BY field1 DESC".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 3);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(5.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    if i == 1 {
                        match row[0] {
                            Val::NumVal(3.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    if i == 2 {
                        match row[0] {
                            Val::NumVal(1.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_where_order() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table WHERE field1 == 3 || field1 == 5 ORDER BY field1 DESC".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 2);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(5.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    if i == 1 {
                        match row[0] {
                            Val::NumVal(3.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_where_order_limit() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table WHERE field1 == 3 || field1 == 5 ORDER BY field1 DESC LIMIT 1".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(5.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn const_val() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Add constants
        db.execute("CREATE CONST test_val = 5".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table WHERE field1 == test_val".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(5.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn const_fun() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Add constants
        db.execute("CREATE CONST max = fun x, y -> if x > y then x else y".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table WHERE field1 == max(1, 5)".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(5.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn calculated_column_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Add constants
        // Add calculated columns
        db.execute("CREATE COLUMN (bool) calc_1 = false INTO test_table".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table LIMIT 1".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[3] {
                            Val::BoolVal(false) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn calculated_column_2() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Add constants
        // Add calculated columns
        db.execute("CREATE COLUMN (bool) calc_1 = field2 INTO test_table".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table LIMIT 1".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[3] {
                            Val::BoolVal(true) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn calculated_column_3() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Add constants
        // Add calculated columns
        db.execute("CREATE COLUMN (bool) calc_1 = field1 > 10 INTO test_table".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table LIMIT 1".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[3] {
                            Val::BoolVal(false) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn calculated_column_before_insert() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Add calculated columns
        db.execute(" CREATE COLUMN (bool) calc_1 = field1 > 10 INTO test_table".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Add constants
        // Perform select query
        let result = db.execute("SELECT * FROM test_table LIMIT 1".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[3] {
                            Val::BoolVal(false) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn calculated_column_insert_before_after() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Add constants
        // Add calculated columns
        db.execute("CREATE COLUMN (bool) calc_1 = field1 === 7 INTO test_table".to_string());
        // Insert more data
        db.execute("INSERT INTO test_table VALUES (7, false, false)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table ORDER BY field1 DESC LIMIT 1".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[3] {
                            Val::BoolVal(true) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn calculated_column_multi_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Add constants
        // Add calculated columns
        db.execute("CREATE COLUMN (bool) calc_1 = field1 == 5 INTO test_table".to_string());
        db.execute("CREATE COLUMN (bool) calc_2 = !calc_1 INTO test_table".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table LIMIT 1".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[4] {
                            Val::BoolVal(false) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn calculated_column_multi_2() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Add constants
        // Add calculated columns
        db.execute("CREATE COLUMN (num) calc_1 = if field2 then field1 * 2 else field1 * 3 INTO test_table".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table LIMIT 1".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[3] {
                            Val::NumVal(10.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn aggregate_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Add aggregate
        db.execute("CREATE AGGREGATE max_field1 = current INIT field1 INTO test_table".to_string());
        // Perform select aggregate query
        let result = db.execute("SELECT AGGREGATE max_field1 FROM test_table".to_string());
        match result {
            ExecutionResult::ValueResult(v) => {
                match v {
                    Val::NumVal(5.0) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn aggregate_2() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Add aggregate
        db.execute("CREATE AGGREGATE max_field1 = if field1 > current then field1 else current INIT field1 INTO test_table".to_string());
        // Perform select aggregate query
        let result = db.execute("SELECT AGGREGATE max_field1 FROM test_table".to_string());
        match result {
            ExecutionResult::ValueResult(v) => {
                match v {
                    Val::NumVal(5.0) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn aggregate_multi_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 num)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, 6)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, 11)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, 3)".to_string());
        // Add aggregate
        db.execute("CREATE AGGREGATE max_sum = {sum = field1 + field2; if sum > current then sum else current} INIT field1 + field2 INTO test_table".to_string());
        // Perform select aggregate query
        let result = db.execute("SELECT AGGREGATE max_sum FROM test_table".to_string());
        match result {
            ExecutionResult::ValueResult(v) => {
                match v {
                    Val::NumVal(12.0) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn aggregate_backwards_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 num)".to_string());
        // Add aggregate
        db.execute("CREATE AGGREGATE max_sum = {sum = field1 + field2; if sum > current then sum else current} INIT field1 + field2 INTO test_table".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, 6)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, 11)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, 3)".to_string());
        // Perform select aggregate query
        let result = db.execute("SELECT AGGREGATE max_sum FROM test_table".to_string());
        match result {
            ExecutionResult::ValueResult(v) => {
                match v {
                    Val::NumVal(12.0) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn aggregate_backwards_2() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Add aggregate
        db.execute("CREATE AGGREGATE max_field1 = if field1 > current then field1 else current INIT field1 INTO test_table".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Perform select aggregate query
        let result = db.execute("SELECT AGGREGATE max_field1 FROM test_table".to_string());
        match result {
            ExecutionResult::ValueResult(v) => {
                match v {
                    Val::NumVal(5.0) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn aggregate_no_init() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Add aggregate
        db.execute("CREATE AGGREGATE sum_field1 = if current === null then field1 else current + field1 INTO test_table".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Perform select aggregate query
        let result = db.execute("SELECT AGGREGATE sum_field1 FROM test_table".to_string());
        match result {
            ExecutionResult::ValueResult(v) => {
                match v {
                    Val::NumVal(9.0) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_with_aggregate() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Aggregate
        db.execute("CREATE AGGREGATE max_field1 = if field1 > current then field1 else current INIT field1 INTO test_table".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Perform select query using aggregate
        let result = db.execute("SELECT * FROM test_table WHERE field1 < max_field1".to_string());
        match result {
            ExecutionResult::TableResult(t) => {
                assert_eq!(t.len(), 2);
            },
            _ => assert!(false)
        }
        Ok(())
    }
}