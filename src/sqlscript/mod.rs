mod lexer {
    use regex::Regex;
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum TokenKind {
        // End of file
        EOF,
        // Operators
        PlusKw,
        MinusKw,
        TimesKw,
        DivKw,
        // Assignment
        AssignKw,
        // Semicolon
        SemiKw,
        // Values
        Integer,
        Float,
        Identifier,
        Boolean,
        String,
        // Grouping
        LParen,
        RParen,
    }
    #[derive(Clone)]
    pub enum TokenValue {
        None,
        Float(f64),
        String(String),
        Integer(i64),
        Boolean(bool)
    }
    #[derive(Clone)]
    pub struct Token {
        pub kind: TokenKind,
        pub value: TokenValue,
        pub start: usize,
        pub end: usize
    }
    // Add carrot to start
    macro_rules! reg{
        ($s:expr) => {
            concat!("^", $s)
        }
    }
    // Value generation functions
    fn none_value(_: &str) -> TokenValue { TokenValue::None }
    fn int_value(s: &str) -> TokenValue { TokenValue::Integer(s.parse::<i64>().unwrap()) }
    fn float_value(s: &str) -> TokenValue { TokenValue::Float(s.parse::<f64>().unwrap()) }
    fn ident_value(s: &str) -> TokenValue { TokenValue::String(s.to_string()) }
    fn bool_value(s: &str) -> TokenValue { TokenValue::Boolean(s == "true") }
    fn string_value(s: &str) -> TokenValue { TokenValue::String(s[1..(s.len()-1)].to_string()) } // Remove leading and trailing quotes
    // Associates a kind of token with a regular expression that matches it, a function to derive a value.
    // If token kind is none, won't generate a token
    const TOKEN_MAP: &[(Option<TokenKind>, &str, fn(&str) -> TokenValue)] = &[
        // Keywords
        (Some(TokenKind::Boolean), reg!(r"(false)|(true)"), bool_value),
        // Assignment
        (Some(TokenKind::AssignKw), reg!(r"="), none_value),
        // Semicolon
        (Some(TokenKind::SemiKw), reg!(r";"), none_value),
        // Grouping
        (Some(TokenKind::LParen), reg!(r"\("), none_value),
        (Some(TokenKind::RParen), reg!(r"\)"), none_value),
        // Operators
        (Some(TokenKind::PlusKw), reg!(r"\+"), none_value),
        (Some(TokenKind::MinusKw), reg!(r"-"), none_value),
        (Some(TokenKind::TimesKw), reg!(r"\*"), none_value),
        (Some(TokenKind::DivKw), reg!(r"/"), none_value),
        // Values
        (Some(TokenKind::Integer), reg!(r"(-?)[0-9]+"), int_value),
        (Some(TokenKind::Float), reg!(r"(-?)[0-9]+\.[0-9]+"), float_value),
        (Some(TokenKind::Identifier), reg!(r"[a-zA-Z][a-zA-Z0-9]*"), ident_value),
        (Some(TokenKind::String), reg!(r"'[^']*'"), string_value),
        // Whitespace
        (None, reg!(r"[ \t]+"), none_value),
    ];
    pub struct Lexer {
        stream: String,
        pos: usize
    }
    impl Lexer {
        // Create new lexer
        pub fn new(stream: String) -> Lexer {
            Lexer {
                stream: stream,
                pos: 0
            }
        }
        // Produce next token
        pub fn produce(&mut self) -> Token {
            // Length of stream
            let stream_len: usize = self.stream.len();
            // Check if length left to go
            if self.pos < stream_len {
                // Longest token
                let mut longest_token: Option<Token> = None;
                let mut longest_token_len: usize = 0;
                // Loop through token map
                for token in TOKEN_MAP {
                    // Compile regex
                    let re = Regex::new(token.1).unwrap();
                    // Regex find starting at pos
                    match re.find(&(self.stream.as_str()[self.pos..])) {
                        Some(mat) => {
                            // Length of match
                            let mat_len: usize = mat.end() - mat.start();
                            // Only change current longest if longer than that
                            if mat_len > longest_token_len {
                                // Check type of longest match
                                match token.0 {
                                    Some(kind) =>
                                        longest_token = Some(Token { kind: kind, value: (token.2)(mat.as_str()), start: mat.start() + self.pos, end: mat.end() + self.pos }),
                                    None => 
                                        longest_token = None
                                }
                                // Update longest len
                                longest_token_len = mat_len;
                            }
                        },
                        None => ()
                    }
                }
                // Panic if didn't find a longest token
                if longest_token_len == 0 {
                    panic!("Lexing error")
                }
                // Update self pos
                self.pos += longest_token_len;
                // Check if longest token is to be materialized or thrown away (if throw away, produce next token)
                match longest_token {
                    Some(token) => token,
                    None => self.produce()
                }
            } else {
                // Return EOF token if at end of stream
                Token {
                    kind: TokenKind::EOF, 
                    value: TokenValue::None, 
                    start: self.stream.len(), 
                    end: self.stream.len()
                }
            }
        }
        // Reset lexer
        pub fn reset(&mut self) -> () {
            self.pos = 0;
        }
    }
}

pub mod parser {
    use super::lexer;
    mod parsetree {
        use std::rc::Rc;
        pub enum Script {
            ExprScript(Expr),
            StmtScript(Val, Expr, Rc<Script>) // ident = expr; ...
        }
        pub enum Expr {
            BopExpr(Rc<Expr>, BopType, Rc<Expr>),
            ValExpr(Val)
        }
        pub enum Val {
            IntVal(i64),
            BoolVal(bool),
            StrVal(String),
            IdentVal(String),
        }
        pub enum BopType {
            Plus,
            Minus,
            Times,
            Div
        }
    }
    pub struct Parser {
        lexer: lexer::Lexer,
        token: lexer::Token
    }   
    impl Parser {
        // Constructor
        pub fn new(stream: String) -> Parser {
            let mut lexer = lexer::Lexer::new(stream);
            Parser {
                lexer: lexer,
                token: lexer.produce()
            }
        }
        // Program control
        fn peek(&self) -> &lexer::Token {
            &self.token
        }
        fn pop(&mut self) -> lexer::Token {
            let token = self.token.clone();
            self.token = self.lexer.produce();
            token
        }
        fn peek_expect(&self, kind: lexer::TokenKind) -> &lexer::Token {
            let token = self.peek();
            if token.kind != kind { panic!("Parsing error") };
            token
        }
        fn pop_expect(&mut self, kind: lexer::TokenKind) -> lexer::Token {
            let token = self.pop();
            if token.kind != kind { panic!("Parsing error") };
            token
        }
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn produce_first() -> Result<(), String> {
        // Setup
        let test_input: String = "4 + 5".to_string();
        let mut test_lexer: lexer::Lexer = lexer::Lexer::new(test_input);
        let next_token = test_lexer.produce();
        // Assert token
        assert_eq!(next_token.kind, lexer::TokenKind::Integer);
        Ok(())
    }
    #[test]
    fn produce_second() -> Result<(), String> {
        // Setup
        let test_input: String = "45 + 53".to_string();
        let mut test_lexer: lexer::Lexer = lexer::Lexer::new(test_input);
        test_lexer.produce();
        let next_token = test_lexer.produce();
        // Assert token
        assert_eq!(next_token.kind, lexer::TokenKind::PlusKw);
        Ok(())
    }
    #[test]
    fn produce_int_value() -> Result<(), String> {
        // Setup
        let test_input: String = "432 + 5".to_string();
        let mut test_lexer: lexer::Lexer = lexer::Lexer::new(test_input);
        let next_token = test_lexer.produce();
        // Assert token is integer type with correct value
        match next_token.value {
            lexer::TokenValue::Integer(x) => assert_eq!(x, 432),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn produce_bool_value() -> Result<(), String> {
        // Setup
        let test_input: String = "false".to_string();
        let mut test_lexer: lexer::Lexer = lexer::Lexer::new(test_input);
        let next_token = test_lexer.produce();
        // Assert token is boolean type
        assert_eq!(next_token.kind, lexer::TokenKind::Boolean);
        // Assert token is boolean type with correct value
        match next_token.value {
            lexer::TokenValue::Boolean(x) => assert_eq!(x, false),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn produce_ident_value() -> Result<(), String> {
        // Setup
        let test_input: String = "x".to_string();
        let mut test_lexer: lexer::Lexer = lexer::Lexer::new(test_input);
        let next_token = test_lexer.produce();
        // Assert token is identifier type
        assert_eq!(next_token.kind, lexer::TokenKind::Identifier);
        // Assert token is correct type with correct value
        match next_token.value {
            lexer::TokenValue::String(x) => assert_eq!(x, "x"),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn produce_string_value() -> Result<(), String> {
        // Setup
        let test_input: String = "'I love cats!'".to_string();
        let mut test_lexer: lexer::Lexer = lexer::Lexer::new(test_input);
        let next_token = test_lexer.produce();
        // Assert token is correct type with correct value
        match next_token.value {
            lexer::TokenValue::String(x) => assert_eq!(x, "I love cats!"),
            _ => assert!(false)
        }
        // Assert token is identifier type
        assert_eq!(next_token.kind, lexer::TokenKind::String);
        Ok(())
    }
    #[test]
    fn produce_eof() -> Result<(), String> {
        // Setup
        let test_input: String = "45 + 53".to_string();
        let mut test_lexer: lexer::Lexer = lexer::Lexer::new(test_input);
        for _ in 0..3 { test_lexer.produce(); }
        let next_token = test_lexer.produce();
        // Assert token
        assert_eq!(next_token.kind, lexer::TokenKind::EOF);
        Ok(())
    }
    #[test]
    fn lexer_reset() -> Result<(), String> {
        // Setup
        let test_input: String = "45 + 53".to_string();
        let mut test_lexer: lexer::Lexer = lexer::Lexer::new(test_input);
        test_lexer.produce();
        test_lexer.reset();
        let next_token = test_lexer.produce();
        // Assert token
        assert_eq!(next_token.kind, lexer::TokenKind::Integer);
        Ok(())
    }
}