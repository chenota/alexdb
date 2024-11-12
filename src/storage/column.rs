pub mod generic {
    pub trait ColumnInterace<T: Clone> {
        fn insert(&mut self, data: Option<T>) -> ();
        fn extract(&self) -> &Vec<Option<T>>;
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
    impl<T: Clone> ColumnInterace<T> for Uncompressed<T> {
        fn insert(&mut self, data: Option<T>) -> () {
            self.data.push(data)
        } 
        fn extract(&self) -> &Vec<Option<T>> {
            &self.data
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
            if self.index >= self.column.data.len() {
                None
            } else {
                let data = Some(self.column.data[self.index].clone());
                self.index += 1;
                data
            }
        }
    }
    pub enum Column {
        Number(Box<dyn ColumnInterace<f64>>),
        Boolean(Box<dyn ColumnInterace<bool>>),
        String(Box<dyn ColumnInterace<String>>)
    }
}