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

pub mod parser {
    use super::lexer::*;
    use std::rc::Rc;
    pub mod parsetree {
        use std::rc::Rc;
        pub enum Script {
            ExprScript(Expr),
            StmtScript(String, Expr, Rc<Script>) // ident = expr; ...
        }
        pub enum Expr {
            BopExpr(Rc<Expr>, BopType, Rc<Expr>),
            UopExpr(UopType, Rc<Expr>),
            ScriptExpr(Rc<Script>),
            ValExpr(Val),
            CallExpr(Rc<Expr>, Option<ExprList>)
        }
        pub enum Val {
            IntVal(i64),
            BoolVal(bool),
            StrVal(String),
            IdentVal(String),
            FloatVal(f64),
        }
        pub enum ExprList {
            MultiList(Rc<Expr>, Rc<ExprList>),
            SingleList(Rc<Expr>)
        }
        #[derive(PartialEq, Debug)]
        pub enum BopType {
            PlusBop,
            MinusBop,
            TimesBop,
            DivBop
        }
        #[derive(PartialEq, Debug)]
        pub enum UopType {
            NegUop,
        }
    }
    pub struct Parser {
        lexer: Lexer,
        token: Token
    }   
    impl Parser {
        // Constructor
        pub fn new(stream: String) -> Parser {
            let mut lexer = Lexer::new(stream);
            let token = lexer.produce();
            Parser {
                lexer: lexer,
                token: token
            }
        }
        // Program control
        fn peek(&self) -> &Token {
            &self.token
        }
        fn peek_ahead(&mut self) -> Token {
            // Save current position
            let pos_save = self.lexer.get_pos();
            // Produce next token
            let next_token = self.lexer.produce();
            // Reset lexer
            self.lexer.set_pos(pos_save);
            // Return
            next_token
        }
        fn peek_bop(&self) -> Option<parsetree::BopType> {
            // Peek first token
            let token = self.peek();
            // Match type, return appropriate value
            match token.kind {
                TokenKind::PlusKw => Some(parsetree::BopType::PlusBop),
                TokenKind::MinusKw => Some(parsetree::BopType::MinusBop),
                TokenKind::TimesKw => Some(parsetree::BopType::TimesBop),
                TokenKind::DivKw => Some(parsetree::BopType::DivBop),
                _ => None
            }
        }
        fn pop(&mut self) -> Token {
            let old_token = self.token.clone();
            self.token = self.lexer.produce();
            old_token
        }
        fn peek_expect(&self, kind: TokenKind) -> &Token {
            let token = self.peek();
            if token.kind != kind { panic!("Parsing error") };
            token
        }
        fn pop_expect(&mut self, kind: TokenKind) -> Token {
            let old_token = self.pop();
            if old_token.kind != kind { panic!("Parsing error") };
            old_token
        }
        // Parsing entry point
        pub fn parse_script(&mut self) -> parsetree::Script {
            // Reset lexer
            self.lexer.reset();
            // Produce first token
            self.token = self.lexer.produce();
            // Call start symbol (script for now, will eventually be query)
            self.script()
        }
        // Parsing functions
        fn script(&mut self) -> parsetree::Script {
            match self.peek_ahead().kind {
                // If 2nd token is an assignment, parse as statement
                TokenKind::AssignKw => {
                    // Save ident value
                    let ident_val: String = match &self.token.value {
                        TokenValue::String(x) => x.clone(),
                        _ => panic!("Parsing error")
                    };
                    // Pop ident and assignment
                    self.pop();
                    self.pop();
                    // Parse expr
                    let expr: parsetree::Expr = self.expr();
                    println!("{:?}", self.peek().kind);
                    // Expect semicolon, pop it
                    self.pop_expect(TokenKind::SemiKw);
                    // Return constructed statement
                    parsetree::Script::StmtScript(ident_val, expr, Rc::new(self.script()))
                }
                // Parse as expression
                _ => parsetree::Script::ExprScript(self.expr())
            }
        }
        fn expr(&mut self) -> parsetree::Expr {
            // Check first value
            let first = match self.peek().kind {
                TokenKind::LParen => {
                    // Pop lparen
                    self.pop();
                    // Parse expr
                    let expr = self.expr();
                    // Expect rparen, pop it
                    self.pop_expect(TokenKind::RParen);
                    // Return parsed expression
                    expr
                },
                TokenKind::LCBracket => {
                    // Pop curly bracket
                    self.pop();
                    // Parse script
                    let script = self.script();
                    // Expect right curly bracket, pop it
                    self.pop_expect(TokenKind::RCBracket);
                    // Return parsed expression
                    parsetree::Expr::ScriptExpr(Rc::new(script))
                },
                TokenKind::MinusKw => {
                    // Pop minus sign
                    self.pop();
                    // Parse expr
                    let expr = self.expr();
                    // Return negated expression
                    parsetree::Expr::UopExpr(parsetree::UopType::NegUop, Rc::new(expr))
                },
                _ => parsetree::Expr::ValExpr(self.value())
            };
            // Check if postfix (call only postfix) at front
            let second = match self.peek().kind {
                TokenKind::LParen => {
                    // Pop LParen
                    self.pop();
                    // Check if next is rparen. If not, parse exprlist
                    let elist = match self.peek().kind {
                        TokenKind::RParen => None,
                        _ => Some(self.exprlist())
                    };
                    // Expect RParen
                    self.pop_expect(TokenKind::RParen);
                    // Construct expression
                    parsetree::Expr::CallExpr(Rc::new(first), elist)
                },
                _ => first
            };
            // Check if bop at front
            match self.peek_bop() {
                Some(bop) => {
                    // Pop bop
                    self.pop();
                    // Return parsed expr
                    parsetree::Expr::BopExpr(Rc::new(second), bop, Rc::new(self.expr()))
                },
                None => second
            }
        }
        fn value(&mut self) -> parsetree::Val {
            // Pop first token
            let token = self.pop();
            // Match type, return appropriate value
            match token.value {
                TokenValue::Integer(x) => parsetree::Val::IntVal(x),
                TokenValue::Boolean(x) => parsetree::Val::BoolVal(x),
                TokenValue::Float(x) => parsetree::Val::FloatVal(x),
                // String could be either ident or string value
                TokenValue::String(x) => {
                    match token.kind {
                        TokenKind::String => parsetree::Val::StrVal(x),
                        TokenKind::Identifier => parsetree::Val::IdentVal(x),
                        _ => panic!("Parsing error")
                    }
                }
                _ => panic!("Parsing error")
            }
        }
        fn exprlist(&mut self) -> parsetree::ExprList {
            // Parse expr
            let expr = self.expr();
            // Check if comma
            match self.peek().kind {
                TokenKind::Comma => {
                    // Pop comma
                    self.pop();
                    // Parse next expr
                    parsetree::ExprList::MultiList(Rc::new(expr), Rc::new(self.exprlist()))
                },
                _ => parsetree::ExprList::SingleList(Rc::new(expr))
            }
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
#[cfg(test)]
mod parser_tests {
    use super::parser::*;
    #[test]
    fn parser_integer() -> Result<(), String> {
        // Setup
        let test_input: String = "4".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse_script();
        // Assert correct AST
        match ast {
            // Should be just expr
            parsetree::Script::ExprScript(e1) => {
                match e1 {
                    // Should be val expr
                    parsetree::Expr::ValExpr(v) => {
                        // First value should be four
                        match v {
                            parsetree::Val::IntVal(x) => assert_eq!(x, 4),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_basic_arithmetic() -> Result<(), String> {
        // Setup
        let test_input: String = "5 + 10".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse_script();
        // Assert correct AST
        match ast {
            // Should be just expr
            parsetree::Script::ExprScript(e1) => {
                match e1 {
                    // Should be bop expr
                    parsetree::Expr::BopExpr(v, t, _) => {
                        // Bop type should be plus
                        assert_eq!(t, parsetree::BopType::PlusBop);
                        // First value should be four
                        match v.as_ref() {
                            parsetree::Expr::ValExpr(y) => {
                                match y {
                                    parsetree::Val::IntVal(x) => assert_eq!(*x, 5),
                                    _ => assert!(false)
                                }
                            },
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_nontrivial_arithmetic() -> Result<(), String> {
        // Setup
        let test_input: String = "(5 + 10) - 3".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse_script();
        // Assert correct AST
        match ast {
            // Should be just expr
            parsetree::Script::ExprScript(e1) => {
                match e1 {
                    // Should be bop expr
                    parsetree::Expr::BopExpr(_, t, v) => {
                        // Bop type should be plus
                        assert_eq!(t, parsetree::BopType::MinusBop);
                        // First value should be four
                        match v.as_ref() {
                            parsetree::Expr::ValExpr(y) => {
                                match y {
                                    parsetree::Val::IntVal(x) => assert_eq!(*x, 3),
                                    _ => assert!(false)
                                }
                            },
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_very_nested() -> Result<(), String> {
        // Setup
        let test_input: String = "((((((((((((4))))))))))))".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse_script();
        // Assert correct AST
        match ast {
            // Should be just expr
            parsetree::Script::ExprScript(e1) => {
                match e1 {
                    // Should be val expr
                    parsetree::Expr::ValExpr(v) => {
                        // First value should be four
                        match v {
                            parsetree::Val::IntVal(x) => assert_eq!(x, 4),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_basic_stmt() -> Result<(), String> {
        // Setup
        let test_input: String = "x = 5; x".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse_script();
        // Assert correct AST
        match ast {
            // Should be stmtscript
            parsetree::Script::StmtScript(id, e1, sc) => {
                // Make sure ID is x
                assert_eq!(id, "x");
                // Check e1
                match e1 {
                    // Should be val expr w/ 5
                    parsetree::Expr::ValExpr(v) => {
                        match v {
                            parsetree::Val::IntVal(x) => assert_eq!(x, 5),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
                // Check type of proceeding script
                match sc.as_ref() {
                    parsetree::Script::ExprScript(_) => assert!(true),
                    _ => assert!(false)
                }
            }
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_nontrivial_stmt() -> Result<(), String> {
        // Setup
        let test_input: String = "x = (3+2)-1; x - 15".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse_script();
        // Assert correct AST
        match ast {
            // Should be stmtscript
            parsetree::Script::StmtScript(id, _, sc) => {
                // Make sure ID is x
                assert_eq!(id, "x");
                // Check type of proceeding script
                match sc.as_ref() {
                    parsetree::Script::ExprScript(_) => assert!(true),
                    _ => assert!(false)
                }
            }
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_nested_stmt() -> Result<(), String> {
        // Setup
        let test_input: String = "x = 5; y = 10; x + y".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse_script();
        // Assert correct AST
        match ast {
            // Should be stmtscript
            parsetree::Script::StmtScript(_, _, sc1) => {
                // Check type of proceeding script
                match sc1.as_ref() {
                    parsetree::Script::StmtScript(_, _, sc2) => {
                        match sc2.as_ref() {
                            parsetree::Script::ExprScript(_) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    _ => assert!(false)
                }
            }
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_script_expr() -> Result<(), String> {
        // Setup
        let test_input: String = "{a = 4; a} + 5".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse_script();
        // Assert correct AST
        match ast {
            // Should be stmtscript
            parsetree::Script::ExprScript(e1) => {
                // Check type of proceeding script
                match e1 {
                    parsetree::Expr::BopExpr(e11, _, _) => {
                        match e11.as_ref() {
                            parsetree::Expr::ScriptExpr(_) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    _ => assert!(false)
                }
            }
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_script_script_expr() -> Result<(), String> {
        // Setup
        let test_input: String = "x = {a = 4; a}; x".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse_script();
        // Assert correct AST
        match ast {
            // Should be stmtscript
            parsetree::Script::StmtScript(_, e1, _) => {
                // Check type of proceeding script
                match e1 {
                    parsetree::Expr::ScriptExpr(_) => assert!(true),
                    _ => assert!(false)
                }
            }
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_uop_expr() -> Result<(), String> {
        // Setup
        let test_input: String = "-5".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse_script();
        // Assert correct AST
        match ast {
            // Should be exprscript
            parsetree::Script::ExprScript(e1) => {
                // Check type of proceeding script
                match e1 {
                    parsetree::Expr::UopExpr(t, e2) => {
                        assert_eq!(t, parsetree::UopType::NegUop);
                        match e2.as_ref() {
                            parsetree::Expr::ValExpr(_) => assert!(true),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_uop_expr_2() -> Result<(), String> {
        // Setup
        let test_input: String = "0 - -1".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse_script();
        // Assert correct AST
        match ast {
            // Should be exprscript
            parsetree::Script::ExprScript(e1) => {
                // Check type of proceeding script
                match e1 {
                    parsetree::Expr::BopExpr(_, _, e2) => {
                        match e2.as_ref() {
                            parsetree::Expr::UopExpr(t, _) => assert_eq!(*t, parsetree::UopType::NegUop),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_call_expr() -> Result<(), String> {
        // Setup
        let test_input: String = "x()".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse_script();
        // Assert correct AST
        match ast {
            // Should be exprscript
            parsetree::Script::ExprScript(e1) => {
                // Check type of proceeding script
                match e1 {
                    parsetree::Expr::CallExpr(e1, args) => {
                        match args {
                            None => assert!(true),
                            _ => assert!(false)
                        }
                        match e1.as_ref() {
                            parsetree::Expr::ValExpr(_) => assert!(true),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_call_expr_2() -> Result<(), String> {
        // Setup
        let test_input: String = "x() + y".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse_script();
        // Assert correct AST
        match ast {
            // Should be exprscript
            parsetree::Script::ExprScript(e1) => {
                // Check type of proceeding script
                match e1 {
                    parsetree::Expr::BopExpr(e2, _, _) => {
                        match e2.as_ref() {
                            parsetree::Expr::CallExpr(e3, args) => {
                                match args {
                                    None => assert!(true),
                                    _ => assert!(false)
                                }
                                match e3.as_ref() {
                                    parsetree::Expr::ValExpr(_) => assert!(true),
                                    _ => assert!(false)
                                }
                            }
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_call_expr_3() -> Result<(), String> {
        // Setup
        let test_input: String = "y + x()".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse_script();
        // Assert correct AST
        match ast {
            // Should be exprscript
            parsetree::Script::ExprScript(e1) => {
                // Check type of expr
                match e1 {
                    parsetree::Expr::BopExpr(_, _, _) => assert!(true),
                    _ => assert!(false)
                }
            }
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_call_expr_4() -> Result<(), String> {
        // Setup
        let test_input: String = "x(1)".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse_script();
        // Assert correct AST
        match ast {
            // Should be exprscript
            parsetree::Script::ExprScript(e1) => {
                // Check type of expr
                match e1 {
                    parsetree::Expr::CallExpr(_, args) => {
                        match args {
                            Some(parsetree::ExprList::SingleList(_)) => assert!(true),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_call_expr_5() -> Result<(), String> {
        // Setup
        let test_input: String = "x(1,2,3)".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse_script();
        // Assert correct AST
        match ast {
            // Should be exprscript
            parsetree::Script::ExprScript(e1) => {
                // Check type of expr
                match e1 {
                    parsetree::Expr::CallExpr(_, args) => {
                        match args {
                            Some(parsetree::ExprList::MultiList(_,_)) => assert!(true),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            _ => assert!(false)
        }
        Ok(())
    }
}