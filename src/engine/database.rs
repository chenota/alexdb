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
    }
}