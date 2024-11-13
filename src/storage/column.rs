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
                    // Increment index
                    self.index += 1;
                };
                // Return data
                data
            }
        }
    }
    struct BoolVec {
        data: Vec<u8>,
        len: usize
    }
    impl BoolVec {
        fn new() -> BoolVec {
            BoolVec {
                data: Vec::new(),
                len: 0
            }
        }
        fn push(&mut self, val: bool) {
            if self.len % 8 == 0 {
                self.data.push(0)
            }
            let size = self.data.len();
            self.data[size-1] = (self.data[size-1] << 1) & (val as u8);
            self.len += 1;
        }
        fn get(&self, idx: usize) -> bool {
            let data_idx = idx / 8;
            let data_offset = idx & 8;
            (self.data[data_idx] & (1 << data_offset)) != 0
        }
        fn len(&self) -> usize {
            self.len
        }
    }
    pub struct BitMap<T: Clone + PartialEq> {
        data: Vec<(T, BoolVec)>,
        len: usize,
        size: usize
    }
    impl<T: Clone + PartialEq> ColumnInterface<T> for BitMap<T> {
        fn insert(&mut self, data: Option<T>) -> () {
            // If no data yet, push new tuple if not pushing null
            if self.len == 0 {
                match data {
                    Some(x) => {
                        let mut new_vec = BoolVec::new();
                        new_vec.push(true);
                        self.data.push((x, new_vec));
                        self.size += 1;
                    },
                    None => ()
                }
            } 
            // Otherwise, compare inserted value to most recent tuple
            else {
                match data {
                    // If pushing null, push false to everything
                    None => {
                        for i in 0..self.size {
                            self.data[i].1.push(false)
                        }
                    },
                    Some(x) => {
                        // Locate data in locations
                        let idx = self.data.iter().position(|r| r.0 == x);
                        // Does idx exist?
                        match idx {
                            Some(i) => {
                                // Push true to matched value
                                self.data[i].1.push(true);
                                // Push false to every other value
                                for j in 0..self.size {
                                    if j != i { self.data[j].1.push(false) }
                                }
                            },
                            None => {
                                // Add new value
                                self.data.push((x, BoolVec::new()));
                                // Push false to every other value, n falses to new data
                                for j in 0..self.size { 
                                    self.data[j].1.push(false);
                                    self.data[self.size].1.push(false)
                                }
                                // Push one true to new data's bitmap
                                self.data[self.len].1.push(true);
                                // Increment size
                                self.size += 1;
                            }
                        }
                    }
                }
            }
            // Increment length
            self.len += 1;
        }
        fn extract(&self) -> Vec<Option<T>> {
            let mut uncompressed_data = Vec::new();
            for i in 0..self.len {
                let mut found_data = None;
                for j in 0..self.size {
                    if self.data[j].1.get(i) == true {
                        found_data = Some(self.data[j].0.clone());
                        break;
                    }
                }
                uncompressed_data.push(found_data)
            }
            uncompressed_data
        }
        fn iter<'a>(&'a self) -> Box<dyn Iterator<Item=Option<T>> + 'a> {
            Box::new(BitMapIterator {
                column: self,
                index: 0
            })
        }
        fn len(&self) -> usize {
            self.len
        }
    }
    struct BitMapIterator<'a, T: Clone + PartialEq> {
        column: &'a BitMap<T>,
        index: usize
    }
    impl<'a, T: Clone + PartialEq> Iterator for BitMapIterator<'a, T> {
        type Item = Option<T>;
        fn next(&mut self) -> Option<Self::Item> {
            if self.index >= self.column.len() {
                None
            } else {
                // Set data to none
                let mut data = None;
                // Check if any bitmaps are true
                for j in 0..self.column.data.len() {
                    if self.column.data[j].1.get(self.index) == true {
                        data = Some(self.column.data[j].0.clone());
                        break;
                    }
                }
                // Increment index
                self.index += 1;
                // Return data
                Some(data)
            }
        }
    }
    pub enum Column {
        Number(Box<dyn ColumnInterface<f64>>),
        Boolean(Box<dyn ColumnInterface<bool>>),
        String(Box<dyn ColumnInterface<String>>)
    }
}