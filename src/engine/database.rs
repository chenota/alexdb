pub mod engine {
    use crate::storage::table::table::*;
    use crate::sqlscript::parser::parser::*;
    use crate::sqlscript::types::types::*;
    use super::super::script::env::*;
    use super::super::script::engine::*;
    use std::rc::Rc;
    use crate::storage::column::generic::Column;

    pub enum ExecutionResult {
        TableResult(Table),
        None
    }

    pub struct Database {
        tables: Vec<Table>,
        table_names: Vec<String>
    }
    impl Database {
        fn insert(&mut self, table_name: &String, fields: &Option<Vec<String>>, values: &Vec<Rc<Expr>>) -> ExecutionResult {
            // Get referenced table
            let table_idx = self.table_names.iter().position(|r| *r == *table_name).unwrap();
            let table = &mut self.tables[table_idx];
            // Create environment
            let mut default_env = Environment::new();
            // Evaluate given values
            let mut values_insert: Vec<Val> = Vec::new();
            let mut i: usize = 0;
            for field_name in table.get_headers() {
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
                        // Increment i
                        i += 1;
                    }
                }
            }
            // Add row to table
            table.add_row(values_insert);
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
            // Add schema to new table
            for schema_item in schema {
                self.tables[idx].add_column(&schema_item.0, schema_item.1);
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
                // Augment environment with table-specific stuff
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
                        Some(_) => full_rows.push((row.clone(), i)),
                        None => ()
                    };
                }
                // Increment i
                i += 1;
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
        pub fn execute(&mut self, q: String) -> ExecutionResult {
            // Parse given query
            let mut query_parser = Parser::new(q);
            let parsed_query = query_parser.parse();
            // Execute query
            match &parsed_query {
                Query::CreateTable(table_name, schema) => self.create_table(table_name, schema),
                Query::Insert(table_name, fields, values) => self.insert(table_name, fields, values),
                Query::Select(fields, table_name, where_, sort_by, limit) => self.select(fields, table_name, where_, sort_by, limit),
                _ => panic!("Unimplemented")
            }
        }
        pub fn new() -> Database {
            Database {
                tables: Vec::new(),
                table_names: Vec::new()
            }
        }
        pub fn get_table_names(&self) -> &Vec<String> { &self.table_names }
        pub fn default_environment(&self) -> Environment {  
            // Placeholder
            Environment::new()
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
}