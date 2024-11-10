pub mod table {
    use super::super::column;
    use super::super::column::generic::Column;
    use crate::sqlscript::types::types::{ColType, Val};

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
        pub fn get_column(&self, name: &String) -> &Column {
            // Check that column exists
            if !self.headers.contains(name) { panic!("Invalid column name") }
            // Find column index
            let idx = self.headers.iter().position(|r| *r == *name).unwrap();
            // Return column at index
            &self.table[idx]
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
        pub fn select_project(&self, headers: &Vec<String>) -> Table {
            // Return value
            let mut new_table = Table::new();
            // Add columns to new table
            for header in headers {
                // Push new column
                match self.get_column(header) {
                    Column::Boolean(_) => new_table.add_column(header, ColType::Boolean),
                    Column::Number(_) => new_table.add_column(header, ColType::Number),
                    Column::String(_) => new_table.add_column(header, ColType::String)
                }
            };
            // Iterators for each column
            let mut iters = Vec::new();
            for column in &self.table {
                match column {
                    Column::Boolean(x) => iters.push(IterCont::Boolean(x.as_ref().iter())),
                    Column::Number(x) => iters.push(IterCont::Number(x.as_ref().iter())),
                    Column::String(x) => iters.push(IterCont::String(x.as_ref().iter()))
                }
            };
            // Iterate through each row
            for _ in 0..self.size {
                // Construct whole row
                let mut whole_row: Vec<Val> = Vec::new();
                for iter in &mut iters {
                    whole_row.push(match iter {
                        IterCont::Boolean(itbox) => match itbox.as_mut().next().unwrap() {
                            Some(x) => Val::BoolVal(x),
                            _ => Val::NullVal
                        },
                        IterCont::Number(itbox) => match itbox.as_mut().next().unwrap() {
                            Some(x) => Val::NumVal(x),
                            _ => Val::NullVal
                        },
                        IterCont::String(itbox) => match itbox.as_mut().next().unwrap() {
                            Some(x) => Val::StrVal(x.clone()),
                            _ => Val::NullVal
                        },
                    })
                };
                // Construct parial (projected) row
                let mut partial_row: Vec<Val> = Vec::new();
                for header in headers {
                    let idx = self.headers.iter().position(|r| *r == *header).unwrap();
                    partial_row.push(whole_row[idx].clone());
                }
                // TODO: Check that whole row passes test (selection)
                // Add partial row to new table
                new_table.add_row(partial_row);
            };
            new_table
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
    fn test_project_basic_1() -> Result<(), String> {
        // Setup
        let mut test_table = table::Table::new();
        let col_name1 = "col_1".to_string();
        let col_name2 = "col_2".to_string();
        // Create new columns
        test_table.add_column(&col_name1, ColType::Number);
        test_table.add_column(&col_name2, ColType::Number);
        // Create row
        let row1 = vec![
            Val::NumVal(112.2),
            Val::NumVal(115.7),
        ];
        // Create row
        let row2: Vec<Val> = vec![
            Val::NumVal(119.0),
            Val::NumVal(110.0),
        ];
        // Add rows to table
        test_table.add_row(row1);
        test_table.add_row(row2);
        // Project table
        let headers = vec!["col_1".to_string()];
        let result = test_table.select_project(&headers);
        // Check table
        match result.get_column(&col_name1) {
            Column::Number(x) => {
                match &x.as_ref().extract()[0] {
                    Some(y) => assert_eq!(*y, 112.2),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn test_project_basic_2() -> Result<(), String> {
        // Setup
        let mut test_table = table::Table::new();
        let col_name1 = "col_1".to_string();
        let col_name2 = "col_2".to_string();
        // Create new columns
        test_table.add_column(&col_name1, ColType::Number);
        test_table.add_column(&col_name2, ColType::Number);
        // Create row
        let row1 = vec![
            Val::NumVal(112.2),
            Val::NumVal(115.7),
        ];
        // Create row
        let row2: Vec<Val> = vec![
            Val::NumVal(119.0),
            Val::NumVal(110.0),
        ];
        // Add rows to table
        test_table.add_row(row1);
        test_table.add_row(row2);
        // Project table
        let headers = vec!["col_2".to_string()];
        let result = test_table.select_project(&headers);
        // Check table
        match result.get_column(&col_name2) {
            Column::Number(x) => {
                match &x.as_ref().extract()[1] {
                    Some(y) => assert_eq!(*y, 110.0),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn test_project_basic_3() -> Result<(), String> {
        // Setup
        let mut test_table = table::Table::new();
        let col_name1 = "col_1".to_string();
        let col_name2 = "col_2".to_string();
        // Create new columns
        test_table.add_column(&col_name1, ColType::Number);
        test_table.add_column(&col_name2, ColType::Number);
        // Create row
        let row1 = vec![
            Val::NumVal(112.2),
            Val::NumVal(115.7),
        ];
        // Create row
        let row2: Vec<Val> = vec![
            Val::NumVal(119.0),
            Val::NumVal(110.0),
        ];
        // Add rows to table
        test_table.add_row(row1);
        test_table.add_row(row2);
        // Project table
        let headers = vec!["col_2".to_string(), "col_1".to_string()];
        let result = test_table.select_project(&headers);
        // Check table
        assert_eq!(result.get_headers()[0], col_name2);
        Ok(())
    }
}