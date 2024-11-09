pub mod generic {
    pub trait ColumnInterace<T> {
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
    impl<T> ColumnInterace<T> for Uncompressed<T> {
        fn insert(&mut self, data: Option<T>) -> () {
            self.data.push(data)
        } 
        fn extract(&self) -> &Vec<Option<T>> {
            &self.data
        }
    }
    pub enum Column {
        Number(Box<dyn ColumnInterace<f64>>),
        Boolean(Box<dyn ColumnInterace<bool>>),
        String(Box<dyn ColumnInterace<String>>)
    }
}