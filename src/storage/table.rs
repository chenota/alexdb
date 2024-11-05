pub mod table {
    use super::super::column;
    use super::super::column::generic::ColumnContainer;
    use crate::sqlscript::lexer::lexer;

    pub struct Table {
        table: Vec<ColumnContainer>,
        headers: Vec<String>
    }
    impl Table {
        pub fn new() -> Table {
            Table {
                table: Vec::new(),
                headers: Vec::new()
            }
        }
        pub fn add_column(&mut self, name: String, coltype: lexer::ColType) -> () {
            // Check that column does not already exist
            if self.headers.contains(&name) { panic!("Cannot insert duplicate columns") }
            // Add table name to headers
            self.headers.push(name);
            // Push uncompressed column to table
            match coltype {
                lexer::ColType::Boolean => self.table.push(ColumnContainer::BooleanColumn(Box::new(column::generic::Uncompressed::new()))),
                lexer::ColType::Float => self.table.push(ColumnContainer::FloatColumn(Box::new(column::generic::Uncompressed::new()))),
                lexer::ColType::Integer => self.table.push(ColumnContainer::IntColumn(Box::new(column::generic::Uncompressed::new()))),
                lexer::ColType::String => self.table.push(ColumnContainer::StringColumn(Box::new(column::generic::Uncompressed::new())))
            }
        }
        pub fn get_column(&self, name: String) -> &ColumnContainer {
            // Check that column exists
            if !self.headers.contains(&name) { panic!("Invalid column name") }
            // Find column index
            let idx = self.headers.iter().position(|r| *r == name).unwrap();
            // Return column at index
            &self.table[idx]
        }
    }
}

#[cfg(test)]
mod table_tests {
    use super::*;
    use crate::sqlscript::lexer::lexer;
    #[test]
    fn test_bool_column() -> Result<(), String> {
        // Setup
        let mut test_table = table::Table::new();
        let col_name = "Test1".to_string();
        // Create new column
        test_table.add_column(col_name, lexer::ColType::Boolean);
        // Add some data
        Ok(())
    }
}