mod lexer {
    pub enum TokenKind {
        PlusKw,
        Integer,
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
    // Value generation functions
    fn none_value(_: &str) -> TokenValue { TokenValue::None }
    fn int_value(s: &str) -> TokenValue { TokenValue::Integer(s.parse::<i64>().unwrap()) }
    // Associates a kind of token with a regular expression that matches it, a function to derive a value.
    // If token kind is none, won't generate a token
    const TOKEN_MAP: &[(Option<TokenKind>, &str, fn(&str) -> TokenValue)] = &[
        // Operators
        (Some(TokenKind::PlusKw), r"\+", none_value),
        // Values
        (Some(TokenKind::Integer), r"(-?)[0-9]+", int_value),
    ];
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