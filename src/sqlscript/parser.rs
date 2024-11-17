pub mod parser {
    use crate::sqlscript::types::types::CompressType;
    use super::super::lexer::lexer::*;
    use std::rc::Rc;
    use super::super::types::types;
    use super::super::types::types::{ ColType, SortType };

    macro_rules! handle{
        ($e:expr) => {
            (match $e { Ok(v) => v, Err(s) => return Err(s) })
        }
    }

    macro_rules! perr{
        ($i:ident) => {
            (return Err("Unexpected token at ".to_string() + &($i.lexer.get_pos().to_string()) ))
        }
    }

    pub struct Parser {
        lexer: Lexer,
        token: Token
    }   
    impl Parser {
        // Constructor
        pub fn new(stream: String) -> Parser {
            let lexer = Lexer::new(stream);
            Parser {
                lexer: lexer,
                token: Token { kind: TokenKind::Dot, value: TokenValue::None, start: 0, end: 0 }
            }
        }
        // Program control
        fn peek(&self) -> &Token {
            &self.token
        }
        fn peek_ahead(&mut self) -> Result<Token, String> {
            // Save current position
            let pos_save = self.lexer.get_pos();
            // Produce next token
            let next_token = handle!(self.lexer.produce());
            // Reset lexer
            self.lexer.set_pos(pos_save);
            // Return
            Ok(next_token)
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
                TokenKind::Percent => Some(types::BopType::ModBop),
                _ => None
            }
        }
        fn peek_uop(&self) -> Option<types::UopType> {
            // Peek first token
            let token = self.peek();
            // Match type, return appropriate value
            match token.kind {
                TokenKind::MinusKw => Some(types::UopType::NegUop),
                TokenKind::NotKw => Some(types::UopType::NotUop),
                TokenKind::PlusKw => Some(types::UopType::NumUop),
                TokenKind::Ampersand => Some(types::UopType::StrUop),
                TokenKind::Question => Some(types::UopType::BoolUop),
                TokenKind::Carrot => Some(types::UopType::CeilUop),
                TokenKind::Underscore => Some(types::UopType::FloorUop),
                _ => None
            }
        }
        fn pop(&mut self) -> Result<Token, String> {
            let old_token = self.token.clone();
            match self.lexer.produce() {
                Ok(t) => {
                    self.token = t;
                    Ok(old_token)
                },
                Err(s) => Err(s)
            }
        }
        fn pop_expect(&mut self, kind: TokenKind) -> Result<Token, String> {
            let old_token = self.token.clone();
            if old_token.kind != kind {
                Err("Unexpected token at ".to_string() + &self.lexer.get_pos().to_string())
            } else {
                match self.lexer.produce() {
                    Ok(t) => {
                        self.token = t;
                        Ok(old_token)
                    },
                    Err(s) => Err(s)
                }
            }
        }
        // Parsing entry point
        pub fn parse_script(&mut self) -> Result<types::Block, String> {
            // Reset lexer
            self.lexer.reset();
            // Produce first token
            self.token = match self.lexer.produce() {
                Ok(t) => t,
                Err(s) => return Err(s)
            };
            // Call start symbol
            self.block()
        }
        // Parsing entry point
        pub fn parse(&mut self) -> Result<types::Query, String> {
            // Reset lexer
            self.lexer.reset();
            // Produce first token
            self.token = match self.lexer.produce() {
                Ok(t) => t,
                Err(s) => return Err(s)
            };
            // Call start symbol
            self.query()
        }
        fn query(&mut self) -> Result<types::Query, String> {
            // Match on first item
            match handle!(self.pop()).kind {
                TokenKind::SelectKw => {
                    // Check if select aggregate or regular select
                    match self.peek().kind {
                        TokenKind::AggregateKw => {
                            // Pop aggregate keyword
                            handle!(self.pop());
                            // Parse single ident
                            let agid = handle!(self.ident());
                            // Expect and pop FROM keyword
                            handle!(self.pop_expect(TokenKind::FromKw));
                            // Parse table id
                            let tabid = handle!(self.ident());
                            // Put it together
                            Ok(types::Query::SelectAggregate(agid, tabid))
                        },
                        TokenKind::CompKw => {
                            // Pop aggregate keyword
                            handle!(self.pop());
                            // Parse single ident
                            let cmpid = handle!(self.ident());
                            // Expect and pop FROM keyword
                            handle!(self.pop_expect(TokenKind::FromKw));
                            // Parse table id
                            let tabid = handle!(self.ident());
                            // Put it together
                            Ok(types::Query::SelectComp(cmpid, tabid))
                        },
                        _ => {
                            // Parse identlist
                            let ilist = match self.peek().kind {
                                TokenKind::TimesKw => {
                                    // Pop *
                                    handle!(self.pop());
                                    // Return identlist none
                                    None
                                },
                                _ => Some(handle!(self.identlist()))
                            };
                            // Expect and pop FROM keyword
                            handle!(self.pop_expect(TokenKind::FromKw));
                            // Parse single ident
                            let tableid = handle!(self.ident());
                            // Check if where clause
                            let wherescript = match self.peek().kind {
                                TokenKind::WhereKw => {
                                    // Pop where keyword
                                    handle!(self.pop());
                                    // Parse script
                                    Some(handle!(self.expr()))
                                },
                                _ => None
                            };
                            let sortscript = match self.peek().kind {
                                TokenKind::SortKw => {
                                    // Pop sort keyword
                                    handle!(self.pop());
                                    // Pop expect by keyword
                                    handle!(self.pop_expect(TokenKind::ByKw));
                                    // Parse ident
                                    let ident = handle!(self.ident());
                                    // Check if sort type
                                    match self.peek().kind {
                                        TokenKind::SortType => match handle!(self.pop()).value {
                                            TokenValue::SortType(t) => Some((ident, t)),
                                            _ => perr!(self)
                                        },
                                        _ => Some((ident, SortType::Ascending))
                                    }
                                },
                                _ => None
                            };
                            let limitscript = match self.peek().kind {
                                TokenKind::LimitKw => {
                                    // Pop limit keyword
                                    handle!(self.pop());
                                    // Parse script
                                    Some(handle!(self.expr()))
                                },
                                _ => None
                            };
                            // Put everything together
                            Ok(types::Query::Select(ilist, tableid, wherescript, sortscript, limitscript))
                        }
                    }
                },
                TokenKind::InsertKw => {
                    // Expect and pop INTO
                    handle!(self.pop_expect(TokenKind::IntoKw));
                    // Parse table name
                    let tableid = handle!(self.ident());
                    // Get column ids
                    let colids = match self.peek().kind {
                        TokenKind::LParen => {
                            // Pop lparen
                            handle!(self.pop());
                            // Parse identlist
                            let ilist = handle!(self.identlist());
                            // Expect and pop rparen
                            handle!(self.pop_expect(TokenKind::RParen));
                            // Return ilist 
                            Some(ilist)
                        },
                        _ => None
                    };
                    // Expect and pop VALUES
                    handle!(self.pop_expect(TokenKind::ValuesKw));
                    // Expect and pop LPAREN
                    handle!(self.pop_expect(TokenKind::LParen));
                    // Parse values list
                    let vlist = handle!(self.exprlist());
                    // Expect and pop RPAREN
                    handle!(self.pop_expect(TokenKind::RParen));
                    // Return
                    Ok(types::Query::Insert(tableid, colids, vlist))
                },
                TokenKind::CompressKw => {
                    // Parse table name
                    let table = handle!(self.ident());
                    // Expect LPAREN
                    handle!(self.pop_expect(TokenKind::LParen));
                    // Fields
                    let fields = handle!(self.identlist());
                    // Expect RPAREN
                    handle!(self.pop_expect(TokenKind::RParen));
                    // Check if LPAREN
                    let comptypes = match self.peek().kind {
                        TokenKind::LParen => {
                            // Pop LPAREN
                            handle!(self.pop());
                            // List of compression types
                            let types = handle!(self.compresslist());
                            // Expect RPAREN
                            handle!(self.pop_expect(TokenKind::RParen));
                            // Return types
                            types
                        },
                        _ => {
                            // Expect a single compression type
                            let ctype = handle!(self.compresstype());
                            // Push as many of that type as there are fields
                            let mut clist = Vec::new();
                            for _ in &fields {
                                clist.push(ctype)
                            };
                            // Return types list
                            clist
                        }
                    };
                    Ok(types::Query::Compress(table, fields, comptypes))
                },
                TokenKind::ScriptKw => {
                    // Parse expr
                    let e = types::Expr::BlockExpr(handle!(self.block()));
                    // Check if FROM
                    match self.peek().kind {
                        TokenKind::FromKw => {
                            // Pop FROM
                            handle!(self.pop());
                            // Get table name
                            let tname = handle!(self.ident());
                            // Return
                            Ok(types::Query::Script(e, Some(tname)))
                        },
                        _ => {
                            // Return
                            Ok(types::Query::Script(e, None))
                        }
                    }
                },
                TokenKind::ExitKw => {
                    Ok(types::Query::Exit)
                },
                TokenKind::ImportKw => {
                    // Pop expect CSV
                    handle!(self.pop_expect(TokenKind::CSVKw));
                    // Get CSV name
                    let cpath = handle!(self.ident());
                    // Expect INTO 
                    handle!(self.pop_expect(TokenKind::IntoKw));
                    // Get table name
                    let tname = handle!(self.ident());
                    // Put query together
                    Ok(types::Query::ImportCSV(cpath, tname))
                },
                TokenKind::ExportKw => {
                    // Pop expect CSV
                    handle!(self.pop_expect(TokenKind::CSVKw));
                    // Get CSV name
                    let cpath = handle!(self.ident());
                    // Expect FROM 
                    handle!(self.pop_expect(TokenKind::FromKw));
                    // Get table name
                    let tname = handle!(self.ident());
                    // Put query together
                    Ok(types::Query::ExportCSV(cpath, tname))
                },
                TokenKind::CreateKw => {
                    match handle!(self.pop()).kind {
                        TokenKind::TableKw => {
                            // Read table name
                            let tname = handle!(self.ident());
                            // Expect and pop paren
                            handle!(self.pop_expect(TokenKind::LParen));
                            // Parse column list
                            let clist = handle!(self.collist());
                            // Expect and pop rparen
                            handle!(self.pop_expect(TokenKind::RParen));
                            // Return
                            Ok(types::Query::CreateTable(tname, clist))
                        },
                        TokenKind::CompKw => {
                            // Parse single assignment
                            let assign = handle!(self.singleassign());
                            // Parse INTO
                            handle!(self.pop_expect(TokenKind::IntoKw));
                            // Parse name
                            let table_name = handle!(self.ident());
                            // Put together
                            Ok(types::Query::Comp(assign.0, assign.1, table_name))
                        },
                        TokenKind::ColumnKw => {
                            // Expect LPAREN
                            handle!(self.pop_expect(TokenKind::LParen));
                            // Parse column type
                            let t = handle!(self.parsetype());
                            // Parse compression strategy
                            let s = match self.peek().kind {
                                TokenKind::CompressType => Some(handle!(self.compresstype())),
                                _ => None
                            };
                            // Expect RPAREN
                            handle!(self.pop_expect(TokenKind::RParen));
                            // Parse single assign
                            let assign = handle!(self.singleassign());
                            // Expect and pop INTO
                            handle!(self.pop_expect(TokenKind::IntoKw));
                            // Parse table name
                            let tname = handle!(self.ident());
                            // Put together
                            Ok(types::Query::Column(t, s, assign.0, assign.1, tname))
                        },
                        TokenKind::AggregateKw => {
                            // Parse single equals
                            let assign = handle!(self.singleassign());
                            // Parse INIT
                            let init = match self.peek().kind {
                                TokenKind::InitKw => {
                                    // Pop INIT
                                    handle!(self.pop());
                                    // Parse expr
                                    Some(handle!(self.expr()))
                                },
                                _ => None
                            };
                            // Expect and pop INTO
                            handle!(self.pop_expect(TokenKind::IntoKw));
                            // Parse table name
                            let tname = handle!(self.ident());
                            // Put together
                            Ok(types::Query::Aggregate(assign.0, assign.1, init, tname))
                        },
                        TokenKind::ConstKw => {
                            // Parse single assignment
                            let assign = handle!(self.singleassign());
                            // Put together
                            Ok(types::Query::Const(assign.0, assign.1))
                        },
                        _ => perr!(self)
                    }
                    
                },
                _ => perr!(self)
            }
        }
        // Parsing functions
        fn block(&mut self) -> Result<types::Block, String> {
            match handle!(self.peek_ahead()).kind {
                // If 2nd token is an assignment, parse as statement
                TokenKind::AssignKw => {
                    // Save ident value
                    let ident_val: String = match &self.token.value {
                        TokenValue::String(x) => x.clone(),
                        _ => perr!(self)
                    };
                    // Pop ident and assignment
                    handle!(self.pop());
                    handle!(self.pop());
                    // Parse expr
                    let expr: types::Expr = handle!(self.expr());
                    // Expect semicolon, pop it
                    handle!(self.pop_expect(TokenKind::SemiKw));
                    // Return constructed statement
                    Ok(types::Block::StmtBlock(ident_val, Rc::new(expr), Rc::new(handle!(self.block()))))
                }
                // Parse as expression
                _ => Ok(types::Block::ExprBlock(Rc::new(handle!(self.expr()))))
            }
        }
        fn expr(&mut self) -> Result<types::Expr, String> {
            // Check first value
            let first = match self.peek().kind {
                TokenKind::FunKw => {
                    // Pop fun kw
                    handle!(self.pop());
                    // Get parameter list
                    let paramlist = match self.peek().kind {
                        TokenKind::Arrow => Vec::new(),
                        _ => handle!(self.identlist())
                    };
                    // Expect arrow, pop it
                    handle!(self.pop_expect(TokenKind::Arrow));
                    // Parse body
                    let body = handle!(self.expr());
                    // Put everything together
                    types::Expr::FunExpr(paramlist, Rc::new(body))
                },
                TokenKind::IfKw => {
                    // Pop if kw
                    handle!(self.pop());
                    // Parse conditional
                    let if_expr = handle!(self.expr());
                    // Expect then keyword
                    handle!(self.pop_expect(TokenKind::ThenKw));
                    // Parse then expr
                    let then_expr = handle!(self.expr());
                    // Expect else kw
                    handle!(self.pop_expect(TokenKind::ElseKw));
                    // Parse else expr
                    let else_expr = handle!(self.expr());
                    // Put it all together
                    types::Expr::CondExpr(Rc::new(if_expr), Rc::new(then_expr), Rc::new(else_expr))
                },
                _ => handle!(self.expr_level_2())
            };
            // Check if bop at front
            match self.peek_l1_bop() {
                Some(bop) => {
                    // Pop bop
                    handle!(self.pop());
                    // Return parsed expr
                    Ok(types::Expr::BopExpr(Rc::new(first), bop, Rc::new(handle!(self.expr()))))
                },
                None => Ok(first)
            }
        }
        fn expr_level_2(&mut self) -> Result<types::Expr, String> {
            let first = handle!(self.expr_level_3());
            // Check if bop at front
            match self.peek_l2_bop() {
                Some(bop) => {
                    // Pop bop
                    handle!(self.pop());
                    // Return parsed expr
                    Ok(types::Expr::BopExpr(Rc::new(first), bop, Rc::new(handle!(self.expr_level_2()))))
                },
                None => Ok(first)
            }
        }
        fn expr_level_3(&mut self) -> Result<types::Expr, String> {
            let first = handle!(self.expr_level_4());
            // Check if bop at front
            match self.peek_l3_bop() {
                Some(bop) => {
                    // Pop bop
                    handle!(self.pop());
                    // Return parsed expr
                    Ok(types::Expr::BopExpr(Rc::new(first), bop, Rc::new(handle!(self.expr_level_3()))))
                },
                None => Ok(first)
            }
        }
        fn expr_level_4(&mut self) -> Result<types::Expr, String> {
            let first = handle!(self.expr_level_5());
            // Check if bop at front
            match self.peek_l4_bop() {
                Some(bop) => {
                    // Pop bop
                    handle!(self.pop());
                    // Return parsed expr
                    Ok(types::Expr::BopExpr(Rc::new(first), bop, Rc::new(handle!(self.expr_level_4()))))
                },
                None => Ok(first)
            }
        }
        fn expr_level_5(&mut self) -> Result<types::Expr, String> {
            match self.peek_uop() {
                Some(u) => {
                    // Pop uop
                    handle!(self.pop());
                    // Parse expr
                    let expr = handle!(self.expr_level_5());
                    // Return expression
                    Ok(types::Expr::UopExpr(u, Rc::new(expr)))
                },
                _ => Ok(handle!(self.expr_level_6()))
            }
        }
        fn expr_level_6(&mut self) -> Result<types::Expr, String> {
            let first = handle!(self.expr_level_7());
            // Check if postfix (call only postfix) at front
            match self.peek().kind {
                TokenKind::LParen => {
                    // Pop LParen
                    handle!(self.pop());
                    // Check if next is rparen. If not, parse exprlist
                    let elist = match self.peek().kind {
                        TokenKind::RParen => Vec::new(),
                        _ => handle!(self.exprlist())
                    };
                    // Expect RParen
                    self.pop_expect(TokenKind::RParen);
                    // Construct expression
                    Ok(types::Expr::CallExpr(Rc::new(first), elist))
                },
                _ => Ok(first)
            }
        }
        fn expr_level_7(&mut self) -> Result<types::Expr, String> {
            let first = handle!(self.expr_level_8());
            match self.peek().kind {
                TokenKind::Dot => {
                    // Pop bop
                    handle!(self.pop());
                    // Return parsed expr
                    Ok(types::Expr::BopExpr(Rc::new(first), types::BopType::DotBop, Rc::new(handle!(self.expr_level_7()))))
                },
                _ => Ok(first)
            }
        }
        fn expr_level_8(&mut self) -> Result<types::Expr, String> {
            match self.peek().kind {
                TokenKind::Identifier => Ok(types::Expr::IdentExpr(match handle!(self.pop()).value {
                    TokenValue::String(x) => x.clone(),
                    _ => perr!(self)
                })),
                TokenKind::LCBracket => {
                    // Pop curly bracket
                    handle!(self.pop());
                    // Parse script
                    let script = handle!(self.block());
                    // Expect right curly bracket, pop it
                    self.pop_expect(TokenKind::RCBracket);
                    // Return parsed expression
                    Ok(types::Expr::BlockExpr(script))
                },
                TokenKind::LParen => {
                    // Pop lparen
                    handle!(self.pop());
                    // Parse expr
                    let expr = handle!(self.expr());
                    // Expect rparen, pop it
                    handle!(self.pop_expect(TokenKind::RParen));
                    // Return parsed expression
                    Ok(expr)
                },
                TokenKind::LBracket => {
                    // Pop lbracket
                    handle!(self.pop());
                    // Parse exprlist
                    let exprlist = handle!(self.exprlist());
                    // Expect rbracket, pop it
                    handle!(self.pop_expect(TokenKind::RBracket));
                    // Return parsed expression
                    Ok(types::Expr::TupExpr(exprlist))
                },
                _ => Ok(types::Expr::ValExpr(handle!(self.value())))
            }
        }
        fn value(&mut self) -> Result<types::Val, String> {
            // Pop first token
            let token = handle!(self.pop());
            // Match type, return appropriate value
            match token.value {
                TokenValue::Number(x) => Ok(types::Val::NumVal(x)),
                TokenValue::Boolean(x) => Ok(types::Val::BoolVal(x)),
                // String could be either ident or string value
                TokenValue::String(x) => {
                    match token.kind {
                        TokenKind::String => Ok(types::Val::StrVal(x)),
                        _ => perr!(self)
                    }
                },
                // None could either be undefined or null
                TokenValue::None => {
                    match token.kind {
                        TokenKind::UndefinedKw => Ok(types::Val::UndefVal),
                        TokenKind::NullKw => Ok(types::Val::NullVal),
                        _ => perr!(self)
                    }
                },
                _ => perr!(self)
            }
        }
        fn exprlist(&mut self) -> Result<types::ExprList, String> {
            // Parse expr
            let expr = handle!(self.expr());
            // Check if comma
            match self.peek().kind {
                TokenKind::Comma => {
                    // Pop comma
                    handle!(self.pop());
                    // Get rest of list
                    let mut rest = handle!(self.exprlist_rest());
                    // Parse next expr
                    rest.push(Rc::new(expr));
                    rest.reverse();
                    Ok(rest)
                },
                _ => {
                    let new_vec = vec![Rc::new(expr)];
                    Ok(new_vec)
                }
            }
        }
        fn exprlist_rest(&mut self) -> Result<types::ExprList, String> {
            // Parse expr
            let expr = handle!(self.expr());
            // Check if comma
            match self.peek().kind {
                TokenKind::Comma => {
                    // Pop comma
                    handle!(self.pop());
                    // Get rest of list
                    let mut rest = handle!(self.exprlist_rest());
                    // Parse next expr
                    rest.push(Rc::new(expr));
                    Ok(rest)
                },
                _ => {
                    let new_vec = vec![Rc::new(expr)];
                    Ok(new_vec)
                }
            }
        }
        fn identlist(&mut self) -> Result<types::IdentList, String> {
            // Parse value
            let val = handle!(self.ident());
            // Check if comma
            match self.peek().kind {
                TokenKind::Comma => {
                    // Pop comma
                    handle!(self.pop());
                    // Parse next expr
                    let mut next_vec = handle!(self.identlist_rest());
                    next_vec.push(val);
                    next_vec.reverse();
                    Ok(next_vec)
                },
                _ => {
                    let new_vec = vec![val];
                    Ok(new_vec)
                }
            }
        }
        fn identlist_rest(&mut self) -> Result<types::IdentList, String> {
            // Parse value
            let val = handle!(self.ident());
            // Check if comma
            match self.peek().kind {
                TokenKind::Comma => {
                    // Pop comma
                    handle!(self.pop());
                    // Parse next expr
                    let mut next_vec = handle!(self.identlist_rest());
                    next_vec.push(val);
                    Ok(next_vec)
                },
                _ => {
                    let new_vec = vec![val];
                    Ok(new_vec)
                }
            }
        }
        fn ident(&mut self) -> Result<String, String> {
            // Extract string from ident
            match self.peek().kind {
                TokenKind::Identifier => {
                    match handle!(self.pop()).value {
                        TokenValue::String(x) => Ok(x.clone()),
                        _ => perr!(self)
                    }
                },
                _ => perr!(self)
            }
        }
        fn singleassign(&mut self) -> Result<(String, types::Expr), String> {
            // Parse ident
            let id = handle!(self.ident());
            // Expect and pop = sign
            handle!(self.pop_expect(TokenKind::AssignKw));
            // Parse expr
            let expr = handle!(self.expr());
            // Put together
            Ok((id, expr))
        }
        fn collist(&mut self) -> Result<types::ColList, String> {
            // Parse column name
            let colname = handle!(self.ident());
            // Parse type
            let t = handle!(self.parsetype());
            // Parse compression strategy
            let s = match self.peek().kind {
                TokenKind::CompressType => Some(handle!(self.compresstype())),
                _ => None
            };
            // Check if comma or not
            match self.peek().kind {
                TokenKind::Comma => {
                    // Pop comma
                    handle!(self.pop());
                    // Get rest of list
                    let mut rest = handle!(self.collist_rest());
                    // Add next to rest
                    rest.push((colname, t, s));
                    rest.reverse();
                    Ok(rest)
                },
                _ => Ok(vec![(colname, t, s)])
            }
        }
        fn collist_rest(&mut self) -> Result<types::ColList, String> {
            // Parse column name
            let colname = handle!(self.ident());
            // Parse type
            let t = handle!(self.parsetype());
            // Parse compression strategy
            let s = match self.peek().kind {
                TokenKind::CompressType => Some(handle!(self.compresstype())),
                _ => None
            };
            // Check if comma or not
            match self.peek().kind {
                TokenKind::Comma => {
                    // Pop comma
                    handle!(self.pop());
                    // Get rest of list
                    let mut rest = handle!(self.collist_rest());
                    // Add next to rest
                    rest.push((colname, t, s));
                    Ok(rest)
                },
                _ => Ok(vec![(colname, t, s)])
            }
        }
        fn parsetype(&mut self) -> Result<ColType, String> {
            // Extract type from token
            match handle!(self.pop()).value {
                TokenValue::Type(x) => Ok(x),
                _ => perr!(self)
            }
        }
        fn compresstype(&mut self) -> Result<CompressType, String> {
            // Extract type from token
            match handle!(self.pop()).value {
                TokenValue::CompressionType(x) => Ok(x),
                _ => perr!(self)
            }
        }
        fn compresslist(&mut self) -> Result<types::CompressList, String> {
            // Parse type
            let t = handle!(self.compresstype());
            // Check if comma or not
            match self.peek().kind {
                TokenKind::Comma => {
                    // Pop comma
                    handle!(self.pop());
                    // Get rest of list
                    let mut rest = handle!(self.compresslist_rest());
                    // Add next to rest
                    rest.push(t);
                    rest.reverse();
                    Ok(rest)
                },
                _ => Ok(vec![t])
            }
        }
        fn compresslist_rest(&mut self) -> Result<types::CompressList, String> {
            // Parse type
            let t = handle!(self.compresstype());
            // Check if comma or not
            match self.peek().kind {
                TokenKind::Comma => {
                    // Pop comma
                    handle!(self.pop());
                    // Get rest of list
                    let mut rest = handle!(self.compresslist_rest());
                    // Add next to rest
                    rest.push(t);
                    Ok(rest)
                },
                _ => Ok(vec![t])
            }
        }
    }
}