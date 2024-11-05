pub mod table {
    use super::super::column;
    use super::super::column::generic::ColumnContainer;
    use super::super::column::generic::DataContainer;
    use crate::sqlscript::lexer::lexer;

    pub struct Table {
        table: Vec<ColumnContainer>,
        headers: Vec<String>,
        size: usize
    }
    impl Table {
        pub fn new() -> Table {
            Table {
                table: Vec::new(),
                headers: Vec::new(),
                size: 0
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
        pub fn add_row(&mut self, data: Vec<column::generic::DataContainer>) {
            // Check that vector has appropriate number of items
            if data.len() != self.table.len() { panic!("Incorrect number of items") }
            // Add item to each column
            for i in 0..data.len() {
                match self.table[i] {
                    ColumnContainer::IntColumn(mut vec) => {
                        match data[i] {
                            DataContainer::Int(x) => vec.as_mut().insert(x),
                            _ => panic!("Bad data type")
                        }
                    },
                    ColumnContainer::FloatColumn(mut vec) => {
                        match data[i] {
                            DataContainer::Float(x) => vec.as_mut().insert(x),
                            _ => panic!("Bad data type")
                        }
                    },
                    ColumnContainer::BooleanColumn(mut vec) => {
                        match data[i] {
                            DataContainer::Boolean(x) => vec.as_mut().insert(x),
                            _ => panic!("Bad data type")
                        }
                    },
                    ColumnContainer::StringColumn(mut vec) => {
                        match data[i].clone() {
                            DataContainer::String(x) => vec.as_mut().insert(x),
                            _ => panic!("Bad data type")
                        }
                    }
                }
            }
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