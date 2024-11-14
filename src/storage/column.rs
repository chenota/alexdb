pub mod generic {
    use bitvec::prelude::*;
    pub trait ColumnInterface<T: Clone> {
        fn insert(&mut self, data: Option<T>) -> ();
        fn iter<'a>(&'a self) -> Box<dyn Iterator<Item=Option<T>> + 'a>;
        fn len(&self) -> usize;
        fn uncompress(&self) -> Vec<Option<T>> {
            let mut data = Vec::new();
            for item in self.iter() {
                data.push(item)
            };
            data
        }
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
    pub struct BoolCol {
        data: BitVec,
        len: usize
    }
    impl BoolCol {
        pub fn new() -> BoolCol {
            BoolCol{ data: BitVec::new(), len: 0 }
        }
    }
    impl ColumnInterface<bool> for BoolCol {
        fn insert(&mut self, data: Option<bool>) -> () {
            match data {
                None => self.data.push(false),
                Some(x) => {
                    self.data.push(true);
                    self.data.push(x)
                }
            }
            self.len += 1;
        } 
        fn iter<'a>(&'a self) -> Box<(dyn Iterator<Item = Option<bool>> + 'a)>{
            Box::new(BoolColIterator {
                column: self,
                index: 0
            })
        }
        fn len(&self) -> usize {
            self.len
        }
    }
    struct BoolColIterator<'a> {
        column: &'a BoolCol,
        index: usize,
    }
    impl<'a> Iterator for BoolColIterator<'a> {
        type Item = Option<bool>;
        fn next(&mut self) -> Option<Self::Item> {
            if self.index >= self.column.data.len() {
                None
            } else {
                let control = self.column.data[self.index];
                self.index += 1;
                match control {
                    false => Some(None),
                    true => {
                        let retval = Some(Some(self.column.data[self.index]));
                        self.index += 1;
                        retval
                    }
                }
            }
        }
    }
    pub struct RunLength<T: Clone + PartialEq> {
        data: Vec<(Option<T>, usize)>,
        len: usize,
        size: usize
    }
    impl<T: Clone + PartialEq> RunLength<T> {
        pub fn new() -> RunLength<T> {
            RunLength{
                data: Vec::new(),
                len: 0,
                size: 0
            }
        }    
    }
    impl<T: Clone + PartialEq> ColumnInterface<T> for RunLength<T> {
        fn insert(&mut self, data: Option<T>) -> () {
            // If no data yet, push new tuple
            if self.len == 0 {
                self.data.push((data, 1));
                self.size += 1;
            } 
            // Otherwise, compare inserted value to most recent tuple
            else {
                match (&data, &self.data[self.size - 1].0) {
                    (None, None) => {
                        self.data[self.size - 1].1 += 1
                    },
                    (Some(x), Some(y)) => {
                        if x == y { 
                            self.data[self.size - 1].1 += 1 
                        } else { 
                            self.data.push((data, 1));
                            self.size += 1;
                        }
                    },
                    (Some(_), None) | (None, Some(_)) => {
                        self.data.push((data, 1));
                        self.size += 1;
                    }
                }
            }
            self.len += 1;
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
    pub struct BitMap<T: Clone + PartialEq> {
        data: Vec<(T, BitVec)>,
        len: usize,
        size: usize
    }
    impl<T: Clone + PartialEq> BitMap<T> {
        pub fn new() -> BitMap<T> {
            BitMap {
                data: Vec::new(),
                len: 0,
                size: 0
            }
        }
    }
    impl<T: Clone + PartialEq> ColumnInterface<T> for BitMap<T> {
        fn insert(&mut self, data: Option<T>) -> () {
            // If no data yet, push new tuple if not pushing null
            if self.len == 0 {
                match data {
                    Some(x) => {
                        let mut new_vec = BitVec::new();
                        new_vec.push(true);
                        self.data.push((x, new_vec));
                        self.size += 1;
                    },
                    None => ()
                }
            } 
            // Otherwise, compare inserted value to existing tuples
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
                                self.data.push((x, BitVec::new()));
                                // Push false to every other value
                                for j in 0..self.size { 
                                    self.data[j].1.push(false);
                                }
                                // Push self.len falses to new data
                                for _ in 0..self.len {
                                    self.data[self.size].1.push(false)
                                }
                                // Push one true to new data's bitmap
                                self.data[self.size].1.push(true);
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
                    if self.column.data[j].1[self.index] == true {
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
    pub struct XorCol {
        data: BitVec,
        len: usize,
        prev_value: Option<u64>,
        // Following only need to be u8, but for some reason rust uses u32 for these values
        prev_num_leading: u32,
        prev_num_trailing: u32,
    }
    impl XorCol {
        pub fn new() -> XorCol {
            XorCol {
                data: BitVec::new(),
                len: 9,
                prev_value: None,
                prev_num_leading: 0,
                prev_num_trailing: 0
            }
        }
    }
    impl ColumnInterface<f64> for XorCol {
        fn insert(&mut self, data: Option<f64>) -> () {
            // Increment length
            self.len += 1;
            // Insert value
            match data {
                None => self.data.push(false),
                Some(x_f64) => {
                    // Inserted value as bits
                    let x = f64::to_bits(x_f64);
                    // Push true to indicate has first value
                    self.data.push(true);
                    // Has previous value?
                    match &self.prev_value {
                        None => {
                            // Push whole float to the column
                            for i in 0..64 {
                                self.data.push((x >> i) & 1 != 0);
                            }
                        },
                        Some(y) => {
                            // XOR value
                            let xor_val = x ^ *y;
                            // If XOR with the previous is zero, store single '0' bit
                            if xor_val == 0 {
                                self.data.push(false);
                            }
                            else {
                                // When XOR is non-zero, calculate the number of leading and trailing zeros in the XOR, store bit ‘1’
                                self.data.push(true);
                                // Calculate number of leading and trailing zeros
                                let num_leading = xor_val.leading_zeros();
                                let num_trailing = xor_val.trailing_zeros();
                                let meaningful_bits = xor_val >> num_trailing;
                                let num_meaningful_bits = 64 - num_leading - num_trailing;
                                // Case A: Number of leading and trailing same as previous
                                if num_leading == self.prev_num_leading && num_trailing == self.prev_num_trailing {
                                    // (Control bit ‘0’)
                                    self.data.push(false);
                                    // just store the meaningful XORed value
                                    for i in 0..num_meaningful_bits {
                                        self.data.push((meaningful_bits >> i) & 1 == 1)
                                    }
                                }
                                // Case B: Number of leading an trailing different than previous
                                else {
                                    // Store new trailing and leading bits
                                    self.prev_num_leading = num_leading;
                                    self.prev_num_trailing = num_trailing;
                                    // (Control bit ‘1’)
                                    self.data.push(true);
                                    // Store the length of the number of leading zeros in the next 5 bits
                                    for i in 0..5 {
                                        self.data.push((num_leading >> i) & 1 == 1)
                                    }
                                    // then store the length of the meaningful XORed value in the next 6 bits
                                    for i in 0..6 {
                                        self.data.push((num_meaningful_bits >> i) & 1 == 1)
                                    }
                                    // Finally store the meaningful bits of the XORed value.
                                    for i in 0..num_meaningful_bits {
                                        self.data.push((meaningful_bits >> i) & 1 == 1)
                                    }
                                }
                            }
                        }
                    }
                    // Indicate previous value
                    self.prev_value = Some(x)
                }
            }
        }
        fn iter<'a>(&'a self) -> Box<dyn Iterator<Item=Option<f64>> + 'a> {
            Box::new(XorColIterator {
                column: self,
                base_value: None,
                index: 0,
                prev_leading: 0,
                prev_trailing: 0
            })
        }
        fn len(&self) -> usize {
            self.len
        }
    }
    struct XorColIterator<'a> {
        column: &'a XorCol,
        base_value: Option<u64>,
        index: usize,
        prev_leading: u32,
        prev_trailing: u32
    }
    impl<'a> Iterator for XorColIterator<'a> {
        type Item = Option<f64>;
        fn next(&mut self) -> Option<Self::Item> {
            if self.index >= self.column.data.len() {
                None
            } else {
                // Read first bit stored at index
                let first_bit: bool = self.column.data[self.index];
                self.index += 1;
                match first_bit {
                    // If first bit is a zero, return none
                    false => Some(None),
                    true => {
                        // Check if have a base value
                        match self.base_value {
                            // Have base value
                            Some(x) => {
                                // Check if is the same or not
                                let same_val = ! self.column.data[self.index];
                                self.index += 1;
                                // If same value, return value again
                                if same_val {
                                    Some(Some(f64::from_bits(x)))
                                } else {
                                    // Determine 'control bit'
                                    let control_bit = self.column.data[self.index];
                                    self.index += 1;
                                    // Control bit true or false?
                                    if control_bit == false {
                                        let meaningful_size = 64 - self.prev_leading - self.prev_trailing;
                                        let mut new_value: u64 = 0;
                                        // Push lower bits of base value
                                        for i in 0..self.prev_trailing {
                                            new_value = new_value | (((x >> i) & 1) << i)
                                        }
                                        // Push inverse of meaningful XORed bits
                                        for i in 0..meaningful_size {
                                            new_value = new_value | ((!self.column.data[self.index] as u64) << (i + self.prev_trailing));
                                            self.index += 1;
                                        }
                                        // Push upper bits of base value
                                        for i in 0..self.prev_leading {
                                            new_value = new_value | (((x >> (i + self.prev_trailing + meaningful_size)) & 1) << (i + self.prev_trailing + meaningful_size))
                                        }
                                        // Set base value to be new value
                                        self.base_value = Some(new_value);
                                        // Return new value
                                        Some(Some(f64::from_bits(new_value)))
                                    } else {
                                        // Get the length of the number of leading zeros
                                        let mut num_leading_zeros: u32 = 0;
                                        for i in 0..5 {
                                            num_leading_zeros = num_leading_zeros | (self.column.data[self.index] as u32) << i;
                                            self.index += 1;
                                        }
                                        // Get length of the meaningful XORed value
                                        let mut meaningful_size: u32 = 0;
                                        for i in 0..6 {
                                            meaningful_size = meaningful_size | (self.column.data[self.index] as u32) << i;
                                            self.index += 1;
                                        }
                                        // Reconstruct meaningful XORed value
                                        let mut xor_meaningful: u32 = 0;
                                        for i in 0..meaningful_size {
                                            xor_meaningful = xor_meaningful | (self.column.data[self.index] as u32) << i;
                                            self.index += 1;
                                        }
                                        // Calcualte the number of trailing zeros
                                        let num_trailing_zeros: u32 = 64 - meaningful_size - num_leading_zeros;
                                        // Update iterator
                                        self.prev_leading = num_leading_zeros;
                                        self.prev_trailing = num_trailing_zeros;
                                        // Construct base value
                                        let mut new_value: u64 = 0;
                                        // Push lower bits of base value
                                        for i in 0..self.prev_trailing {
                                            new_value = new_value | (((x >> i) & 1) << i)
                                        }
                                        // Push lower bits of base value
                                        for i in 0..self.prev_trailing {
                                            new_value = new_value | (((x >> i) & 1) << i)
                                        }
                                        // Push inverse of meaningful XORed bits
                                        for i in 0..meaningful_size {
                                            new_value = new_value | ((!self.column.data[self.index] as u64) << (i + self.prev_trailing));
                                            self.index += 1;
                                        }
                                        // Push upper bits of base value
                                        for i in 0..self.prev_leading {
                                            new_value = new_value | (((x >> (i + self.prev_trailing + meaningful_size)) & 1) << (i + self.prev_trailing + meaningful_size))
                                        }
                                        // Set base value to be new value
                                        self.base_value = Some(new_value);
                                        // Return new value
                                        Some(Some(f64::from_bits(new_value)))
                                    }
                                }
                            },
                            // No base value
                            None => {
                                // Read base value (next 64 bits)
                                let mut base_value_bits: u64 = 0;
                                for i in 0..64 {
                                    base_value_bits = base_value_bits | ((self.column.data[self.index] as u64) << i);
                                    self.index += 1;
                                };
                                // Store base value
                                self.base_value = Some(base_value_bits);
                                // Return base value
                                Some(Some(f64::from_bits(base_value_bits)))
                            }
                        }
                    }
                }
            }
        }
    }
    pub enum Column {
        Number(Box<dyn ColumnInterface<f64>>),
        Boolean(Box<dyn ColumnInterface<bool>>),
        String(Box<dyn ColumnInterface<String>>)
    }
}