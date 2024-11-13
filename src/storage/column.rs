pub mod generic {
    pub trait ColumnInterface<T: Clone> {
        fn insert(&mut self, data: Option<T>) -> ();
        fn extract(&self) -> Vec<Option<T>>;
        fn iter<'a>(&'a self) -> Box<dyn Iterator<Item=Option<T>> + 'a>;
        fn len(&self) -> usize;
    }
    pub struct Uncompressed<T: Clone> {
        data: Vec<Option<T>>
    }
    impl<T: Clone> Uncompressed<T> {
        pub fn new() -> Uncompressed<T> {
            Uncompressed{ data: Vec::new() }
        }
    }
    impl<T: Clone> ColumnInterface<T> for Uncompressed<T> {
        fn insert(&mut self, data: Option<T>) -> () {
            self.data.push(data)
        } 
        fn extract(&self) -> Vec<Option<T>> {
            self.data.clone()
        }
        fn iter<'a>(&'a self) -> Box<(dyn Iterator<Item = Option<T>> + 'a)>{
            Box::new(UncompressedIterator {
                column: self,
                index: 0
            })
        }
        fn len(&self) -> usize {
            self.data.len()
        }
    }
    struct UncompressedIterator<'a, T: Clone> {
        column: &'a Uncompressed<T>,
        index: usize,
    }
    impl<'a, T: Clone> Iterator for UncompressedIterator<'a, T> {
        type Item = Option<T>;
        fn next(&mut self) -> Option<Self::Item> {
            if self.index >= self.column.len() {
                None
            } else {
                let data = Some(self.column.data[self.index].clone());
                self.index += 1;
                data
            }
        }
    }
    pub struct RunLength<T: Clone + PartialEq> {
        data: Vec<(Option<T>, usize)>,
        len: usize
    }
    impl<T: Clone + PartialEq> ColumnInterface<T> for RunLength<T> {
        fn insert(&mut self, data: Option<T>) -> () {
            // If no data yet, push new tuple
            if self.len == 0 {
                self.data.push((data, 1))
            } 
            // Otherwise, compare inserted value to most recent tuple
            else {
                match (&data, &self.data[self.len - 1].0) {
                    (None, None) => {
                        self.data[self.len - 1].1 += 1
                    },
                    (Some(x), Some(y)) => {
                        if x == y { 
                            self.data[self.len - 1].1 += 1 
                        } else { 
                            self.data.push((data, 1)) 
                        }
                    },
                    (Some(_), None) | (None, Some(_)) => {
                        self.data.push((data, 1))
                    }
                }
            }
            self.len += 1;
        }
        fn extract(&self) -> Vec<Option<T>> {
            let mut uncompressed_data = Vec::new();
            for tup in &self.data {
                for _ in 0..tup.1 {
                    uncompressed_data.push(tup.0.clone())
                }
            };
            uncompressed_data
        }
        fn iter<'a>(&'a self) -> Box<dyn Iterator<Item=Option<T>> + 'a> {
            Box::new(RunLengthIterator {
                column: self,
                index: 0,
                pos: 0
            })
        }
        fn len(&self) -> usize {
            self.len
        }
    }
    struct RunLengthIterator<'a, T: Clone + PartialEq> {
        column: &'a RunLength<T>,
        index: usize,
        pos: usize
    }
    impl<'a, T: Clone + PartialEq> Iterator for RunLengthIterator<'a, T> {
        type Item = Option<T>;
        fn next(&mut self) -> Option<Self::Item> {
            if self.index >= self.column.data.len() {
                None
            } else {
                // Get data
                let data = Some(self.column.data[self.index].0.clone());
                // Increment position in tuple
                self.pos += 1;
                // If overshot, move on to next tuple
                if self.pos >= self.column.data[self.index].1 {
                    self.pos = 0;
                    self.index += 1;
                };
                // Return data
                data
            }
        }
    }
    pub enum Column {
        Number(Box<dyn ColumnInterface<f64>>),
        Boolean(Box<dyn ColumnInterface<bool>>),
        String(Box<dyn ColumnInterface<String>>)
    }
}