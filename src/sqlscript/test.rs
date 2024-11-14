#[cfg(test)]
mod lexer_tests {
    use super::super::lexer::*;
    #[test]
    fn produce_first() -> Result<(), String> {
        // Setup
        let test_input: String = "4 + 5".to_string();
        let mut test_lexer: lexer::Lexer = lexer::Lexer::new(test_input);
        let next_token = test_lexer.produce();
        // Assert token
        assert_eq!(next_token.kind, lexer::TokenKind::Number);
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
            lexer::TokenValue::Number(x) => assert_eq!(x, 432.0),
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
            lexer::TokenValue::Number(x) => assert_eq!(x, 15.0),
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
        assert_eq!(next_token.kind, lexer::TokenKind::Number);
        Ok(())
    }
    #[test]
    fn lexer_sort_type() -> Result<(), String> {
        // Setup
        let test_input: String = "ASC".to_string();
        let mut test_lexer: lexer::Lexer = lexer::Lexer::new(test_input);
        let next_token = test_lexer.produce();
        // Assert token
        assert_eq!(next_token.kind, lexer::TokenKind::SortType);
        Ok(())
    }
}

#[cfg(test)]
mod parser_tests {
    use super::super::parser::parser::*;
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
        let test_input: String = "CREATE CONST max = fun x, y -> if x > y then x else y".to_string();
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
        let test_input: String = "CREATE AGGREGATE maxval = max(field1, current) INTO table".to_string();
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
        let test_input: String = "CREATE COLUMN (num) awesome = max(field1, field2) INTO table".to_string();
        let mut test_parser: Parser = Parser::new(test_input);
        let ast = test_parser.parse();
        // Assert correct AST
        match ast {
            // Should be exprscript
            types::Query::Column(_, _, _, _, _) => assert!(true),
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