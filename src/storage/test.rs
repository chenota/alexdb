#[cfg(test)]
mod test_column {
    use super::super::column::generic::*;

    #[test]
    fn run_length_1() -> Result<(), String> {
        // New run length column
        let mut col: RunLength<f64> = RunLength::new();
        // Insert some new values
        col.insert(Some(5.0));
        col.insert(Some(5.0));
        col.insert(Some(5.0));
        col.insert(Some(4.0));
        col.insert(Some(4.0));
        // Uncompress col
        let col_unc = col.uncompress();
        // Check values
        assert_eq!(col_unc.len(), 5);
        assert_eq!(col_unc[0].unwrap(), 5.0);
        assert_eq!(col_unc[2].unwrap(), 5.0);
        assert_eq!(col_unc[4].unwrap(), 4.0);
        Ok(())
    }
    #[test]
    fn run_length_2() -> Result<(), String> {
        // New run length column
        let mut col: RunLength<f64> = RunLength::new();
        // Insert some new values
        col.insert(Some(5.0));
        col.insert(Some(5.0));
        col.insert(Some(5.0));
        col.insert(None);
        col.insert(None);
        // Uncompress col
        let col_unc = col.uncompress();
        // Check values
        assert_eq!(col_unc.len(), 5);
        assert_eq!(col_unc[0].unwrap(), 5.0);
        assert_eq!(col_unc[2].unwrap(), 5.0);
        match &col_unc[4] {
            None => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn bitmap_1() -> Result<(), String> {
        // New run length column
        let mut col: BitMap<f64> = BitMap::new();
        // Insert some new values
        col.insert(Some(5.0));
        col.insert(Some(5.0));
        col.insert(Some(5.0));
        col.insert(Some(4.0));
        col.insert(Some(4.0));
        // Uncompress col
        let col_unc = col.uncompress();
        // Check values
        assert_eq!(col_unc.len(), 5);
        assert_eq!(col_unc[0].unwrap(), 5.0);
        assert_eq!(col_unc[2].unwrap(), 5.0);
        assert_eq!(col_unc[4].unwrap(), 4.0);
        Ok(())
    }
    #[test]
    fn bitmap_2() -> Result<(), String> {
        // New run length column
        let mut col: BitMap<f64> = BitMap::new();
        // Insert some new values
        col.insert(Some(5.0));
        col.insert(Some(5.0));
        col.insert(Some(5.0));
        col.insert(None);
        col.insert(None);
        // Uncompress col
        let col_unc = col.uncompress();
        // Check values
        assert_eq!(col_unc.len(), 5);
        assert_eq!(col_unc[0].unwrap(), 5.0);
        assert_eq!(col_unc[2].unwrap(), 5.0);
        match &col_unc[4] {
            None => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn xor_1() -> Result<(), String> {
        // New run length column
        let mut col: XorCol = XorCol::new();
        // Insert some new values
        col.insert(None);
        // Uncompress col
        let col_unc = col.uncompress();
        // Check values
        assert_eq!(col_unc.len(), 1);
        match &col_unc[0] {
            None => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn xor_2() -> Result<(), String> {
        // New run length column
        let mut col: XorCol = XorCol::new();
        // Insert some new values
        col.insert(None);
        col.insert(None);
        col.insert(None);
        col.insert(None);
        col.insert(None);
        // Uncompress col
        let col_unc = col.uncompress();
        // Check values
        assert_eq!(col_unc.len(), 5);
        match &col_unc[4] {
            None => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn xor_3() -> Result<(), String> {
        // New run length column
        let mut col: XorCol = XorCol::new();
        // Insert some new values
        col.insert(Some(5.0));
        // Uncompress col
        let col_unc = col.uncompress();
        // Check values
        assert_eq!(col_unc.len(), 1);
        assert_eq!(col_unc[0].unwrap(), 5.0);
        Ok(())
    }
    #[test]
    fn xor_4() -> Result<(), String> {
        // New run length column
        let mut col: XorCol = XorCol::new();
        // Insert some new values
        col.insert(None);
        col.insert(Some(5.0));
        // Uncompress col
        let col_unc = col.uncompress();
        // Check values
        assert_eq!(col_unc.len(), 2);
        assert_eq!(col_unc[1].unwrap(), 5.0);
        Ok(())
    }
    #[test]
    fn xor_5() -> Result<(), String> {
        // New run length column
        let mut col: XorCol = XorCol::new();
        // Insert some new values
        col.insert(Some(5.0));
        col.insert(Some(5.0));
        col.insert(Some(5.0));
        // Uncompress col
        let col_unc = col.uncompress();
        // Check values
        assert_eq!(col_unc.len(), 3);
        assert_eq!(col_unc[0].unwrap(), 5.0);
        assert_eq!(col_unc[2].unwrap(), 5.0);
        Ok(())
    }
    #[test]
    fn xor_6() -> Result<(), String> {
        // New run length column
        let mut col: XorCol = XorCol::new();
        // Insert some new values
        col.insert(Some(4.0));
        col.insert(Some(5.0));
        col.insert(Some(5.0));
        // Uncompress col
        let col_unc = col.uncompress();
        // Check values
        assert_eq!(col_unc.len(), 3);
        assert_eq!(col_unc[0].unwrap(), 4.0);
        assert_eq!(col_unc[2].unwrap(), 5.0);
        Ok(())
    }
    #[test]
    fn xor_7() -> Result<(), String> {
        // New run length column
        let mut col: XorCol = XorCol::new();
        // Insert some new values
        col.insert(Some(0.0));
        col.insert(Some(5.0));
        col.insert(Some(3.0));
        // Uncompress col
        let col_unc = col.uncompress();
        // Check values
        assert_eq!(col_unc.len(), 3);
        assert_eq!(col_unc[0].unwrap(), 0.0);
        assert_eq!(col_unc[2].unwrap(), 3.0);
        Ok(())
    }
    #[test]
    fn xor_8() -> Result<(), String> {
        // New run length column
        let mut col: XorCol = XorCol::new();
        // Insert some new values
        col.insert(Some(0.0));
        col.insert(Some(14.0));
        col.insert(Some(3.0));
        col.insert(Some(-1110.2292));
        col.insert(Some(3.0));
        col.insert(Some(-3.0));
        col.insert(Some(5.0));
        col.insert(Some(5.0));
        col.insert(Some(45.654));
        col.insert(Some(5.0));
        col.insert(Some(13.6765));
        col.insert(Some(100000.7));
        col.insert(Some(0.0));
        col.insert(Some(45.0));
        col.insert(Some(0.0000005));
        // Uncompress col5
        let col_unc = col.uncompress();
        // Check values
        assert_eq!(col_unc.len(), 15);
        assert_eq!(col_unc[1].unwrap(), 14.0);
        assert_eq!(col_unc[5].unwrap(), -3.0);
        assert_eq!(col_unc[9].unwrap(), 5.0);
        assert_eq!(col_unc[13].unwrap(), 45.0);
        Ok(())
    }
    #[test]
    fn xor_9() -> Result<(), String> {
        // New run length column
        let mut col: XorCol = XorCol::new();
        // Insert some new values
        col.insert(Some(0.0));
        col.insert(Some(14.0));
        col.insert(Some(3.0));
        col.insert(None);
        col.insert(Some(3.0));
        col.insert(Some(-3.0));
        col.insert(Some(5.0));
        col.insert(None);
        col.insert(Some(45.654));
        col.insert(Some(5.0));
        col.insert(Some(13.6765));
        col.insert(Some(100000.7));
        col.insert(Some(0.0));
        col.insert(Some(45.0));
        col.insert(None);
        // Uncompress col5
        let col_unc = col.uncompress();
        // Check values
        assert_eq!(col_unc.len(), 15);
        assert_eq!(col_unc[1].unwrap(), 14.0);
        assert_eq!(col_unc[5].unwrap(), -3.0);
        assert_eq!(col_unc[9].unwrap(), 5.0);
        assert_eq!(col_unc[13].unwrap(), 45.0);
        Ok(())
    }
    #[test]
    fn xor_10() -> Result<(), String> {
        // New run length column
        let mut col: XorCol = XorCol::new();
        // Insert some new values
        col.insert(Some(5.0));
        col.insert(Some(6.0));
        col.insert(Some(8.0));
        col.insert(Some(13.0));
        col.insert(Some(2.0));
        col.insert(Some(5.0));
        // Uncompress col5
        let col_unc = col.uncompress();
        // Check values
        assert_eq!(col_unc.len(), 6);
        assert_eq!(col_unc[1].unwrap(), 6.0);
        assert_eq!(col_unc[2].unwrap(), 8.0);
        assert_eq!(col_unc[4].unwrap(), 2.0);
        assert_eq!(col_unc[5].unwrap(), 5.0);
        Ok(())
    }
    #[test]
    fn bool_1() -> Result<(), String> {
        // New run length column
        let mut col: BoolCol = BoolCol::new();
        // Insert some new values
        col.insert(Some(true));
        // Uncompress col5
        let col_unc = col.uncompress();
        // Check values
        assert_eq!(col_unc.len(), 1);
        assert_eq!(col_unc[0].unwrap(), true);
        Ok(())
    }
    #[test]
    fn bool_2() -> Result<(), String> {
        // New run length column
        let mut col: BoolCol = BoolCol::new();
        // Insert some new values
        col.insert(Some(true));
        col.insert(Some(false));
        // Uncompress col5
        let col_unc = col.uncompress();
        // Check values
        assert_eq!(col_unc.len(), 2);
        assert_eq!(col_unc[0].unwrap(), true);
        assert_eq!(col_unc[1].unwrap(), false);
        Ok(())
    }
    #[test]
    fn bool_3() -> Result<(), String> {
        // New run length column
        let mut col: BoolCol = BoolCol::new();
        // Insert some new values
        col.insert(Some(true));
        col.insert(None);
        col.insert(Some(false));
        // Uncompress col5
        let col_unc = col.uncompress();
        // Check values
        assert_eq!(col_unc.len(), 3);
        assert_eq!(col_unc[0].unwrap(), true);
        assert!(col_unc[1].is_none());
        assert_eq!(col_unc[2].unwrap(), false);
        Ok(())
    }
}

#[cfg(test)]
mod table_tests {
    use super::super::table::*;
    use crate::sqlscript::types::types::{Val, ColType, CompressType};
    use super::super::column::generic::Column;
    #[test]
    fn test_bool_column() -> Result<(), String> {
        // Setup
        let mut test_table = table::Table::new();
        let col_name1 = "Test1".to_string();
        let col_name2 = "Test2".to_string();
        // Create new columns
        test_table.add_column(&col_name1, ColType::Boolean, CompressType::Uncompressed);
        test_table.add_column(&col_name2, ColType::Boolean, CompressType::Uncompressed);
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
                match x.as_ref().uncompress()[0] {
                    Some(y) => assert!(!y),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        match test_table.get_column(&col_name2) {
            Column::Boolean(x) => {
                match x.as_ref().uncompress()[0] {
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
        test_table.add_column(&col_name1, ColType::Number, CompressType::Uncompressed);
        test_table.add_column(&col_name2, ColType::Number, CompressType::Uncompressed);
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
                match x.as_ref().uncompress()[0] {
                    Some(y) => assert_eq!(y, 12.5),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        match test_table.get_column(&col_name2) {
            Column::Number(x) => {
                match x.as_ref().uncompress()[0] {
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
        test_table.add_column(&col_name1, ColType::String, CompressType::Uncompressed);
        test_table.add_column(&col_name2, ColType::String, CompressType::Uncompressed);
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
                match &x.as_ref().uncompress()[0] {
                    Some(y) => assert_eq!(y, "Hello"),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        match test_table.get_column(&col_name2) {
            Column::String(x) => {
                match x.as_ref().uncompress()[0] {
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
        test_table.add_column(&col_name1, ColType::Number, CompressType::Uncompressed);
        test_table.add_column(&col_name2, ColType::String, CompressType::Uncompressed);
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
                match &x.as_ref().uncompress()[0] {
                    Some(y) => assert_eq!(*y, 112.0),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        match test_table.get_column(&col_name2) {
            Column::String(x) => {
                match &x.as_ref().uncompress()[0] {
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
        test_table.add_column(&col_name1, ColType::Number, CompressType::Uncompressed);
        test_table.add_column(&col_name2, ColType::String, CompressType::Uncompressed);
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
                match &x.as_ref().uncompress()[0] {
                    Some(y) => assert_eq!(*y, 112.2),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        match test_table.get_column(&col_name2) {
            Column::String(x) => {
                match &x.as_ref().uncompress()[1] {
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
        test_table.add_column(&col_name1, ColType::Number, CompressType::Uncompressed);
        test_table.add_column(&col_name2, ColType::String, CompressType::Uncompressed);
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