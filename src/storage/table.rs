pub mod table {
    use super::super::column;
    use super::super::column::generic::Column;
    use crate::sqlscript::types::types::{ColType, Val};
    use crate::engine::script::env::Environment;

    enum IterCont<'a> {
        Number(Box<dyn Iterator<Item=Option<f64>> + 'a>),
        Boolean(Box<dyn Iterator<Item=Option<bool>> + 'a>),
        String(Box<dyn Iterator<Item=Option<String>> + 'a>),
    }

    pub struct Table {
        table: Vec<Column>,
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
        pub fn add_column(&mut self, name: &String, coltype: ColType) -> () {
            // Check that column does not already exist
            if self.headers.contains(&name) { panic!("Cannot insert duplicate columns") }
            // Add table name to headers
            self.headers.push(name.clone());
            // Push uncompressed column to table
            match coltype {
                ColType::Boolean => self.table.push(Column::Boolean(Box::new(column::generic::Uncompressed::new()))),
                ColType::Number => self.table.push(Column::Number(Box::new(column::generic::Uncompressed::new()))),
                ColType::String => self.table.push(Column::String(Box::new(column::generic::Uncompressed::new())))
            }
        }
        pub fn header_idx(&self, name: &String) -> usize {
            // Check that column exists
            if !self.headers.contains(name) { panic!("Invalid column name") }
            // Find column index
            self.headers.iter().position(|r| *r == *name).unwrap()
        }
        pub fn get_column(&self, name: &String) -> &Column {
            // Return column at index
            &self.table[self.header_idx(name)]
        }
        pub fn add_row(&mut self, data: Vec<Val>) {
            // Check that vector has appropriate number of items
            if data.len() != self.table.len() { panic!("Incorrect number of items") }
            // Add item to each column
            for i in 0..data.len() {
                match &mut self.table[i] {
                    Column::Number(vec) => {
                        match data[i] {
                            Val::NumVal(x) => vec.as_mut().insert(Some(x)),
                            Val::NullVal => vec.as_mut().insert(None),
                            _ => panic!("Bad data type")
                        }
                    },
                    Column::Boolean(vec) => {
                        match data[i] {
                            Val::BoolVal(x) => vec.as_mut().insert(Some(x)),
                            Val::NullVal => vec.as_mut().insert(None),
                            _ => panic!("Bad data type")
                        }
                    },
                    Column::String(vec) => {
                        match data[i].clone() {
                            Val::StrVal(x) => vec.as_mut().insert(Some(x)),
                            Val::NullVal => vec.as_mut().insert(None),
                            _ => panic!("Bad data type")
                        }
                    }
                }
            }
            // Increment size
            self.size += 1;
        }
        pub fn get_headers(&self) -> &Vec<String> { &self.headers }
        pub fn iter<'a>(&'a self) -> TableIterator<'a> {
            // Column iterators
            let mut citers = Vec::new();
            for col in &self.table {
                match col {
                    Column::Boolean(cb) => citers.push(IterCont::Boolean(Box::new(cb.as_ref().iter()))),
                    Column::Number(cb) => citers.push(IterCont::Number(Box::new(cb.as_ref().iter()))),
                    Column::String(cb) => citers.push(IterCont::String(Box::new(cb.as_ref().iter()))),
                }
            };
            // Create iterator
            TableIterator {
                table: self,
                col_iters: citers
            }
        }
        pub fn len(&self) -> usize {
            self.size
        }
        pub fn push_aggregates(&self, env: &mut Environment) -> () {
            // Placeholder
            ()
        }
    }

    pub struct TableIterator<'a> {
        table: &'a Table,
        col_iters: Vec<IterCont<'a>>
    }
    impl<'a> Iterator for TableIterator<'a> {
        type Item = Vec<Val>;
        fn next(&mut self) -> Option<Self::Item> {
            let mut retvec = Vec::new();
            for col_iter in self.col_iters.iter_mut() {
                match col_iter {
                    IterCont::Boolean(b) => match b.as_mut().next() {
                        Some(Some(x)) => retvec.push(Val::BoolVal(x)),
                        Some(None) => retvec.push(Val::NullVal),
                        None => return None
                    },
                    IterCont::Number(b) => match b.as_mut().next() {
                        Some(Some(x)) => retvec.push(Val::NumVal(x)),
                        Some(None) => retvec.push(Val::NullVal),
                        None => return None
                    },
                    IterCont::String(b) => match b.as_mut().next() {
                        Some(Some(x)) => retvec.push(Val::StrVal(x.clone())),
                        Some(None) => retvec.push(Val::NullVal),
                        None => return None
                    }
                }
            };
            Some(retvec)
        }
    }
}

#[cfg(test)]
mod table_tests {
    use super::*;
    use crate::sqlscript::types::types::{Val, ColType};
    use super::super::column::generic::Column;
    #[test]
    fn test_bool_column() -> Result<(), String> {
        // Setup
        let mut test_table = table::Table::new();
        let col_name1 = "Test1".to_string();
        let col_name2 = "Test2".to_string();
        // Create new columns
        test_table.add_column(&col_name1, ColType::Boolean);
        test_table.add_column(&col_name2, ColType::Boolean);
        // Create row
        let row1 = vec![
            Val::BoolVal(false),
            Val::NullVal
        ];
        // Add row to table
        test_table.add_row(row1);
        // Assert correct values
        match test_table.get_column(&col_name1) {
            Column::Boolean(x) => {
                match x.as_ref().extract()[0] {
                    Some(y) => assert!(!y),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        match test_table.get_column(&col_name2) {
            Column::Boolean(x) => {
                match x.as_ref().extract()[0] {
                    None => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn test_number_column() -> Result<(), String> {
        // Setup
        let mut test_table = table::Table::new();
        let col_name1 = "Test1".to_string();
        let col_name2 = "Test2".to_string();
        // Create new columns
        test_table.add_column(&col_name1, ColType::Number);
        test_table.add_column(&col_name2, ColType::Number);
        // Create row
        let row1 = vec![
            Val::NumVal(12.5),
            Val::NullVal
        ];
        // Add row to table
        test_table.add_row(row1);
        // Assert correct values
        match test_table.get_column(&col_name1) {
            Column::Number(x) => {
                match x.as_ref().extract()[0] {
                    Some(y) => assert_eq!(y, 12.5),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        match test_table.get_column(&col_name2) {
            Column::Number(x) => {
                match x.as_ref().extract()[0] {
                    None => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn test_str_column() -> Result<(), String> {
        // Setup
        let mut test_table = table::Table::new();
        let col_name1 = "Test1".to_string();
        let col_name2 = "Test2".to_string();
        // Create new columns
        test_table.add_column(&col_name1, ColType::String);
        test_table.add_column(&col_name2, ColType::String);
        // Create row
        let row1 = vec![
            Val::StrVal("Hello".to_string()),
            Val::NullVal
        ];
        // Add row to table
        test_table.add_row(row1);
        // Assert correct values
        match test_table.get_column(&col_name1) {
            Column::String(x) => {
                match &x.as_ref().extract()[0] {
                    Some(y) => assert_eq!(y, "Hello"),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        match test_table.get_column(&col_name2) {
            Column::String(x) => {
                match x.as_ref().extract()[0] {
                    None => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn test_mixed_column() -> Result<(), String> {
        // Setup
        let mut test_table = table::Table::new();
        let col_name1 = "Test1".to_string();
        let col_name2 = "Test2".to_string();
        // Create new columns
        test_table.add_column(&col_name1, ColType::Number);
        test_table.add_column(&col_name2, ColType::String);
        // Create row
        let row1 = vec![
            Val::NumVal(112.0),
            Val::StrVal("Hello".to_string())
        ];
        // Add row to table
        test_table.add_row(row1);
        // Assert correct values
        match test_table.get_column(&col_name1) {
            Column::Number(x) => {
                match &x.as_ref().extract()[0] {
                    Some(y) => assert_eq!(*y, 112.0),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        match test_table.get_column(&col_name2) {
            Column::String(x) => {
                match &x.as_ref().extract()[0] {
                    Some(y) => assert_eq!(y, "Hello"),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn test_multi_row() -> Result<(), String> {
        // Setup
        let mut test_table = table::Table::new();
        let col_name1 = "Test1".to_string();
        let col_name2 = "Test2".to_string();
        // Create new columns
        test_table.add_column(&col_name1, ColType::Number);
        test_table.add_column(&col_name2, ColType::String);
        // Create row
        let row1 = vec![
            Val::NumVal(112.2),
            Val::StrVal("Hello".to_string())
        ];
        // Create row
        let row2: Vec<Val> = vec![
            Val::NumVal(119.0),
            Val::StrVal("Goodbye".to_string())
        ];
        // Add rows to table
        test_table.add_row(row1);
        test_table.add_row(row2);
        // Assert correct values
        match test_table.get_column(&col_name1) {
            Column::Number(x) => {
                match &x.as_ref().extract()[0] {
                    Some(y) => assert_eq!(*y, 112.2),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        match test_table.get_column(&col_name2) {
            Column::String(x) => {
                match &x.as_ref().extract()[1] {
                    Some(y) => assert_eq!(y, "Goodbye"),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn test_iter() -> Result<(), String> {
        // Setup
        let mut test_table = table::Table::new();
        let col_name1 = "Test1".to_string();
        let col_name2 = "Test2".to_string();
        // Create new columns
        test_table.add_column(&col_name1, ColType::Number);
        test_table.add_column(&col_name2, ColType::String);
        // Create row
        let row1 = vec![
            Val::NumVal(112.2),
            Val::StrVal("Hello".to_string())
        ];
        // Create row
        let row2: Vec<Val> = vec![
            Val::NumVal(119.0),
            Val::StrVal("Goodbye".to_string())
        ];
        // Add rows to table
        test_table.add_row(row1);
        test_table.add_row(row2);
        // Iterate
        let mut i: usize = 0;
        for row in test_table.iter() {
            if i == 0 {
                match &row[0] {
                    Val::NumVal(112.2) => assert!(true),
                    _ => assert!(false)
                }
            } else if i == 1 {
                match &row[1] {
                    Val::StrVal(x) => assert_eq!(*x, "Goodbye".to_string()),
                    _ => assert!(false)
                }
            }
            i += 1;
        };
        assert_eq!(i, 2);
        Ok(())
    }
}