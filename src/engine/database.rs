pub mod engine {
    use crate::storage::table::table::*;
    use crate::sqlscript::parser::parser::*;
    use crate::sqlscript::types::types::*;
    use super::super::script::env::*;
    use super::super::script::engine::*;
    use std::rc::Rc;

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
        pub fn execute(&mut self, q: String) -> ExecutionResult {
            // Parse given query
            let mut query_parser = Parser::new(q);
            let parsed_query = query_parser.parse();
            // Execute query
            match &parsed_query {
                Query::CreateTable(table_name, schema) => self.create_table(table_name, schema),
                Query::Insert(table_name, fields, values) => self.insert(table_name, fields, values),
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
    }
}

#[cfg(test)]
mod test_database {
    use super::engine::*;
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
        db.execute("INSERT INTO test_table VALUES (field2) ('hello, world')".to_string());
        db.execute("INSERT INTO test_table VALUES (field2, field1) ('hello, world', 2)".to_string());
        Ok(())
    }
}