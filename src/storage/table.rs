pub mod table {
    use super::super::column::generic::*;
    use crate::sqlscript::types::types::{ ColType, Val, Expr, CompressType, str_of_ctype };
    use crate::engine::script::env::Environment;

    macro_rules! handle{
        ($e:expr) => {
            (match $e { Ok(v) => v, Err(s) => return Err(s) })
        }
    }

    enum IterCont<'a> {
        Number(Box<dyn Iterator<Item=Option<f64>> + 'a>),
        Boolean(Box<dyn Iterator<Item=Option<bool>> + 'a>),
        String(Box<dyn Iterator<Item=Option<String>> + 'a>),
    }

    pub struct Table {
        table: Vec<Column>,
        headers: Vec<String>,
        size: usize,
        aggregates: Vec<(String, Val, Expr, Option<Expr>)>,
        computations: Vec<(String, Val, Expr)>,
        compression_strats: Vec<CompressType>,
    }
    impl Table {
        pub fn new() -> Table {
            Table {
                table: Vec::new(),
                headers: Vec::new(),
                size: 0,
                aggregates: Vec::new(),
                computations: Vec::new(),
                compression_strats: Vec::new(),
            }
        }
        pub fn add_column(&mut self, name: &String, coltype: ColType, compression: CompressType) -> Result<(), String> {
            // Check that column does not already exist
            if self.headers.contains(&name) { return Err("Duplicate column ".to_string() + name) }
            // Add column name to headers
            self.headers.push(name.clone());
            // Push uncompressed column to table
            let col = match coltype {
                ColType::Boolean => {
                    let mut col_data: BoolCol = BoolCol::new();
                    for _ in 0..self.size { col_data.insert(None) }
                    Column::Boolean(Box::new(col_data))
                },
                ColType::Number => {
                    let mut col_data: Uncompressed<f64> = Uncompressed::new();
                    for _ in 0..self.size { col_data.insert(None) }
                    Column::Number(Box::new(col_data))
                },
                ColType::String => {
                    let mut col_data: Uncompressed<String> = Uncompressed::new();
                    for _ in 0..self.size { col_data.insert(None) }
                    Column::String(Box::new(col_data))
                }
            };
            // Push column
            self.table.push(col);
            // Push uncompressed
            self.compression_strats.push(CompressType::Uncompressed);
            // Change strategy of column
            handle!(self.recompress(self.headers.len() - 1, compression));
            Ok(())
        }
        pub fn add_populated_column(&mut self, name: &String, col: Column, compression: CompressType) -> Result<(), String> {
            // Check that column does not already exist
            if self.headers.contains(&name) { return Err("Duplicate column ".to_string() + name) }
            // Check length of column
            let len = match &col {
                Column::Boolean(cbox) => cbox.as_ref().len(),
                Column::Number(cbox) => cbox.as_ref().len(),
                Column::String(cbox) => cbox.as_ref().len(),
            };
            // Check that length of column matches current length
            if len != self.size { return Err("Could not insert pre-populated column due to size mismatch ".to_string() + name) }
            // Insert header
            self.headers.push(name.clone());
            // Insert column
            self.table.push(col);
            // Push uncompressed
            self.compression_strats.push(CompressType::Uncompressed);
            // Change strategy of column
            handle!(self.recompress(self.headers.len() - 1, compression));
            Ok(())
        }
        pub fn header_idx(&self, name: &String) -> Result<usize, String> {
            // Check that column exists
            if !self.headers.contains(name) { return Err("Invalid column name ".to_string() + name) }
            // Find column index
            Ok(self.headers.iter().position(|r| *r == *name).unwrap())
        }
        pub fn get_column(&self, name: &String) -> Result<&Column, String> {
            // Return column at index
            Ok(&self.table[handle!(self.header_idx(name))])
        }
        pub fn add_row(&mut self, data: Vec<Val>) -> Result<(), String> {
            // Check that vector has appropriate number of items
            if data.len() != self.table.len() { return Err("Number of items inserted does not match number of fields".to_string()) }
            // Add item to each column
            for i in 0..data.len() {
                match &mut self.table[i] {
                    Column::Number(vec) => {
                        match data[i] {
                            Val::NumVal(x) => vec.as_mut().insert(Some(x)),
                            Val::NullVal => vec.as_mut().insert(None),
                            _ => return Err("Cannot insert non-number into a number column".to_string())
                        }
                    },
                    Column::Boolean(vec) => {
                        match data[i] {
                            Val::BoolVal(x) => vec.as_mut().insert(Some(x)),
                            Val::NullVal => vec.as_mut().insert(None),
                            _ => return Err("Cannot insert non-boolean into a boolean column".to_string())
                        }
                    },
                    Column::String(vec) => {
                        match data[i].clone() {
                            Val::StrVal(x) => vec.as_mut().insert(Some(x)),
                            Val::NullVal => vec.as_mut().insert(None),
                            _ => return Err("Cannot insert non-string into a string column".to_string())
                        }
                    }
                }
            }
            // Increment size
            self.size += 1;
            Ok(())
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
        pub fn add_aggregate(&mut self, name: &String, val: &Val, expr: &Expr, init: &Option<Expr>) {
            self.aggregates.push((name.clone(), val.clone(), expr.clone(), init.clone()))
        }
        pub fn update_aggregates(&mut self, vals: &Vec<Val>) -> Result<(), String> {
            // Check length of values vector
            if vals.len() != self.aggregates.len() { return Err("Number of aggregate values given does not match number of aggregates stored".to_string()) }
            // Update all aggregate values
            let mut i: usize = 0;
            for val in vals {
                self.aggregates[i].1 = val.clone();
                i += 1;
            };
            Ok(())
        }
        pub fn get_aggregates(&self) -> &Vec<(String, Val, Expr, Option<Expr>)> {
            &self.aggregates
        }
        pub fn get_aggregate(&self, name: &String) -> Result<Val, String> {
            // Find index of aggregate
            let ag_idx = match self.aggregates.iter().position(|r| r.0 == *name) {
                Some(i) => i,
                _ => return Err("Aggregate ".to_string() + " does not exist")
            };
            // Return
            Ok(self.aggregates[ag_idx].1.clone())
        }
        pub fn push_aggregates(&self, env: &mut Environment) {
            for ag in &self.aggregates {
                env.push(&ag.0, &ag.1);
            }
        }
        pub fn push_all(&self, env: &mut Environment) {
            for ag in &self.aggregates {
                env.push(&ag.0, &ag.1);
            }
            for cmp in &self.computations {
                env.push(&cmp.0, &cmp.1)
            }
        }
        pub fn add_computation(&mut self, name: &String, val: &Val, expr: &Expr) {
            self.computations.push((name.clone(), val.clone(), expr.clone()))
        }
        pub fn update_computations(&mut self, vals: &Vec<Val>) -> Result<(), String> {
            // Check length of values vector
            if vals.len() != self.computations.len() { return Err("Number of values given does not match number of computations".to_string()) }
            // Update all aggregate values
            let mut i: usize = 0;
            for val in vals {
                self.computations[i].1 = val.clone();
                i += 1;
            };
            Ok(())
        }
        pub fn get_computations(&self) -> &Vec<(String, Val, Expr)> {
            &self.computations
        }
        pub fn get_computation(&self, name: &String) -> Result<Val, String> {
            // Find index of computation
            let cmp_idx = match self.computations.iter().position(|r| r.0 == *name) {
                Some(i) => i,
                _ => return Err("Computation ".to_string() + name + " does not exist")
            };
            // Return
            Ok(self.computations[cmp_idx].1.clone())
        }
        pub fn recompress(&mut self, col_idx: usize, strategy: CompressType) -> Result<(), String> {
            // If already compressing using chosen strategy, don't do anything
            if self.compression_strats[col_idx] == strategy { return Ok(()) }
            // Change comression strategy array
            self.compression_strats[col_idx] = strategy;
            // Otherwise, compress accordingly
            match &mut self.table[col_idx] {
                Column::Boolean(_) => {
                    match strategy {
                        CompressType::Uncompressed => (),
                        _ => return Err("Boolean columns do not implement compression type ".to_string() + &str_of_ctype(strategy))
                    }
                },
                Column::Number(curr) => {
                    let new_col: Box<dyn ColumnInterface<f64>> = match strategy {
                        CompressType::BitMap => {
                            let mut new_col: BitMap<f64> = BitMap::new();
                            for item in curr.as_ref().iter() {
                                new_col.insert(item);
                            }
                            Box::new(new_col)
                        },
                        CompressType::RunLength => {
                            let mut new_col: RunLength<f64> = RunLength::new();
                            for item in curr.as_ref().iter() {
                                new_col.insert(item);
                            }
                            Box::new(new_col)
                        },
                        CompressType::Xor => {
                            let mut new_col: XorCol = XorCol::new();
                            for item in curr.as_ref().iter() {
                                new_col.insert(item);
                            }
                            Box::new(new_col)
                        },
                        CompressType::Uncompressed => {
                            let mut new_col: Uncompressed<f64> = Uncompressed::new();
                            for item in curr.as_ref().iter() {
                                new_col.insert(item);
                            }
                            Box::new(new_col)
                        }
                    };
                    *curr = new_col;
                },
                Column::String(curr) => {
                    let new_col: Box<dyn ColumnInterface<String>> = match strategy {
                        CompressType::BitMap => {
                            let mut new_col: BitMap<String> = BitMap::new();
                            for item in curr.as_ref().iter() {
                                new_col.insert(item);
                            }
                            Box::new(new_col)
                        },
                        CompressType::RunLength => {
                            let mut new_col: RunLength<String> = RunLength::new();
                            for item in curr.as_ref().iter() {
                                new_col.insert(item);
                            }
                            Box::new(new_col)
                        },
                        CompressType::Uncompressed => {
                            let mut new_col: Uncompressed<String> = Uncompressed::new();
                            for item in curr.as_ref().iter() {
                                new_col.insert(item);
                            }
                            Box::new(new_col)
                        },
                        _ => return Err("String columns do not implement compression type ".to_string() + &str_of_ctype(strategy))
                    };
                    *curr = new_col;
                }
            };
            Ok(())
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