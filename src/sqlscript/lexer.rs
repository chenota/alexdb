pub mod lexer {
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
        Gt,
        Gte,
        Lt,
        Lte,
        Eq,
        // Functions
        Arrow,
        FunKw,
        // Conditional
        IfKw,
        ThenKw,
        ElseKw,
        // Assignment
        AssignKw,
        // Semicolon
        SemiKw,
        // Comma,
        Comma,
        // Values
        Integer,
        Float,
        Identifier,
        Boolean,
        String,
        // Grouping
        LParen,
        RParen,
        LCBracket,
        RCBracket,
        // SQL keywords
        SelectKw,
        FromKw,
        WhereKw,
        InsertKw,
        IntoKw,
        ValuesKw,
        AggregateKw,
        ColumnKw,
        ConstKw,
        // Type keywords
        IntKw,
        FloatKw,
        StrKw,
        BoolKw
    }
    #[derive(Clone)]
    pub enum TokenValue {
        None,
        Float(f64),
        String(String),
        Integer(i64),
        Boolean(bool),
        Type(ColType)
    }
    #[derive(Clone)]
    pub struct Token {
        pub kind: TokenKind,
        pub value: TokenValue,
        pub start: usize,
        pub end: usize
    }
    #[derive(Clone)]
    pub enum ColType {
        Integer,
        Float,
        String,
        Boolean
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
    fn int_type_value(s: &str) -> TokenValue { TokenValue::Type(ColType::Integer) }
    fn float_type_value(s: &str) -> TokenValue { TokenValue::Type(ColType::Float) }
    fn str_type_value(s: &str) -> TokenValue { TokenValue::Type(ColType::String) }
    fn bool_type_value(s: &str) -> TokenValue { TokenValue::Type(ColType::Boolean) }
    // Associates a kind of token with a regular expression that matches it, a function to derive a value.
    // If token kind is none, won't generate a token
    const TOKEN_MAP: &[(Option<TokenKind>, &str, fn(&str) -> TokenValue)] = &[
        // Keywords
        (Some(TokenKind::Boolean), reg!(r"true"), bool_value),
        (Some(TokenKind::Boolean), reg!(r"false"), bool_value),
        (Some(TokenKind::IfKw), reg!(r"if"), none_value),
        (Some(TokenKind::ThenKw), reg!(r"then"), none_value),
        (Some(TokenKind::ElseKw), reg!(r"else"), none_value),
        // SQL keywords
        (Some(TokenKind::SelectKw), reg!(r"SELECT"), none_value),
        (Some(TokenKind::FromKw), reg!(r"FROM"), none_value),
        (Some(TokenKind::WhereKw), reg!(r"WHERE"), none_value),
        (Some(TokenKind::InsertKw), reg!(r"INSERT"), none_value),
        (Some(TokenKind::IntoKw), reg!(r"INTO"), none_value),
        (Some(TokenKind::ValuesKw), reg!(r"VALUES"), none_value),
        (Some(TokenKind::AggregateKw), reg!(r"AGGREGATE"), none_value),
        (Some(TokenKind::ColumnKw), reg!(r"COLUMN"), none_value),
        (Some(TokenKind::ConstKw), reg!(r"CONST"), none_value),
        // Type keywords
        (Some(TokenKind::IntKw), reg!(r"int"), int_type_value),
        (Some(TokenKind::FloatKw), reg!(r"float"), float_type_value),
        (Some(TokenKind::StrKw), reg!(r"str"), str_type_value),
        (Some(TokenKind::BoolKw), reg!(r"bool"), bool_type_value),
        // Comparison
        (Some(TokenKind::Gte), reg!(r">="), none_value),
        (Some(TokenKind::Gt), reg!(r">"), none_value),
        (Some(TokenKind::Lte), reg!(r"<="), none_value),
        (Some(TokenKind::Lt), reg!(r"<"), none_value),
        (Some(TokenKind::Eq), reg!(r"=="), none_value),
        // Function stuff
        (Some(TokenKind::Arrow), reg!(r"->"), none_value),
        (Some(TokenKind::FunKw), reg!(r"fun"), none_value),
        // Assignment
        (Some(TokenKind::AssignKw), reg!(r"="), none_value),
        // Semicolon
        (Some(TokenKind::SemiKw), reg!(r";"), none_value),
        // Comma
        (Some(TokenKind::Comma), reg!(r","), none_value),
        // Grouping
        (Some(TokenKind::LParen), reg!(r"\("), none_value),
        (Some(TokenKind::RParen), reg!(r"\)"), none_value),
        (Some(TokenKind::LCBracket), reg!(r"\{"), none_value),
        (Some(TokenKind::RCBracket), reg!(r"\}"), none_value),
        // Operators
        (Some(TokenKind::PlusKw), reg!(r"\+"), none_value),
        (Some(TokenKind::MinusKw), reg!(r"-"), none_value),
        (Some(TokenKind::TimesKw), reg!(r"\*"), none_value),
        (Some(TokenKind::DivKw), reg!(r"/"), none_value),
        // Values
        (Some(TokenKind::Integer), reg!(r"[0-9]+"), int_value),
        (Some(TokenKind::Float), reg!(r"[0-9]+\.[0-9]+"), float_value),
        (Some(TokenKind::Identifier), reg!(r"[a-zA-Z]([a-zA-Z0-9]|_)*"), ident_value),
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
        // Set/get position
        pub fn set_pos(&mut self, pos: usize) -> () {
            self.pos = pos;
        }
        pub fn get_pos(&self) -> usize {
            self.pos
        }
        // Testing
        pub fn remaining_stream(&self) -> &str {
            &self.stream.as_str()[self.pos..]
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
    fn produce_bool_value_2() -> Result<(), String> {
        // Setup
        let test_input: String = "true".to_string();
        let mut test_lexer: lexer::Lexer = lexer::Lexer::new(test_input);
        let next_token = test_lexer.produce();
        // Assert token is boolean type
        assert_eq!(next_token.kind, lexer::TokenKind::Boolean);
        // Assert token is boolean type with correct value
        match next_token.value {
            lexer::TokenValue::Boolean(x) => assert_eq!(x, true),
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
    fn produce_ident_value_underscore() -> Result<(), String> {
        // Setup
        let test_input: String = "field_two".to_string();
        let mut test_lexer: lexer::Lexer = lexer::Lexer::new(test_input);
        let next_token = test_lexer.produce();
        // Assert token is identifier type
        assert_eq!(next_token.kind, lexer::TokenKind::Identifier);
        // Assert token is correct type with correct value
        match next_token.value {
            lexer::TokenValue::String(x) => assert_eq!(x, "field_two"),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn produce_ident_value_number() -> Result<(), String> {
        // Setup
        let test_input: String = "field2".to_string();
        let mut test_lexer: lexer::Lexer = lexer::Lexer::new(test_input);
        let next_token = test_lexer.produce();
        // Assert token is identifier type
        assert_eq!(next_token.kind, lexer::TokenKind::Identifier);
        // Assert token is correct type with correct value
        match next_token.value {
            lexer::TokenValue::String(x) => assert_eq!(x, "field2"),
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
    fn produce_paren() -> Result<(), String> {
        // Setup
        let test_input: String = "( )".to_string();
        let mut test_lexer: lexer::Lexer = lexer::Lexer::new(test_input);
        let next_token = test_lexer.produce();
        let next_next_token = test_lexer.produce();
        // Assert token kind
        assert_eq!(next_token.kind, lexer::TokenKind::LParen);
        assert_eq!(next_next_token.kind, lexer::TokenKind::RParen);
        Ok(())
    }
    #[test]
    fn produce_int_after_plus() -> Result<(), String> {
        // Setup
        let test_input: String = "5 + 15".to_string();
        let mut test_lexer: lexer::Lexer = lexer::Lexer::new(test_input);
        test_lexer.produce();
        test_lexer.produce();
        let next_token = test_lexer.produce();
        // Assert token is integer type with correct value
        match next_token.value {
            lexer::TokenValue::Integer(x) => assert_eq!(x, 15),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn produce_if_kw() -> Result<(), String> {
        // Setup
        let test_input: String = "if true".to_string();
        let mut test_lexer: lexer::Lexer = lexer::Lexer::new(test_input);
        let next_token = test_lexer.produce();
        // Assert token is integer type with correct value
        match next_token.kind {
            lexer::TokenKind::IfKw => assert!(true),
            _ => assert!(false)
        }
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