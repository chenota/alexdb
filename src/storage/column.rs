pub mod generic {
    pub trait Column<T> {
        fn insert(&mut self, data: T) -> ();
        fn extract(&self) -> Vec<T>;
    }
    pub enum ColumnContainer {
        IntColumn(Box<dyn Column<i64>>),
        FloatColumn(Box<dyn Column<f64>>),
        BooleanColumn(Box<dyn Column<bool>>),
        StringColumn(Box<dyn Column<String>>)
    }
}

pub mod intcolumn {
    use super::generic::*;

    pub struct Uncompressed {
        data: Vec<i64>
    }
    impl Uncompressed {
        pub fn new() -> Uncompressed {
            Uncompressed { data: Vec::new() }
        }
    }
    impl Column<i64> for Uncompressed {
        fn insert(&mut self, data: i64) -> () {
            self.data.push(data)
        }
        fn extract(&self) -> Vec<i64> {
            self.data.clone()
        }
    }
}

pub mod floatcolumn {
    use super::generic::*;

    pub struct Uncompressed {
        data: Vec<f64>
    }
    impl Uncompressed {
        pub fn new() -> Uncompressed {
            Uncompressed { data: Vec::new() }
        }
    }
    impl Column<f64> for Uncompressed {
        fn insert(&mut self, data: f64) -> () {
            self.data.push(data)
        }
        fn extract(&self) -> Vec<f64> {
            self.data.clone()
        }
    }
}

pub mod boolcolumn {
    use super::generic::*;

    pub struct Uncompressed {
        data: Vec<bool>
    }
    impl Uncompressed {
        pub fn new() -> Uncompressed {
            Uncompressed { data: Vec::new() }
        }
    }
    impl Column<bool> for Uncompressed {
        fn insert(&mut self, data: bool) -> () {
            self.data.push(data)
        }
        fn extract(&self) -> Vec<bool> {
            self.data.clone()
        }
    }
}

pub mod stringcolumn {
    use super::generic::*;

    pub struct Uncompressed {
        data: Vec<String>
    }
    impl Uncompressed {
        pub fn new() -> Uncompressed {
            Uncompressed { data: Vec::new() }
        }
    }
    impl Column<String> for Uncompressed {
        fn insert(&mut self, data: String) -> () {
            self.data.push(data)
        }
        fn extract(&self) -> Vec<String> {
            self.data.clone()
        }
    }
}