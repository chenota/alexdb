pub mod lexer {
    use regex::Regex;
    use crate::sqlscript::types::types::CompressType;

    use super::super::types::types::ColType;
    use super::super::types::types::SortType;
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
        LogOrKw,
        LogAndKw,
        StrEq,
        NotKw,
        Colon,
        Question,
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
        // Values (keyword)
        UndefinedKw,
        NullKw,
        // Values
        Number,
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
        CreateKw,
        TableKw,
        LimitKw,
        SortKw,
        ByKw,
        SortType,
        InitKw,
        CompKw,
        CompressKw,
        CompressType,
        // Type keywords
        NumberKw,
        StrKw,
        BoolKw
    }
    #[derive(Clone)]
    pub enum TokenValue {
        None,
        Number(f64),
        String(String),
        Boolean(bool),
        Type(ColType),
        SortType(SortType),
        CompressionType(CompressType)
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
    fn number_value (s: &str) -> TokenValue { TokenValue::Number(s.parse::<f64>().unwrap()) }
    fn ident_value(s: &str) -> TokenValue { TokenValue::String(s.to_string()) }
    fn bool_value(s: &str) -> TokenValue { TokenValue::Boolean(s == "true") }
    fn string_value(s: &str) -> TokenValue { TokenValue::String(s[1..(s.len()-1)].to_string()) } // Remove leading and trailing quotes
    fn num_type_value(s: &str) -> TokenValue { TokenValue::Type(ColType::Number) }
    fn str_type_value(s: &str) -> TokenValue { TokenValue::Type(ColType::String) }
    fn bool_type_value(s: &str) -> TokenValue { TokenValue::Type(ColType::Boolean) }
    fn sort_value(s: &str) -> TokenValue { if s == "ASC" { TokenValue::SortType(SortType::Ascending) } else { TokenValue::SortType(SortType::Descending) } }
    fn compression_value_none (s: &str) -> TokenValue { TokenValue::CompressionType(CompressType::Uncompressed) }
    fn compression_value_xor (s: &str) -> TokenValue { TokenValue::CompressionType(CompressType::Xor) }
    fn compression_value_bitmap (s: &str) -> TokenValue { TokenValue::CompressionType(CompressType::BitMap) }
    fn compression_value_runlen (s: &str) -> TokenValue { TokenValue::CompressionType(CompressType::RunLength) }
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
        (Some(TokenKind::CreateKw), reg!(r"CREATE"), none_value),
        (Some(TokenKind::TableKw), reg!(r"TABLE"), none_value),
        (Some(TokenKind::LimitKw), reg!(r"LIMIT"), none_value),
        (Some(TokenKind::SortKw), reg!(r"ORDER"), none_value),
        (Some(TokenKind::SortType), reg!(r"ASC"), sort_value),
        (Some(TokenKind::SortType), reg!(r"DESC"), sort_value),
        (Some(TokenKind::ByKw), reg!(r"BY"), none_value),
        (Some(TokenKind::InitKw), reg!(r"INIT"), none_value),
        (Some(TokenKind::CompKw), reg!(r"COMP"), none_value),
        (Some(TokenKind::CompressKw), reg!(r"COMPRESS"), none_value),
        (Some(TokenKind::CompressType), reg!(r"none"), compression_value_none),
        (Some(TokenKind::CompressType), reg!(r"bitmap"), compression_value_bitmap),
        (Some(TokenKind::CompressType), reg!(r"xor"), compression_value_xor),
        (Some(TokenKind::CompressType), reg!(r"runlen"), compression_value_runlen),
        // Type keywords
        (Some(TokenKind::NumberKw), reg!(r"num"), num_type_value),
        (Some(TokenKind::StrKw), reg!(r"str"), str_type_value),
        (Some(TokenKind::BoolKw), reg!(r"bool"), bool_type_value),
        // Value keywords
        (Some(TokenKind::UndefinedKw), reg!(r"undefined"), none_value),
        (Some(TokenKind::NullKw), reg!(r"null"), none_value),
        // Comparison
        (Some(TokenKind::Gte), reg!(r">="), none_value),
        (Some(TokenKind::Gt), reg!(r">"), none_value),
        (Some(TokenKind::Lte), reg!(r"<="), none_value),
        (Some(TokenKind::Lt), reg!(r"<"), none_value),
        (Some(TokenKind::Eq), reg!(r"=="), none_value),
        (Some(TokenKind::StrEq), reg!(r"==="), none_value),
        // Logical
        (Some(TokenKind::LogOrKw), reg!(r"\|\|"), none_value),
        (Some(TokenKind::LogAndKw), reg!(r"&&"), none_value),
        (Some(TokenKind::NotKw), reg!(r"!"), none_value),
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
        (Some(TokenKind::Question), reg!(r"\?"), none_value),
        (Some(TokenKind::Colon), reg!(r":"), none_value),
        // Values
        (Some(TokenKind::Number), reg!(r"[0-9]+\.[0-9]+"), number_value),
        (Some(TokenKind::Number), reg!(r"[0-9]+"), number_value),
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