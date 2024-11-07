pub mod engine {
    use crate::storage::table::table::*;
    use crate::storage::column;
    use crate::sqlscript::parser::parser::*;
    use crate::sqlscript::lexer::lexer::*;
    use crate::sqlscript::parser::parser::parsetree::*;

    pub enum ExecutionResult {
        TableResult(Table)
    }

    pub struct Database {
        tables: Vec<Table>,
        table_names: Vec<String>
    }
    impl Database {
        pub fn execute(&mut self, q: String) -> ExecutionResult {
            // Parse given query
            let mut query_parser = Parser::new(q);
            let parsed_query = query_parser.parse();
            // Match parse tree with function
            match parsed_query {
                Query::Select(il, st, wh, lim) => ExecutionResult::TableResult(self.execute_select(il, st, wh, lim)),
                _ => panic!("Unimplemented")
            }
        }
        fn execute_select(&self, fields: IdentList, tablename: String, whereclause: Option<Expr>, limitclause: Option<Expr>) -> Table {
            
        }
        fn execute_script(&self, scr: Expr) {

        }  
    }
}