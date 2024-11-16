pub mod parser {
    use crate::sqlscript::types::types::CompressType;

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
                        TokenKind::CompKw => {
                            // Pop aggregate keyword
                            self.pop();
                            // Parse single ident
                            let cmpid = self.ident();
                            // Expect and pop FROM keyword
                            self.pop_expect(TokenKind::FromKw);
                            // Parse table id
                            let tabid = self.ident();
                            // Put it together
                            types::Query::SelectComp(cmpid, tabid)
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
                TokenKind::CompressKw => {
                    // Parse table name
                    let table = self.ident();
                    // Expect LPAREN
                    self.pop_expect(TokenKind::LParen);
                    // Fields
                    let fields = self.identlist();
                    // Expect RPAREN
                    self.pop_expect(TokenKind::RParen);
                    // Check if LPAREN
                    let comptypes = match self.peek().kind {
                        TokenKind::LParen => {
                            // Pop LPAREN
                            self.pop();
                            // List of compression types
                            let types = self.compresslist();
                            // Expect RPAREN
                            self.pop_expect(TokenKind::RParen);
                            // Return types
                            types
                        },
                        _ => {
                            // Expect a single compression type
                            let ctype = self.compresstype();
                            // Push as many of that type as there are fields
                            let mut clist = Vec::new();
                            for _ in &fields {
                                clist.push(ctype)
                            };
                            // Return types list
                            clist
                        }
                    };
                    types::Query::Compress(table, fields, comptypes)
                }
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
                            // Parse compression strategy
                            let s = match self.peek().kind {
                                TokenKind::CompressType => Some(self.compresstype()),
                                _ => None
                            };
                            // Expect RPAREN
                            self.pop_expect(TokenKind::RParen);
                            // Parse single assign
                            let assign = self.singleassign();
                            // Expect and pop INTO
                            self.pop_expect(TokenKind::IntoKw);
                            // Parse table name
                            let tname = self.ident();
                            // Put together
                            types::Query::Column(t, s, assign.0, assign.1, tname)
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
            match self.peek_uop() {
                Some(u) => {
                    // Pop uop
                    self.pop();
                    // Parse expr
                    let expr = self.expr_level_5();
                    // Return expression
                    types::Expr::UopExpr(u, Rc::new(expr))
                },
                _ => self.expr_level_6()
            }
        }
        fn expr_level_6(&mut self) -> types::Expr {
            let first = self.expr_level_7();
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
        fn expr_level_7(&mut self) -> types::Expr {
            let first = self.expr_level_8();
            match self.peek().kind {
                TokenKind::Dot => {
                    // Pop bop
                    self.pop();
                    // Return parsed expr
                    types::Expr::BopExpr(Rc::new(first), types::BopType::DotBop, Rc::new(self.expr_level_7()))
                },
                _ => first
            }
        }
        fn expr_level_8(&mut self) -> types::Expr {
            match self.peek().kind {
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
                TokenKind::LBracket => {
                    // Pop lbracket
                    self.pop();
                    // Parse exprlist
                    let exprlist = self.exprlist();
                    // Expect rbracket, pop it
                    self.pop_expect(TokenKind::RBracket);
                    // Return parsed expression
                    types::Expr::TupExpr(exprlist)
                },
                _ => types::Expr::ValExpr(self.value())
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
            // Parse compression strategy
            let s = match self.peek().kind {
                TokenKind::CompressType => Some(self.compresstype()),
                _ => None
            };
            // Check if comma or not
            match self.peek().kind {
                TokenKind::Comma => {
                    // Pop comma
                    self.pop();
                    // Get rest of list
                    let mut rest = self.collist_rest();
                    // Add next to rest
                    rest.push((colname, t, s));
                    rest.reverse();
                    rest
                },
                _ => vec![(colname, t, s)]
            }
        }
        fn collist_rest(&mut self) -> types::ColList {
            // Parse column name
            let colname = self.ident();
            // Parse type
            let t = self.parsetype();
            // Parse compression strategy
            let s = match self.peek().kind {
                TokenKind::CompressType => Some(self.compresstype()),
                _ => None
            };
            // Check if comma or not
            match self.peek().kind {
                TokenKind::Comma => {
                    // Pop comma
                    self.pop();
                    // Get rest of list
                    let mut rest = self.collist_rest();
                    // Add next to rest
                    rest.push((colname, t, s));
                    rest
                },
                _ => vec![(colname, t, s)]
            }
        }
        fn parsetype(&mut self) -> ColType {
            // Extract type from token
            match self.pop().value {
                TokenValue::Type(x) => x,
                _ => panic!("Parsing error")
            }
        }
        fn compresstype(&mut self) -> CompressType {
            // Extract type from token
            match self.pop().value {
                TokenValue::CompressionType(x) => x,
                _ => panic!("Parsing error")
            }
        }
        fn compresslist(&mut self) -> types::CompressList {
            // Parse type
            let t = self.compresstype();
            // Check if comma or not
            match self.peek().kind {
                TokenKind::Comma => {
                    // Pop comma
                    self.pop();
                    // Get rest of list
                    let mut rest = self.compresslist_rest();
                    // Add next to rest
                    rest.push(t);
                    rest.reverse();
                    rest
                },
                _ => vec![t]
            }
        }
        fn compresslist_rest(&mut self) -> types::CompressList {
            // Parse type
            let t = self.compresstype();
            // Check if comma or not
            match self.peek().kind {
                TokenKind::Comma => {
                    // Pop comma
                    self.pop();
                    // Get rest of list
                    let mut rest = self.compresslist_rest();
                    // Add next to rest
                    rest.push(t);
                    rest
                },
                _ => vec![t]
            }
        }
    }
}