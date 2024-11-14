pub mod table {
    use super::super::column::generic::*;
    use crate::sqlscript::types::types::{ ColType, Val, Expr };
    use crate::engine::script::env::Environment;
    use super::super::types::types::*;
    use bitvec::prelude::*;

    const compression_trigger_default: usize = 30;

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
        is_auto: BitVec,
        compression_strats: Vec<CompressionStrategy>,
        compression_trigger: usize,
    }
    impl Table {
        pub fn new() -> Table {
            Table {
                table: Vec::new(),
                headers: Vec::new(),
                size: 0,
                aggregates: Vec::new(),
                computations: Vec::new(),
                is_auto: BitVec::new(),
                compression_strats: Vec::new(),
                compression_trigger: compression_trigger_default,
            }
        }
        pub fn add_column(&mut self, name: &String, coltype: ColType) -> () {
            // Check that column does not already exist
            if self.headers.contains(&name) { panic!("Cannot insert duplicate columns") }
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
            // Push compression strategy
            self.is_auto.push(true);
            self.compression_strats.push(CompressionStrategy::Uncompressed);
        }
        pub fn add_populated_column(&mut self, name: &String, col: Column) -> () {
            // Check that column does not already exist
            if self.headers.contains(&name) { panic!("Cannot insert duplicate columns") }
            // Check length of column
            let len = match &col {
                Column::Boolean(cbox) => cbox.as_ref().len(),
                Column::Number(cbox) => cbox.as_ref().len(),
                Column::String(cbox) => cbox.as_ref().len(),
            };
            // Check that length of column matches current length
            if len != self.size { panic!("Inconsistent column length") }
            // Insert header
            self.headers.push(name.clone());
            // Insert column
            self.table.push(col);
            // Push compression strategy
            self.is_auto.push(true);
            self.compression_strats.push(CompressionStrategy::Uncompressed);
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
            // Check if should recompress
            if self.size >= self.compression_trigger {
                for i in self.is_auto.clone().iter_ones() {
                    let strategy = self.estimate_compression_type(i);
                    self.recompress(i, strategy.clone());
                    self.compression_strats[i] = strategy;
                }
                self.compression_trigger = self.size * 2;
            }
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
        pub fn update_aggregates(&mut self, vals: &Vec<Val>) {
            // Check length of values vector
            if vals.len() != self.aggregates.len() { panic!("Unequal lengths") }
            // Update all aggregate values
            let mut i: usize = 0;
            for val in vals {
                self.aggregates[i].1 = val.clone();
                i += 1;
            }
        }
        pub fn get_aggregates(&self) -> &Vec<(String, Val, Expr, Option<Expr>)> {
            &self.aggregates
        }
        pub fn get_aggregate(&self, name: &String) -> Val {
            // Find index of aggregate
            let ag_idx = self.aggregates.iter().position(|r| r.0 == *name).unwrap();
            // Return
            self.aggregates[ag_idx].1.clone()
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
        pub fn update_computations(&mut self, vals: &Vec<Val>) {
            // Check length of values vector
            if vals.len() != self.computations.len() { panic!("Unequal lengths") }
            // Update all aggregate values
            let mut i: usize = 0;
            for val in vals {
                self.computations[i].1 = val.clone();
                i += 1;
            }
        }
        pub fn get_computations(&self) -> &Vec<(String, Val, Expr)> {
            &self.computations
        }
        pub fn get_computation(&self, name: &String) -> Val {
            // Find index of computation
            let cmp_idx = self.computations.iter().position(|r| r.0 == *name).unwrap();
            // Return
            self.computations[cmp_idx].1.clone()
        }
        fn compression_sample_size(&self) -> usize {
            (25.0 * (self.size as f64).ln()) as usize
        }
        fn estimate_compression_type(&self, col_idx: usize) -> CompressionStrategy {
            let sample_size = self.compression_sample_size();
            match &self.table[col_idx] {
                Column::Boolean(_) => CompressionStrategy::Uncompressed,
                Column::String(_) => CompressionStrategy::Uncompressed,
                Column::Number(_) => CompressionStrategy::Uncompressed,
            }
        }
        fn recompress(&mut self, col_idx: usize, strategy: CompressionStrategy) {
            // If already compressing using chosen strategy, don't do anything
            if self.compression_strats[col_idx] == strategy { return }
            // Otherwise, compress accordingly
            match &mut self.table[col_idx] {
                Column::Boolean(_) => {
                    match strategy {
                        CompressionStrategy::Uncompressed => (),
                        _ => panic!("Boolean does not implement that compresison strategy")
                    }
                },
                Column::Number(curr) => {
                    let new_col: Box<dyn ColumnInterface<f64>> = match strategy {
                        CompressionStrategy::Bitmap => {
                            let mut new_col: BitMap<f64> = BitMap::new();
                            for item in curr.as_ref().iter() {
                                new_col.insert(item);
                            }
                            Box::new(new_col)
                        },
                        CompressionStrategy::RunLength => {
                            let mut new_col: RunLength<f64> = RunLength::new();
                            for item in curr.as_ref().iter() {
                                new_col.insert(item);
                            }
                            Box::new(new_col)
                        },
                        CompressionStrategy::Xor => {
                            let mut new_col: XorCol = XorCol::new();
                            for item in curr.as_ref().iter() {
                                new_col.insert(item);
                            }
                            Box::new(new_col)
                        },
                        CompressionStrategy::Uncompressed => {
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
                        CompressionStrategy::Bitmap => {
                            let mut new_col: BitMap<String> = BitMap::new();
                            for item in curr.as_ref().iter() {
                                new_col.insert(item);
                            }
                            Box::new(new_col)
                        },
                        CompressionStrategy::RunLength => {
                            let mut new_col: RunLength<String> = RunLength::new();
                            for item in curr.as_ref().iter() {
                                new_col.insert(item);
                            }
                            Box::new(new_col)
                        },
                        CompressionStrategy::Uncompressed => {
                            let mut new_col: Uncompressed<String> = Uncompressed::new();
                            for item in curr.as_ref().iter() {
                                new_col.insert(item);
                            }
                            Box::new(new_col)
                        },
                        _ => panic!("String does not implement that compression type")
                    };
                    *curr = new_col;
                }
            }
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