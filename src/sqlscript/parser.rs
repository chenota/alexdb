pub mod parser {
    use super::super::lexer::lexer::*;
    use std::rc::Rc;
    use super::super::types::types;
    use super::super::types::types::{ ColType, SortType };
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
        fn peek_l1_bop(&self) -> Option<types::BopType> {
            // Peek first token
            let token = self.peek();
            // Match type, return appropriate value
            match token.kind {
                TokenKind::LogAndKw => Some(types::BopType::LogAndBop),
                TokenKind::LogOrKw => Some(types::BopType::LogOrBop),
                _ => None
            }
        }
        fn peek_l2_bop(&self) -> Option<types::BopType> {
            // Peek first token
            let token = self.peek();
            // Match type, return appropriate value
            match token.kind {
                TokenKind::Gt => Some(types::BopType::GtBop),
                TokenKind::Gte => Some(types::BopType::GteBop),
                TokenKind::Lt => Some(types::BopType::LtBop),
                TokenKind::Lte => Some(types::BopType::LteBop),
                TokenKind::Eq => Some(types::BopType::EqBop),
                TokenKind::StrEq => Some(types::BopType::StrEqBop),
                _ => None
            }
        }
        fn peek_l3_bop(&self) -> Option<types::BopType> {
            // Peek first token
            let token = self.peek();
            // Match type, return appropriate value
            match token.kind {
                TokenKind::PlusKw => Some(types::BopType::PlusBop),
                TokenKind::MinusKw => Some(types::BopType::MinusBop),
                _ => None
            }
        }
        fn peek_l4_bop(&self) -> Option<types::BopType> {
            // Peek first token
            let token = self.peek();
            // Match type, return appropriate value
            match token.kind {
                TokenKind::TimesKw => Some(types::BopType::TimesBop),
                TokenKind::DivKw => Some(types::BopType::DivBop),
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
        pub fn parse_script(&mut self) -> types::Block {
            // Reset lexer
            self.lexer.reset();
            // Produce first token
            self.token = self.lexer.produce();
            // Call start symbol (script for now, will eventually be query)
            self.block()
        }
        // Parsing entry point
        pub fn parse(&mut self) -> types::Query {
            // Reset lexer
            self.lexer.reset();
            // Produce first token
            self.token = self.lexer.produce();
            // Call start symbol (script for now, will eventually be query)
            self.query()
        }
        fn query(&mut self) -> types::Query {
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
                            types::Query::SelectAggregate(agid, tabid)
                        },
                        _ => {
                            // Parse identlist
                            let ilist = match self.peek().kind {
                                TokenKind::TimesKw => {
                                    // Pop *
                                    self.pop();
                                    // Return identlist none
                                    None
                                },
                                _ => Some(self.identlist())
                            };
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
                            let sortscript = match self.peek().kind {
                                TokenKind::SortKw => {
                                    // Pop sort keyword
                                    self.pop();
                                    // Pop expect by keyword
                                    self.pop_expect(TokenKind::ByKw);
                                    // Parse ident
                                    let ident = self.ident();
                                    // Check if sort type
                                    match self.peek().kind {
                                        TokenKind::SortType => match self.pop().value {
                                            TokenValue::SortType(t) => Some((ident, t)),
                                            _ => panic!("Parsing error")
                                        },
                                        _ => Some((ident, SortType::Ascending))
                                    }
                                },
                                _ => None
                            };
                            let limitscript = match self.peek().kind {
                                TokenKind::LimitKw => {
                                    // Pop limit keyword
                                    self.pop();
                                    // Parse script
                                    Some(self.expr())
                                },
                                _ => None
                            };
                            // Put everything together
                            types::Query::Select(ilist, tableid, wherescript, sortscript, limitscript)
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
                    types::Query::Insert(tableid, colids, vlist)
                },
                TokenKind::CreateKw => {
                    match self.pop().kind {
                        TokenKind::TableKw => {
                            // Read table name
                            let tname = self.ident();
                            // Expect and pop paren
                            self.pop_expect(TokenKind::LParen);
                            // Parse column list
                            let clist = self.collist();
                            // Expect and pop rparen
                            self.pop_expect(TokenKind::RParen);
                            // Return
                            types::Query::CreateTable(tname, clist)
                        },
                        TokenKind::CompKw => {
                            // Parse single assignment
                            let assign = self.singleassign();
                            // Parse INTO
                            self.pop_expect(TokenKind::IntoKw);
                            // Parse name
                            let table_name = self.ident();
                            // Put together
                            types::Query::Comp(assign.0, assign.1, table_name)
                        },
                        TokenKind::ColumnKw => {
                            // Expect LPAREN
                            self.pop_expect(TokenKind::LParen);
                            // Parse column type
                            let t = self.parsetype();
                            // Expect RPAREN
                            self.pop_expect(TokenKind::RParen);
                            // Parse single assign
                            let assign = self.singleassign();
                            // Expect and pop INTO
                            self.pop_expect(TokenKind::IntoKw);
                            // Parse table name
                            let tname = self.ident();
                            // Put together
                            types::Query::Column(t, assign.0, assign.1, tname)
                        },
                        TokenKind::AggregateKw => {
                            // Parse single equals
                            let assign = self.singleassign();
                            // Parse INIT
                            let init = match self.peek().kind {
                                TokenKind::InitKw => {
                                    // Pop INIT
                                    self.pop();
                                    // Parse expr
                                    Some(self.expr())
                                },
                                _ => None
                            };
                            // Expect and pop INTO
                            self.pop_expect(TokenKind::IntoKw);
                            // Parse table name
                            let tname = self.ident();
                            // Put together
                            types::Query::Aggregate(assign.0, assign.1, init, tname)
                        },
                        TokenKind::ConstKw => {
                            // Parse single assignment
                            let assign = self.singleassign();
                            // Put together
                            types::Query::Const(assign.0, assign.1)
                        },
                        _ => panic!("Parsing error")
                    }
                    
                },
                _ => panic!("Parsing error")
            }
        }
        // Parsing functions
        fn block(&mut self) -> types::Block {
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
                    let expr: types::Expr = self.expr();
                    // Expect semicolon, pop it
                    self.pop_expect(TokenKind::SemiKw);
                    // Return constructed statement
                    types::Block::StmtBlock(ident_val, Rc::new(expr), Rc::new(self.block()))
                }
                // Parse as expression
                _ => types::Block::ExprBlock(Rc::new(self.expr()))
            }
        }
        fn expr(&mut self) -> types::Expr {
            // Check first value
            let first = match self.peek().kind {
                TokenKind::FunKw => {
                    // Pop fun kw
                    self.pop();
                    // Get parameter list
                    let paramlist = match self.peek().kind {
                        TokenKind::Arrow => Vec::new(),
                        _ => self.identlist()
                    };
                    // Expect arrow, pop it
                    self.pop_expect(TokenKind::Arrow);
                    // Parse body
                    let body = self.expr();
                    // Put everything together
                    types::Expr::FunExpr(paramlist, Rc::new(body))
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
                    types::Expr::CondExpr(Rc::new(if_expr), Rc::new(then_expr), Rc::new(else_expr))
                },
                _ => self.expr_level_2()
            };
            // Check if bop at front
            match self.peek_l1_bop() {
                Some(bop) => {
                    // Pop bop
                    self.pop();
                    // Return parsed expr
                    types::Expr::BopExpr(Rc::new(first), bop, Rc::new(self.expr()))
                },
                None => first
            }
        }
        fn expr_level_2(&mut self) -> types::Expr {
            let first = self.expr_level_3();
            // Check if bop at front
            match self.peek_l2_bop() {
                Some(bop) => {
                    // Pop bop
                    self.pop();
                    // Return parsed expr
                    types::Expr::BopExpr(Rc::new(first), bop, Rc::new(self.expr_level_2()))
                },
                None => first
            }
        }
        fn expr_level_3(&mut self) -> types::Expr {
            let first = self.expr_level_4();
            // Check if bop at front
            match self.peek_l3_bop() {
                Some(bop) => {
                    // Pop bop
                    self.pop();
                    // Return parsed expr
                    types::Expr::BopExpr(Rc::new(first), bop, Rc::new(self.expr_level_3()))
                },
                None => first
            }
        }
        fn expr_level_4(&mut self) -> types::Expr {
            let first = self.expr_level_5();
            // Check if bop at front
            match self.peek_l4_bop() {
                Some(bop) => {
                    // Pop bop
                    self.pop();
                    // Return parsed expr
                    types::Expr::BopExpr(Rc::new(first), bop, Rc::new(self.expr_level_4()))
                },
                None => first
            }
        }
        fn expr_level_5(&mut self) -> types::Expr {
            match self.peek().kind {
                TokenKind::MinusKw => {
                    // Pop minus sign
                    self.pop();
                    // Parse expr
                    let expr = self.expr_level_5();
                    // Return negated expression
                    types::Expr::UopExpr(types::UopType::NegUop, Rc::new(expr))
                },
                TokenKind::NotKw => {
                    // Pop not sign
                    self.pop();
                    // Parse expr
                    let expr = self.expr_level_5();
                    // Return negated expression
                    types::Expr::UopExpr(types::UopType::NotUop, Rc::new(expr))
                },
                _ => self.expr_level_6()
            }
        }
        fn expr_level_6(&mut self) -> types::Expr {
            let first = match self.peek().kind {
                TokenKind::Identifier => types::Expr::IdentExpr(match self.pop().value {
                    TokenValue::String(x) => x.clone(),
                    _ => panic!("Parsing error")
                }),
                TokenKind::LCBracket => {
                    // Pop curly bracket
                    self.pop();
                    // Parse script
                    let script = self.block();
                    // Expect right curly bracket, pop it
                    self.pop_expect(TokenKind::RCBracket);
                    // Return parsed expression
                    types::Expr::BlockExpr(script)
                },
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
                _ => types::Expr::ValExpr(self.value())
            };
            // Check if postfix (call only postfix) at front
            match self.peek().kind {
                TokenKind::LParen => {
                    // Pop LParen
                    self.pop();
                    // Check if next is rparen. If not, parse exprlist
                    let elist = match self.peek().kind {
                        TokenKind::RParen => Vec::new(),
                        _ => self.exprlist()
                    };
                    // Expect RParen
                    self.pop_expect(TokenKind::RParen);
                    // Construct expression
                    types::Expr::CallExpr(Rc::new(first), elist)
                },
                _ => first
            }
        }
        fn value(&mut self) -> types::Val {
            // Pop first token
            let token = self.pop();
            // Match type, return appropriate value
            match token.value {
                TokenValue::Number(x) => types::Val::NumVal(x),
                TokenValue::Boolean(x) => types::Val::BoolVal(x),
                // String could be either ident or string value
                TokenValue::String(x) => {
                    match token.kind {
                        TokenKind::String => types::Val::StrVal(x),
                        _ => panic!("Parsing error")
                    }
                },
                // None could either be undefined or null
                TokenValue::None => {
                    match token.kind {
                        TokenKind::UndefinedKw => types::Val::UndefVal,
                        TokenKind::NullKw => types::Val::NullVal,
                        _ => panic!("Parsing error")
                    }
                },
                _ => panic!("Parsing error")
            }
        }
        fn exprlist(&mut self) -> types::ExprList {
            // Parse expr
            let expr = self.expr();
            // Check if comma
            match self.peek().kind {
                TokenKind::Comma => {
                    // Pop comma
                    self.pop();
                    // Get rest of list
                    let mut rest = self.exprlist_rest();
                    // Parse next expr
                    rest.push(Rc::new(expr));
                    rest.reverse();
                    rest
                },
                _ => {
                    let new_vec = vec![Rc::new(expr)];
                    new_vec
                }
            }
        }
        fn exprlist_rest(&mut self) -> types::ExprList {
            // Parse expr
            let expr = self.expr();
            // Check if comma
            match self.peek().kind {
                TokenKind::Comma => {
                    // Pop comma
                    self.pop();
                    // Get rest of list
                    let mut rest = self.exprlist_rest();
                    // Parse next expr
                    rest.push(Rc::new(expr));
                    rest
                },
                _ => {
                    let new_vec = vec![Rc::new(expr)];
                    new_vec
                }
            }
        }
        fn identlist(&mut self) -> types::IdentList {
            // Parse value
            let val = self.ident();
            // Check if comma
            match self.peek().kind {
                TokenKind::Comma => {
                    // Pop comma
                    self.pop();
                    // Parse next expr
                    let mut next_vec = self.identlist_rest();
                    next_vec.push(val);
                    next_vec.reverse();
                    next_vec
                },
                _ => {
                    let new_vec = vec![val];
                    new_vec
                }
            }
        }
        fn identlist_rest(&mut self) -> types::IdentList {
            // Parse value
            let val = self.ident();
            // Check if comma
            match self.peek().kind {
                TokenKind::Comma => {
                    // Pop comma
                    self.pop();
                    // Parse next expr
                    let mut next_vec = self.identlist_rest();
                    next_vec.push(val);
                    next_vec
                },
                _ => {
                    let new_vec = vec![val];
                    new_vec
                }
            }
        }
        fn ident(&mut self) -> String {
            // Extract string from ident
            match self.peek().kind {
                TokenKind::Identifier => {
                    match self.pop().value {
                        TokenValue::String(x) => x.clone(),
                        _ => panic!("Parsing error")
                    }
                },
                _ => panic!("Parsing error")
            }
        }
        fn singleassign(&mut self) -> (String, types::Expr) {
            // Parse ident
            let id = self.ident();
            // Expect and pop = sign
            self.pop_expect(TokenKind::AssignKw);
            // Parse expr
            let expr = self.expr();
            // Put together
            (id, expr)
        }
        fn collist(&mut self) -> types::ColList {
            // Parse column name
            let colname = self.ident();
            // Parse type
            let t = self.parsetype();
            // Check if comma or not
            match self.peek().kind {
                TokenKind::Comma => {
                    // Pop comma
                    self.pop();
                    // Get rest of list
                    let mut rest = self.collist_rest();
                    // Add next to rest
                    rest.push((colname, t));
                    rest.reverse();
                    rest
                },
                _ => vec![(colname, t)]
            }
        }
        fn collist_rest(&mut self) -> types::ColList {
            // Parse column name
            let colname = self.ident();
            // Parse type
            let t = self.parsetype();
            // Check if comma or not
            match self.peek().kind {
                TokenKind::Comma => {
                    // Pop comma
                    self.pop();
                    // Get rest of list
                    let mut rest = self.collist_rest();
                    // Add next to rest
                    rest.push((colname, t));
                    rest
                },
                _ => vec![(colname, t)]
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
    use super::super::types::types;
    #[test]
    fn parser_integer() -> Result<(), String> {
        // Setup
        let test_input: String = "4".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse_script();
        // Assert correct AST
        match ast {
            // Should be just expr
            types::Block::ExprBlock(e1) => {
                match e1.as_ref() {
                    // Should be val expr
                    types::Expr::ValExpr(v) => {
                        // First value should be four
                        match v {
                            types::Val::NumVal(x) => assert_eq!(*x, 4.0),
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
            types::Block::ExprBlock(e1) => {
                match e1.as_ref() {
                    // Should be bop expr
                    types::Expr::BopExpr(v, t, _) => {
                        // Bop type should be plus
                        assert_eq!(*t, types::BopType::PlusBop);
                        // First value should be four
                        match v.as_ref() {
                            types::Expr::ValExpr(y) => {
                                match y {
                                    types::Val::NumVal(x) => assert_eq!(*x, 5.0),
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
            types::Block::ExprBlock(e1) => {
                match e1.as_ref() {
                    // Should be bop expr
                    types::Expr::BopExpr(_, t, v) => {
                        // Bop type should be plus
                        assert_eq!(*t, types::BopType::MinusBop);
                        // First value should be four
                        match v.as_ref() {
                            types::Expr::ValExpr(y) => {
                                match y {
                                    types::Val::NumVal(x) => assert_eq!(*x, 3.0),
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
            types::Block::ExprBlock(e1) => {
                match e1.as_ref() {
                    // Should be val expr
                    types::Expr::ValExpr(v) => {
                        // First value should be four
                        match v {
                            types::Val::NumVal(x) => assert_eq!(*x, 4.0),
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
            types::Block::StmtBlock(id, e1, sc) => {
                // Make sure ID is x
                assert_eq!(id, "x");
                // Check e1
                match e1.as_ref() {
                    // Should be val expr w/ 5
                    types::Expr::ValExpr(v) => {
                        match v {
                            types::Val::NumVal(x) => assert_eq!(*x, 5.0),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
                // Check type of proceeding script
                match sc.as_ref() {
                    types::Block::ExprBlock(_) => assert!(true),
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
            types::Block::StmtBlock(id, _, sc) => {
                // Make sure ID is x
                assert_eq!(id, "x");
                // Check type of proceeding script
                match sc.as_ref() {
                    types::Block::ExprBlock(_) => assert!(true),
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
            types::Block::StmtBlock(_, _, sc1) => {
                // Check type of proceeding script
                match sc1.as_ref() {
                    types::Block::StmtBlock(_, _, sc2) => {
                        match sc2.as_ref() {
                            types::Block::ExprBlock(_) => assert!(true),
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
            types::Block::ExprBlock(e1) => {
                // Check type of proceeding script
                match e1.as_ref() {
                    types::Expr::BopExpr(e11, _, _) => {
                        match e11.as_ref() {
                            types::Expr::BlockExpr(_) => assert!(true),
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
            types::Block::StmtBlock(_, e1, _) => {
                // Check type of proceeding script
                match e1.as_ref() {
                    types::Expr::BlockExpr(_) => assert!(true),
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
            types::Block::ExprBlock(e1) => {
                // Check type of proceeding script
                match e1.as_ref() {
                    types::Expr::UopExpr(t, e2) => {
                        assert_eq!(*t, types::UopType::NegUop);
                        match e2.as_ref() {
                            types::Expr::ValExpr(_) => assert!(true),
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
            types::Block::ExprBlock(e1) => {
                // Check type of proceeding script
                match e1.as_ref() {
                    types::Expr::BopExpr(_, _, e2) => {
                        match e2.as_ref() {
                            types::Expr::UopExpr(t, _) => assert_eq!(*t, types::UopType::NegUop),
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
            types::Block::ExprBlock(e1) => {
                // Check type of proceeding script
                match e1.as_ref() {
                    types::Expr::CallExpr(e1, args) => {
                        assert_eq!(args.len(), 0);
                        match e1.as_ref() {
                            types::Expr::IdentExpr(_) => assert!(true),
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
            types::Block::ExprBlock(e1) => {
                // Check type of proceeding script
                match e1.as_ref() {
                    types::Expr::BopExpr(e2, _, _) => {
                        match e2.as_ref() {
                            types::Expr::CallExpr(e3, args) => {
                                assert_eq!(args.len(), 0);
                                match e3.as_ref() {
                                    types::Expr::IdentExpr(_) => assert!(true),
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
            types::Block::ExprBlock(e1) => {
                // Check type of expr
                match e1.as_ref() {
                    types::Expr::BopExpr(_, _, _) => assert!(true),
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
            types::Block::ExprBlock(e1) => {
                // Check type of expr
                match e1.as_ref() {
                    types::Expr::CallExpr(_, args) => assert_eq!(args.len(), 1),
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
            types::Block::ExprBlock(e1) => {
                // Check type of expr
                match e1.as_ref() {
                    types::Expr::CallExpr(_, args) => assert_eq!(args.len(), 3),
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
            types::Block::ExprBlock(e1) => {
                // Check type of expr
                match e1.as_ref() {
                    types::Expr::FunExpr(params, body) => {
                        assert_eq!(params.len(), 0);
                        match body.as_ref() {
                            types::Expr::ValExpr(_) => assert!(true),
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
            types::Block::ExprBlock(e1) => {
                // Check type of expr
                match e1.as_ref() {
                    types::Expr::FunExpr(params, body) => {
                        assert_eq!(params.len(), 1);
                        match body.as_ref() {
                            types::Expr::IdentExpr(_) => assert!(true),
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
            types::Block::ExprBlock(e1) => {
                // Check type of expr
                match e1.as_ref() {
                    types::Expr::FunExpr(params, body) => {
                        assert_eq!(params.len(), 3);
                        match body.as_ref() {
                            types::Expr::IdentExpr(_) => assert!(true),
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
            types::Block::StmtBlock(_, e1, _) => {
                // Check type of expr
                match e1.as_ref() {
                    types::Expr::FunExpr(params, body) => {
                        assert_eq!(params.len(), 1);
                        match body.as_ref() {
                            types::Expr::IdentExpr(_) => assert!(true),
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
            types::Block::ExprBlock(e1) => {
                // Check type of expr
                match e1.as_ref() {
                    types::Expr::CondExpr(_, _, _) => assert!(true),
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
            types::Block::StmtBlock(_, e1, _) => {
                // Check type of expr
                match e1.as_ref() {
                    types::Expr::FunExpr(_, body) => {
                        match body.as_ref() {
                            types::Expr::CondExpr(_, _, _) => assert!(true),
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
            types::Query::Select(_, _, _, _, _) => assert!(true),
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
            types::Query::Select(_, _, _, _, _) => assert!(true),
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
            types::Query::Insert(_, _, _) => assert!(true),
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
            types::Query::Insert(_, _, _) => assert!(true),
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
            types::Query::SelectAggregate(_, _,) => assert!(true),
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
            types::Query::Const(_, _,) => assert!(true),
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
            types::Query::Aggregate(_, _, _, _) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_query_col() -> Result<(), String> {
        // Setup
        let test_input: String = "COLUMN (num) awesome = max(field1, field2) INTO table".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse();
        // Assert correct AST
        match ast {
            // Should be exprscript
            types::Query::Column(_, _, _, _) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_query_create() -> Result<(), String> {
        // Setup
        let test_input: String = "CREATE TABLE people (age num, name str, height num)".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse();
        // Assert correct AST
        match ast {
            // Should be exprscript
            types::Query::CreateTable(name, _) => assert_eq!(name, "people"),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_select_all() -> Result<(), String> {
        // Setup
        let test_input: String = "SELECT * FROM table1".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse();
        // Assert correct AST
        match ast {
            // Should be exprscript
            types::Query::Select(ids, _, _, _, _) => {
                match ids {
                    None => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_select_limit_1() -> Result<(), String> {
        // Setup
        let test_input: String = "SELECT * FROM table1 WHERE x > 10 LIMIT 10".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse();
        // Assert correct AST
        match ast {
            // Should be exprscript
            types::Query::Select(_ , _, whr, _, lim) => {
                match lim {
                    Some(_) => assert!(true),
                    _ => assert!(false)
                };
                match whr {
                    Some(_) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_select_limit_2() -> Result<(), String> {
        // Setup
        let test_input: String = "SELECT * FROM table1 LIMIT 10".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse();
        // Assert correct AST
        match ast {
            // Should be exprscript
            types::Query::Select(_ , _, whr, _, lim) => {
                match lim {
                    Some(_) => assert!(true),
                    _ => assert!(false)
                };
                match whr {
                    None => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_select_sort_1() -> Result<(), String> {
        // Setup
        let test_input: String = "SELECT * FROM table1 ORDER BY test2 LIMIT 10".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse();
        // Assert correct AST
        match ast {
            // Should be exprscript
            types::Query::Select(_ , _, whr, srt, lim) => {
                match lim {
                    Some(_) => assert!(true),
                    _ => assert!(false)
                };
                match whr {
                    None => assert!(true),
                    _ => assert!(false)
                };
                match srt {
                    Some(s) => assert_eq!(s.0, "test2"),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_select_sort_2() -> Result<(), String> {
        // Setup
        let test_input: String = "SELECT * FROM table1 ORDER BY test1".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse();
        // Assert correct AST
        match ast {
            // Should be exprscript
            types::Query::Select(_ , _, _, srt, _) => {
                match srt {
                    Some(s) => assert_eq!(s.0, "test1"),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn parser_select_sort_3() -> Result<(), String> {
        // Setup
        let test_input: String = "SELECT * FROM table1 WHERE x > 5 ORDER BY x LIMIT 10".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse();
        // Assert correct AST
        match ast {
            // Should be exprscript
            types::Query::Select(_ , _, _, srt, _) => {
                match srt {
                    Some(s) => assert_eq!(s.0, "x"),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
}