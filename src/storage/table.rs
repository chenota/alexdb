pub mod table {
    use super::super::column;
    use super::super::column::generic::ColumnContainer;
    use crate::sqlscript::lexer::lexer;
    
    struct Table {
        table: Vec<ColumnContainer>,
        headers: Vec<String>
    }
    impl Table {
        fn add_column(&mut self, name: String, coltype: lexer::ColType) -> () {
            // Check that column does not already exist
            if self.headers.contains(&name) { panic!("Cannot insert duplicate columns") }
            // Add table name to headers
            self.headers.push(name);
            // Push uncompressed column to table
            match coltype {
                lexer::ColType::Boolean => self.table.push(ColumnContainer::BooleanColumn(Box::new(column::boolcolumn::Uncompressed::new()))),
                lexer::ColType::Float => self.table.push(ColumnContainer::FloatColumn(Box::new(column::floatcolumn::Uncompressed::new()))),
                lexer::ColType::Integer => self.table.push(ColumnContainer::IntColumn(Box::new(column::intcolumn::Uncompressed::new()))),
                lexer::ColType::String => self.table.push(ColumnContainer::StringColumn(Box::new(column::stringcolumn::Uncompressed::new())))
            }
        }
    }
}