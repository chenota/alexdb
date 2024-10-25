mod lexer {
    pub enum Token {
        PlusKw,
        Integer(i64)
    }
    pub struct Lexer {
        stream: String,
        pos: u64
    }
    impl Lexer {
        fn produce(&mut self) -> (Token, u64) {
            (Token::PlusKw, 0)
        }
        fn reset(&mut self) -> () {
            self.pos = 0;
        }
    }
}

pub mod parser {

}