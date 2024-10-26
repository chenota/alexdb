mod lexer {
    use regex::Regex;
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum TokenKind {
        EOF,
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
    // Add carrot to start
    macro_rules! reg{
        ($s:expr) => {
            concat!("^", $s)
        }
    }
    // Value generation functions
    fn none_value(_: &str) -> TokenValue { TokenValue::None }
    fn int_value(s: &str) -> TokenValue { TokenValue::Integer(s.parse::<i64>().unwrap()) }
    // Associates a kind of token with a regular expression that matches it, a function to derive a value.
    // If token kind is none, won't generate a token
    const TOKEN_MAP: &[(Option<TokenKind>, &str, fn(&str) -> TokenValue)] = &[
        // Operators
        (Some(TokenKind::PlusKw), reg!(r"\+"), none_value),
        // Values
        (Some(TokenKind::Integer), reg!(r"(-?)[0-9]+"), int_value),
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
            if self.pos < stream_len - 1 {
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

}

#[cfg(test)]
mod tests {
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