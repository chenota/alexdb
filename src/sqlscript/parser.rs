pub mod parser {
    use super::super::lexer::lexer::*;
    use std::rc::Rc;
    pub mod parsetree {
        use std::rc::Rc;
        use super::super::super::lexer::lexer::ColType;
        pub enum Query {
            Select(IdentList, String, Option<Expr>), // SELECT _ FROM _ WHERE _ (where is optional)
            Insert(String, Option<IdentList>, ExprList), // INSERT INTO _ (_, _, _)? VALUES (_, _, _)
            SelectAggregate(String, String), // SELECT AGGREGATE <name> FROM <table>
            Const(String, Expr), // CONST <name> = <value>
            Aggregate(String, Expr, String), // AGGREGATE <name> = <value> INTO <table>
            Column(String, Expr, String), // COLUMN <name> = <value> INTO <table>
            CreateTable(String, ColList), // CREATE TABLE <name> (col1 type1, col2type2, ...)
        }
        pub enum Expr {
            BopExpr(Rc<Expr>, BopType, Rc<Expr>),
            UopExpr(UopType, Rc<Expr>),
            BlockExpr(Block),
            ValExpr(Val),
            CallExpr(Rc<Expr>, Option<ExprList>),
            FunExpr(Option<IdentList>, Rc<Expr>),
            CondExpr(Rc<Expr>, Rc<Expr>, Rc<Expr>) // if _ then _ else _
        }
        pub enum Block {
            ExprBlock(Rc<Expr>),
            StmtBlock(String, Rc<Expr>, Rc<Block>) // ident = expr; ...
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
        pub enum IdentList {
            MultiList(String, Rc<IdentList>),
            SingleList(String)
        }
        #[derive(PartialEq, Debug)]
        pub enum BopType {
            PlusBop,
            MinusBop,
            TimesBop,
            DivBop,
            GtBop,
            GteBop,
            LtBop,
            LteBop,
            EqBop
        }
        pub enum ColList {
            MultiList(String, ColType, Rc<ColList>),
            SingleList(String, ColType),
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
                TokenKind::Gt => Some(parsetree::BopType::GtBop),
                TokenKind::Gte => Some(parsetree::BopType::GteBop),
                TokenKind::Lt => Some(parsetree::BopType::LtBop),
                TokenKind::Lte => Some(parsetree::BopType::LteBop),
                TokenKind::Eq => Some(parsetree::BopType::EqBop),
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
        pub fn parse_script(&mut self) -> parsetree::Block {
            // Reset lexer
            self.lexer.reset();
            // Produce first token
            self.token = self.lexer.produce();
            // Call start symbol (script for now, will eventually be query)
            self.block()
        }
        // Parsing entry point
        pub fn parse(&mut self) -> parsetree::Query {
            // Reset lexer
            self.lexer.reset();
            // Produce first token
            self.token = self.lexer.produce();
            // Call start symbol (script for now, will eventually be query)
            self.query()
        }
        fn query(&mut self) -> parsetree::Query {
            // Match on first item
            match self.pop().kind {
                TokenKind::SelectKw => {
                    // Check if select aggregate or regular select
                    match self.peek().kind {
                        TokenKind::AggregateKw => {
                            // Pop aggregate keyword
                            self.pop();
                            // Parse single ident
                            let agid = self.ident();
                            // Expect and pop FROM keyword
                            self.pop_expect(TokenKind::FromKw);
                            // Parse table id
                            let tabid = self.ident();
                            // Put it together
                            parsetree::Query::SelectAggregate(agid, tabid)
                        },
                        _ => {
                            // Parse identlist
                            let ilist = self.identlist();
                            // Expect and pop FROM keyword
                            self.pop_expect(TokenKind::FromKw);
                            // Parse single ident
                            let tableid = self.ident();
                            // Check if where clause
                            let wherescript = match self.peek().kind {
                                TokenKind::WhereKw => {
                                    // Pop where keyword
                                    self.pop();
                                    // Parse script
                                    Some(self.expr())
                                },
                                _ => None
                            };
                            // Put everything together
                            parsetree::Query::Select(ilist, tableid, wherescript)
                        }
                    }
                },
                TokenKind::InsertKw => {
                    // Expect and pop INTO
                    self.pop_expect(TokenKind::IntoKw);
                    // Parse table name
                    let tableid = self.ident();
                    // Get column ids
                    let colids = match self.peek().kind {
                        TokenKind::LParen => {
                            // Pop lparen
                            self.pop();
                            // Parse identlist
                            let ilist = self.identlist();
                            // Expect and pop rparen
                            self.pop_expect(TokenKind::RParen);
                            // Return ilist 
                            Some(ilist)
                        },
                        _ => None
                    };
                    // Expect and pop VALUES
                    self.pop_expect(TokenKind::ValuesKw);
                    // Expect and pop LPAREN
                    self.pop_expect(TokenKind::LParen);
                    // Parse values list
                    let vlist = self.exprlist();
                    // Expect and pop RPAREN
                    self.pop_expect(TokenKind::RParen);
                    // Return
                    parsetree::Query::Insert(tableid, colids, vlist)
                },
                TokenKind::ConstKw => {
                    // Parse single assignment
                    let assign = self.singleassign();
                    // Put together
                    parsetree::Query::Const(assign.0, assign.1)
                },
                TokenKind::AggregateKw => {
                    // Parse single equals
                    let assign = self.singleassign();
                    // Expect and pop INTO
                    self.pop_expect(TokenKind::IntoKw);
                    // Parse table name
                    let tname = self.ident();
                    // Put together
                    parsetree::Query::Aggregate(assign.0, assign.1, tname)
                },
                TokenKind::ColumnKw => {
                    // Parse single assign
                    let assign = self.singleassign();
                    // Expect and pop INTO
                    self.pop_expect(TokenKind::IntoKw);
                    // Parse table name
                    let tname = self.ident();
                    // Put together
                    parsetree::Query::Column(assign.0, assign.1, tname)
                },
                TokenKind::CreateKw => {
                    // Pop and expect table kw
                    self.pop_expect(TokenKind::TableKw);
                    // Read table name
                    let tname = self.ident();
                    // Expect and pop paren
                    self.pop_expect(TokenKind::LParen);
                    // Parse column list
                    let clist = self.collist();
                    // Expect and pop rparen
                    self.pop_expect(TokenKind::RParen);
                    // Return
                    parsetree::Query::CreateTable(tname, clist)
                },
                _ => panic!("Parsing error")
            }
        }
        // Parsing functions
        fn block(&mut self) -> parsetree::Block {
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
                    parsetree::Block::StmtBlock(ident_val, Rc::new(expr), Rc::new(self.block()))
                }
                // Parse as expression
                _ => parsetree::Block::ExprBlock(Rc::new(self.expr()))
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
                    let script = self.block();
                    // Expect right curly bracket, pop it
                    self.pop_expect(TokenKind::RCBracket);
                    // Return parsed expression
                    parsetree::Expr::BlockExpr(script)
                },
                TokenKind::MinusKw => {
                    // Pop minus sign
                    self.pop();
                    // Parse expr
                    let expr = self.expr();
                    // Return negated expression
                    parsetree::Expr::UopExpr(parsetree::UopType::NegUop, Rc::new(expr))
                },
                TokenKind::FunKw => {
                    // Pop fun kw
                    self.pop();
                    // Get parameter list
                    let paramlist = match self.peek().kind {
                        TokenKind::Arrow => None,
                        _ => Some(self.identlist())
                    };
                    // Expect arrow, pop it
                    self.pop_expect(TokenKind::Arrow);
                    // Parse body
                    let body = self.expr();
                    // Put everything together
                    parsetree::Expr::FunExpr(paramlist, Rc::new(body))
                },
                TokenKind::IfKw => {
                    // Pop if kw
                    self.pop();
                    // Parse conditional
                    let if_expr = self.expr();
                    // Expect then keyword
                    self.pop_expect(TokenKind::ThenKw);
                    // Parse then expr
                    let then_expr = self.expr();
                    // Expect else kw
                    self.pop_expect(TokenKind::ElseKw);
                    // Parse else expr
                    let else_expr = self.expr();
                    // Put it all together
                    parsetree::Expr::CondExpr(Rc::new(if_expr), Rc::new(then_expr), Rc::new(else_expr))
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
        fn identlist(&mut self) -> parsetree::IdentList {
            // Parse value
            let val = self.ident();
            // Check if comma
            match self.peek().kind {
                TokenKind::Comma => {
                    // Pop comma
                    self.pop();
                    // Parse next expr
                    parsetree::IdentList::MultiList(val, Rc::new(self.identlist()))
                },
                _ => parsetree::IdentList::SingleList(val)
            }
        }
        fn ident(&mut self) -> String {
            // Parse value
            let val = self.value();
            // Extract string from ident
            match val {
                parsetree::Val::IdentVal(x) => x,
                _ => panic!("Parsing error")
            }
        }
        fn singleassign(&mut self) -> (String, parsetree::Expr) {
            // Parse ident
            let id = self.ident();
            // Expect and pop = sign
            self.pop_expect(TokenKind::AssignKw);
            // Parse expr
            let expr = self.expr();
            // Put together
            (id, expr)
        }
        fn collist(&mut self) -> parsetree::ColList {
            // Parse column name
            let colname = self.ident();
            // Parse type
            let t = self.parsetype();
            // Check if comma or not
            match self.peek().kind {
                TokenKind::Comma => {
                    // Pop comma
                    self.pop();
                    // Parse next pair
                    parsetree::ColList::MultiList(colname, t, Rc::new(self.collist()))
                },
                _ => parsetree::ColList::SingleList(colname, t)
            }
        }
        fn parsetype(&mut self) -> ColType {
            // Extract type from token
            match self.pop().value {
                TokenValue::Type(x) => x,
                _ => panic!("Parsing error")
            }
        }
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
            parsetree::Block::ExprBlock(e1) => {
                match e1.as_ref() {
                    // Should be val expr
                    parsetree::Expr::ValExpr(v) => {
                        // First value should be four
                        match v {
                            parsetree::Val::IntVal(x) => assert_eq!(*x, 4),
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
            parsetree::Block::ExprBlock(e1) => {
                match e1.as_ref() {
                    // Should be bop expr
                    parsetree::Expr::BopExpr(v, t, _) => {
                        // Bop type should be plus
                        assert_eq!(*t, parsetree::BopType::PlusBop);
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
            parsetree::Block::ExprBlock(e1) => {
                match e1.as_ref() {
                    // Should be bop expr
                    parsetree::Expr::BopExpr(_, t, v) => {
                        // Bop type should be plus
                        assert_eq!(*t, parsetree::BopType::MinusBop);
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
            parsetree::Block::ExprBlock(e1) => {
                match e1.as_ref() {
                    // Should be val expr
                    parsetree::Expr::ValExpr(v) => {
                        // First value should be four
                        match v {
                            parsetree::Val::IntVal(x) => assert_eq!(*x, 4),
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
            parsetree::Block::StmtBlock(id, e1, sc) => {
                // Make sure ID is x
                assert_eq!(id, "x");
                // Check e1
                match e1.as_ref() {
                    // Should be val expr w/ 5
                    parsetree::Expr::ValExpr(v) => {
                        match v {
                            parsetree::Val::IntVal(x) => assert_eq!(*x, 5),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
                // Check type of proceeding script
                match sc.as_ref() {
                    parsetree::Block::ExprBlock(_) => assert!(true),
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
            parsetree::Block::StmtBlock(id, _, sc) => {
                // Make sure ID is x
                assert_eq!(id, "x");
                // Check type of proceeding script
                match sc.as_ref() {
                    parsetree::Block::ExprBlock(_) => assert!(true),
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
            parsetree::Block::StmtBlock(_, _, sc1) => {
                // Check type of proceeding script
                match sc1.as_ref() {
                    parsetree::Block::StmtBlock(_, _, sc2) => {
                        match sc2.as_ref() {
                            parsetree::Block::ExprBlock(_) => assert!(true),
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
            parsetree::Block::ExprBlock(e1) => {
                // Check type of proceeding script
                match e1.as_ref() {
                    parsetree::Expr::BopExpr(e11, _, _) => {
                        match e11.as_ref() {
                            parsetree::Expr::BlockExpr(_) => assert!(true),
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
            parsetree::Block::StmtBlock(_, e1, _) => {
                // Check type of proceeding script
                match e1.as_ref() {
                    parsetree::Expr::BlockExpr(_) => assert!(true),
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
            parsetree::Block::ExprBlock(e1) => {
                // Check type of proceeding script
                match e1.as_ref() {
                    parsetree::Expr::UopExpr(t, e2) => {
                        assert_eq!(*t, parsetree::UopType::NegUop);
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
            parsetree::Block::ExprBlock(e1) => {
                // Check type of proceeding script
                match e1.as_ref() {
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
            parsetree::Block::ExprBlock(e1) => {
                // Check type of proceeding script
                match e1.as_ref() {
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
            parsetree::Block::ExprBlock(e1) => {
                // Check type of proceeding script
                match e1.as_ref() {
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
            parsetree::Block::ExprBlock(e1) => {
                // Check type of expr
                match e1.as_ref() {
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
            parsetree::Block::ExprBlock(e1) => {
                // Check type of expr
                match e1.as_ref() {
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
            parsetree::Block::ExprBlock(e1) => {
                // Check type of expr
                match e1.as_ref() {
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
    #[test]
    fn parser_fun_expr_empty() -> Result<(), String> {
        // Setup
        let test_input: String = "fun -> 0.0".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse_script();
        // Assert correct AST
        match ast {
            // Should be exprscript
            parsetree::Block::ExprBlock(e1) => {
                // Check type of expr
                match e1.as_ref() {
                    parsetree::Expr::FunExpr(params, body) => {
                        match params {
                            None => assert!(true),
                            _ => assert!(false)
                        }
                        match body.as_ref() {
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
    fn parser_fun_expr_one() -> Result<(), String> {
        // Setup
        let test_input: String = "fun x -> x".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse_script();
        // Assert correct AST
        match ast {
            // Should be exprscript
            parsetree::Block::ExprBlock(e1) => {
                // Check type of expr
                match e1.as_ref() {
                    parsetree::Expr::FunExpr(params, body) => {
                        match params {
                            Some(elist) => {
                                match elist {
                                    parsetree::IdentList::SingleList(_) => assert!(true),
                                    _ => assert!(false)
                                }
                            },
                            _ => assert!(false)
                        }
                        match body.as_ref() {
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
    fn parser_fun_expr_multi() -> Result<(), String> {
        // Setup
        let test_input: String = "fun x,y,z -> x".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse_script();
        // Assert correct AST
        match ast {
            // Should be exprscript
            parsetree::Block::ExprBlock(e1) => {
                // Check type of expr
                match e1.as_ref() {
                    parsetree::Expr::FunExpr(params, body) => {
                        match params {
                            Some(elist) => {
                                match elist {
                                    parsetree::IdentList::MultiList(_,_) => assert!(true),
                                    _ => assert!(false)
                                }
                            },
                            _ => assert!(false)
                        }
                        match body.as_ref() {
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
    fn parser_fun_expr_stmt() -> Result<(), String> {
        // Setup
        let test_input: String = "f = fun x -> x; f(5)".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse_script();
        // Assert correct AST
        match ast {
            // Should be exprscript
            parsetree::Block::StmtBlock(_, e1, _) => {
                // Check type of expr
                match e1.as_ref() {
                    parsetree::Expr::FunExpr(params, body) => {
                        match params {
                            Some(elist) => {
                                match elist {
                                    parsetree::IdentList::SingleList(_) => assert!(true),
                                    _ => assert!(false)
                                }
                            },
                            _ => assert!(false)
                        }
                        match body.as_ref() {
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
    fn parser_cond_expr_1() -> Result<(), String> {
        // Setup
        let test_input: String = "if true then 1 else 0".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse_script();
        // Assert correct AST
        match ast {
            // Should be exprscript
            parsetree::Block::ExprBlock(e1) => {
                // Check type of expr
                match e1.as_ref() {
                    parsetree::Expr::CondExpr(_, _, _) => assert!(true),
                    _ => assert!(false)
                }
            }
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_cond_expr_in_fun() -> Result<(), String> {
        // Setup
        let test_input: String = "max = fun x,y -> if x > y then x else y; max(15, 10)".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse_script();
        // Assert correct AST
        match ast {
            // Should be exprscript
            parsetree::Block::StmtBlock(_, e1, _) => {
                // Check type of expr
                match e1.as_ref() {
                    parsetree::Expr::FunExpr(_, body) => {
                        match body.as_ref() {
                            parsetree::Expr::CondExpr(_, _, _) => assert!(true),
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
    fn parser_query_select_1() -> Result<(), String> {
        // Setup
        let test_input: String = "SELECT field1, field2 FROM tablename".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse();
        // Assert correct AST
        match ast {
            // Should be exprscript
            parsetree::Query::Select(_, _, _) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_query_select_2() -> Result<(), String> {
        // Setup
        let test_input: String = "SELECT field2 FROM tablename WHERE field1 > 100".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse();
        // Assert correct AST
        match ast {
            // Should be exprscript
            parsetree::Query::Select(_, _, _) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_query_insert_1() -> Result<(), String> {
        // Setup
        let test_input: String = "INSERT INTO table VALUES (1,1+1,3)".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse();
        // Assert correct AST
        match ast {
            // Should be exprscript
            parsetree::Query::Insert(_, _, _) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_query_insert_2() -> Result<(), String> {
        // Setup
        let test_input: String = "INSERT INTO table (field1, field2, field3) VALUES (1,2,3)".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse();
        // Assert correct AST
        match ast {
            // Should be exprscript
            parsetree::Query::Insert(_, _, _) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_query_selectagg() -> Result<(), String> {
        // Setup
        let test_input: String = "SELECT AGGREGATE ag1 FROM table".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse();
        // Assert correct AST
        match ast {
            // Should be exprscript
            parsetree::Query::SelectAggregate(_, _,) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_query_const() -> Result<(), String> {
        // Setup
        let test_input: String = "CONST max = fun x, y -> if x > y then x else y".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse();
        // Assert correct AST
        match ast {
            // Should be exprscript
            parsetree::Query::Const(_, _,) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_query_aggregate() -> Result<(), String> {
        // Setup
        let test_input: String = "AGGREGATE maxval = max(field1, current) INTO table".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse();
        // Assert correct AST
        match ast {
            // Should be exprscript
            parsetree::Query::Aggregate(_, _, _) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_query_col() -> Result<(), String> {
        // Setup
        let test_input: String = "COLUMN awesome = max(field1, field2) INTO table".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse();
        // Assert correct AST
        match ast {
            // Should be exprscript
            parsetree::Query::Column(_, _, _) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_query_create() -> Result<(), String> {
        // Setup
        let test_input: String = "CREATE TABLE people (age int, name str, height float)".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse();
        // Assert correct AST
        match ast {
            // Should be exprscript
            parsetree::Query::CreateTable(name, _) => assert_eq!(name, "people"),
            _ => assert!(false)
        }
        Ok(())
    }
}