pub mod engine {
    use crate::storage::table::table::*;
    use crate::sqlscript::parser::parser::*;
    use crate::sqlscript::types::types::*;

    pub enum ExecutionResult {
        TableResult(Table)
    }

    pub struct Database {
        tables: Vec<Table>,
        table_names: Vec<String>
    }
    impl Database {
        pub fn execute(&mut self, q: String) {
            // Parse given query
            let mut query_parser = Parser::new(q);
            let parsed_query = query_parser.parse();
        }
    }
}