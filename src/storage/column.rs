pub mod generic {
    pub trait Column<T> {
        fn insert(&mut self, data: Option<T>) -> ();
        fn extract(&self) -> &Vec<Option<T>>;
    }
    pub struct Uncompressed<T> {
        data: Vec<Option<T>>
    }
    impl<T> Uncompressed<T> {
        pub fn new() -> Uncompressed<T> {
            Uncompressed{ data: Vec::new() }
        }
    }
    impl<T> Column<T> for Uncompressed<T> {
        fn insert(&mut self, data: Option<T>) -> () {
            self.data.push(data)
        } 
        fn extract(&self) -> &Vec<Option<T>> {
            &self.data
        }
    }
    pub enum ColumnContainer {
        IntColumn(Box<dyn Column<i64>>),
        FloatColumn(Box<dyn Column<f64>>),
        BooleanColumn(Box<dyn Column<bool>>),
        StringColumn(Box<dyn Column<String>>)
    }
}