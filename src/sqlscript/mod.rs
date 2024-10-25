mod lexer {
    pub enum TokenKind {
        PlusKw,
        Integer
    }
    pub enum TokenValue {
        None,
        Float(f64),
        String(String),
        Integer(i64),
        Boolean(bool)
    }
    pub struct Token {
        pub kind: TokenKind,
        pub value: TokenValue,
        pub start: usize,
        pub end: usize
    }
    pub struct Lexer {
        stream: String,
        pos: u64
    }
    impl Lexer {
        fn produce(&mut self) -> Token {
            (Token::PlusKw, 0)
        }
        fn reset(&mut self) -> () {
            self.pos = 0;
        }
    }
}

pub mod parser {

}