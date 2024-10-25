mod lexer {
    use regex::Regex;
    #[derive(Clone, Copy)]
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
    ];
    pub struct Lexer {
        stream: String,
        pos: usize
    }
    impl Lexer {
        fn produce(&mut self) -> Token {
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
                // Check if longest token is to be materialized or thrown away
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
        fn reset(&mut self) -> () {
            self.pos = 0;
        }
    }
}

pub mod parser {

}