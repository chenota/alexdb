#[cfg(test)]
mod test_script {
    use crate::sqlscript::parser::parser::Parser;
    use super::super::script::engine::*;
    use super::super::script::env::*;
    use crate::sqlscript::types::types;
    #[test]
    fn basic_addition() -> Result<(), String> {
        // Setup
        let test_input: String = "432 + 5".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(437.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn basic_ident() -> Result<(), String> {
        // Setup
        let test_input: String = "x".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        test_environment.push(&"x".to_string(), &types::Val::NumVal(5.0));
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(5.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn basic_stmt() -> Result<(), String> {
        // Setup
        let test_input: String = "x = 1; x".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(1.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn double_stmt() -> Result<(), String> {
        // Setup
        let test_input: String = "x = 1; y = 2; x + y".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(3.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn triple_stmt() -> Result<(), String> {
        // Setup
        let test_input: String = "x = 1; y = 2; z = 3; x + y + z".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(6.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn str_add() -> Result<(), String> {
        // Setup
        let test_input: String = "'Hello ' + 'World'".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::StrVal(s) => assert_eq!(s, "Hello World"),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn basic_parens() -> Result<(), String> {
        // Setup
        let test_input: String = "3 * (5 + 6)".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(33.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn basic_fn() -> Result<(), String> {
        // Setup
        let test_input: String = "f = fun x -> x; f(5)".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(5.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn empty_fn() -> Result<(), String> {
        // Setup
        let test_input: String = "f = fun -> undefined; f()".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::UndefVal => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn undefined() -> Result<(), String> {
        // Setup
        let test_input: String = "undefined".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::UndefVal => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn null() -> Result<(), String> {
        // Setup
        let test_input: String = "null".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NullVal => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn fn_multi_params() -> Result<(), String> {
        // Setup
        let test_input: String = "f = fun x, y, z -> x + y + z; f(3, 10, 7)".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(20.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn fn_higher_order() -> Result<(), String> {
        // Setup
        let test_input: String = "add = fun x -> fun y -> x + y; inc = add(1); inc(9)".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(10.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn fn_lexical_scope() -> Result<(), String> {
        // Setup
        let test_input: String = "add = fun x -> fun y -> x + y; inc = add(1); x = 1000; inc(9)".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(10.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn fn_lexical_scope_2() -> Result<(), String> {
        // Setup
        let test_input: String = "x = 5; f = {x = 10; fun -> x}; f()".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(10.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn scope_1() -> Result<(), String> {
        // Setup
        let test_input: String = "x = 30; y = {x = 50; x}; x".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(30.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn scope_2() -> Result<(), String> {
        // Setup
        let test_input: String = "x = 30; y = {x = 50; x}; y".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(50.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn loose_comparison() -> Result<(), String> {
        // Setup
        let test_input: String = "30 == '30'".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::BoolVal(true) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn strict_comparison() -> Result<(), String> {
        // Setup
        let test_input: String = "30 === '30'".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::BoolVal(false) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn inequality_1() -> Result<(), String> {
        // Setup
        let test_input: String = "10 > 5".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::BoolVal(true) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn inequality_2() -> Result<(), String> {
        // Setup
        let test_input: String = "10 < 5".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::BoolVal(false) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn inequality_3() -> Result<(), String> {
        // Setup
        let test_input: String = "30 <= 30".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::BoolVal(true) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn add_null() -> Result<(), String> {
        // Setup
        let test_input: String = "0 + null".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(0.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn add_nan() -> Result<(), String> {
        // Setup
        let test_input: String = "0 + undefined".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(x) => assert!(x.is_nan()),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn negative_null() -> Result<(), String> {
        // Setup
        let test_input: String = "-null".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(0.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn negative() -> Result<(), String> {
        // Setup
        let test_input: String = "-5".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(-5.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn negative_sub_1() -> Result<(), String> {
        // Setup
        let test_input: String = "0 - -5".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(5.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn negative_sub_2() -> Result<(), String> {
        // Setup
        let test_input: String = "-5 - -1".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(-4.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn precedence_arithmetic_1() -> Result<(), String> {
        // Setup
        let test_input: String = "5 + 1 * 3".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(8.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn precedence_arithmetic_2() -> Result<(), String> {
        // Setup
        let test_input: String = "5 * 1 + 3".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(8.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn call_precedence() -> Result<(), String> {
        // Setup
        let test_input: String = "f = fun -> 5; 3 + f()".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(8.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn logor() -> Result<(), String> {
        // Setup
        let test_input: String = "3 + 5 || 10".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(8.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn logand() -> Result<(), String> {
        // Setup
        let test_input: String = "true && 5 + 10".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(15.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn logand_2() -> Result<(), String> {
        // Setup
        let test_input: String = "10 === 10 && 5 > -3".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::BoolVal(true) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn conditional_true() -> Result<(), String> {
        // Setup
        let test_input: String = "if true then 10 else 0".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(10.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn conditional_false() -> Result<(), String> {
        // Setup
        let test_input: String = "if false then 10 else 0".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(0.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn conditional_in_expr() -> Result<(), String> {
        // Setup
        let test_input: String = "5 + (if true then 10 else 0)".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(15.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn conditional_in_fun() -> Result<(), String> {
        // Setup
        let test_input: String = "isbig = fun x -> if x > 100 then true else false; isbig(43)".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::BoolVal(false) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn fun_expr_args() -> Result<(), String> {
        // Setup
        let test_input: String = "add = fun x, y -> {z = x + y; z}; add(1 + 2, 4 - 1)".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(6.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn type_cvt_str() -> Result<(), String> {
        // Setup
        let test_input: String = "&1982".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::StrVal(s) => assert_eq!(s, "1982"),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn type_cvt_bool() -> Result<(), String> {
        // Setup
        let test_input: String = "?''".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::BoolVal(false) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn type_cvt_num() -> Result<(), String> {
        // Setup
        let test_input: String = "+''".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(0.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn test_floor() -> Result<(), String> {
        // Setup
        let test_input: String = "_1.45493839".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(1.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn test_ceil() -> Result<(), String> {
        // Setup
        let test_input: String = "^1.45493839".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(2.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn test_tup_1() -> Result<(), String> {
        // Setup
        let test_input: String = "[4, 5, 6].0".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(4.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn test_tup_2() -> Result<(), String> {
        // Setup
        let test_input: String = "[4, 5, 6].1".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(5.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn test_tup_3() -> Result<(), String> {
        // Setup
        let test_input: String = "my_tup = [4, 5, 6]; my_tup.2".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(6.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn test_tup_4() -> Result<(), String> {
        // Setup
        let test_input: String = "my_tup = [4, 5, [6, 7]]; (my_tup.2).1".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(7.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn test_tup_5() -> Result<(), String> {
        // Setup
        let test_input: String = "x = 5; my_tup = [fun y -> x + y, 11]; my_tup.0(my_tup.1)".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(16.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn test_tup_6() -> Result<(), String> {
        // Setup
        let test_input: String = "my_tup = [11, 12, 15]; -my_tup.0".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(-11.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn test_tup_7() -> Result<(), String> {
        // Setup
        let test_input: String = "my_tup = [11, fun -> 11.87, 15]; _my_tup.1()".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::NumVal(11.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn test_tup_8() -> Result<(), String> {
        // Setup
        let test_input: String = "my_tup = [11, fun -> 11.87, 15]; &_my_tup.1()".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = types::Expr::BlockExpr(match test_parser.parse_script() { Ok(x) => x, _ => panic!("false") });
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment).unwrap();
        // Check output value
        match test_val {
            types::Val::StrVal(s) => assert_eq!(s, "11"),
            _ => assert!(false)
        }
        Ok(())
    }
}

#[cfg(test)]
mod test_database {
    use crate::sqlscript::types::types::Val;
    use super::super::database::engine::*;
    #[test]
    fn create_table_single() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (col1 num)".to_string());
        // Get table names
        let table_names = db.get_table_names();
        // Check values
        assert_eq!(table_names[0], "test_table");
        Ok(())
    }
    #[test]
    fn create_table_multi() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (col1 num)".to_string());
        db.execute("CREATE TABLE test_table2 (col1 num)".to_string());
        // Get table names
        let table_names = db.get_table_names();
        // Check values
        assert_eq!(table_names[0], "test_table");
        assert_eq!(table_names[1], "test_table2");
        Ok(())
    }
    #[test]
    fn insert_single() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1)".to_string());
        db.execute("INSERT INTO test_table VALUES (2)".to_string());
        Ok(())
    }
    #[test]
    fn insert_multi() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 str)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, 'testval')".to_string());
        db.execute("INSERT INTO test_table VALUES (2, 'hello, world')".to_string());
        Ok(())
    }
    #[test]
    fn insert_specific() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 str)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table (field1) VALUES (1)".to_string());
        db.execute("INSERT INTO test_table (field2) VALUES ('hello, world')".to_string());
        db.execute("INSERT INTO test_table (field2, field1) VALUES ('hello, world', 2)".to_string());
        Ok(())
    }
    #[test]
    fn select_basic_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 num)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, 2)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, 4)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, 6)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 3);
                let mut i = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(1.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_mix_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 num)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, 2)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, 4)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, 6)".to_string());
        // Perform select query
        let result = db.execute("SELECT field2, field1 FROM test_table".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 3);
                let mut i = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[1] {
                            Val::NumVal(1.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_mix_types() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 3);
                let mut i = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[2] {
                            Val::BoolVal(false) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_where_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table WHERE field2 == true".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 2);
                let mut i = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[2] {
                            Val::BoolVal(false) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    if i == 1 {
                        match row[2] {
                            Val::BoolVal(true) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_where_2() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table WHERE field1 > 4".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 1);
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_where_3() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table WHERE field1 > 1000".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 0);
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_where_4() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table WHERE field1 == 3 && field2 == false && field3 == false".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 1);
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_where_5() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table WHERE {isbig = fun x -> x >= 5; isbig(field1)}".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 1);
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_limit_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table LIMIT 1".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(1.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_limit_2() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table LIMIT 2 - 1".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(1.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_limit_3() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table LIMIT '1'".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(1.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_limit_4() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table LIMIT (fun -> 1)()".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(1.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_limit_5() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table LIMIT 0".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 0);
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_limit_where_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table WHERE field1 > 2 LIMIT (fun -> 1)()".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(3.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_order_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table ORDER BY field1".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 3);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(1.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    if i == 1 {
                        match row[0] {
                            Val::NumVal(3.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    if i == 2 {
                        match row[0] {
                            Val::NumVal(5.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_order_asc() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table ORDER BY field1 ASC".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 3);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(1.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    if i == 1 {
                        match row[0] {
                            Val::NumVal(3.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    if i == 2 {
                        match row[0] {
                            Val::NumVal(5.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_order_desc() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table ORDER BY field1 DESC".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 3);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(5.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    if i == 1 {
                        match row[0] {
                            Val::NumVal(3.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    if i == 2 {
                        match row[0] {
                            Val::NumVal(1.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_where_order() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table WHERE field1 == 3 || field1 == 5 ORDER BY field1 DESC".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 2);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(5.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    if i == 1 {
                        match row[0] {
                            Val::NumVal(3.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_where_order_limit() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table WHERE field1 == 3 || field1 == 5 ORDER BY field1 DESC LIMIT 1".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(5.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn const_val() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Add constants
        db.execute("CREATE CONST test_val = 5".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table WHERE field1 == test_val".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(5.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn const_fun() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Add constants
        db.execute("CREATE CONST max = fun x, y -> if x > y then x else y".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table WHERE field1 == max(1, 5)".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(5.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn calculated_column_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Add constants
        // Add calculated columns
        db.execute("CREATE COLUMN (bool) calc_1 = false INTO test_table".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table LIMIT 1".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[3] {
                            Val::BoolVal(false) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn calculated_column_2() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Add constants
        // Add calculated columns
        db.execute("CREATE COLUMN (bool) calc_1 = field2 INTO test_table".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table LIMIT 1".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[3] {
                            Val::BoolVal(true) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn calculated_column_3() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Add constants
        // Add calculated columns
        db.execute("CREATE COLUMN (bool) calc_1 = field1 > 10 INTO test_table".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table LIMIT 1".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[3] {
                            Val::BoolVal(false) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn calculated_column_before_insert() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Add calculated columns
        db.execute(" CREATE COLUMN (bool) calc_1 = field1 > 10 INTO test_table".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Add constants
        // Perform select query
        let result = db.execute("SELECT * FROM test_table LIMIT 1".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[3] {
                            Val::BoolVal(false) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn calculated_column_insert_before_after() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Add constants
        // Add calculated columns
        db.execute("CREATE COLUMN (bool) calc_1 = field1 === 7 INTO test_table".to_string());
        // Insert more data
        db.execute("INSERT INTO test_table VALUES (7, false, false)".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table ORDER BY field1 DESC LIMIT 1".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[3] {
                            Val::BoolVal(true) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn calculated_column_multi_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Add constants
        // Add calculated columns
        db.execute("CREATE COLUMN (bool) calc_1 = field1 == 5 INTO test_table".to_string());
        db.execute("CREATE COLUMN (bool) calc_2 = !calc_1 INTO test_table".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table LIMIT 1".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[4] {
                            Val::BoolVal(false) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn calculated_column_multi_2() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Add constants
        // Add calculated columns
        db.execute("CREATE COLUMN (num) calc_1 = if field2 then field1 * 2 else field1 * 3 INTO test_table".to_string());
        // Perform select query
        let result = db.execute("SELECT * FROM test_table LIMIT 1".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 1);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[3] {
                            Val::NumVal(10.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn aggregate_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Add aggregate
        db.execute("CREATE AGGREGATE max_field1 = current INIT field1 INTO test_table".to_string());
        // Perform select aggregate query
        let result = db.execute("SELECT AGGREGATE max_field1 FROM test_table".to_string());
        match result {
            QueryResult::Value(v) => {
                match v {
                    Val::NumVal(5.0) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn aggregate_2() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Add aggregate
        db.execute("CREATE AGGREGATE max_field1 = if field1 > current then field1 else current INIT field1 INTO test_table".to_string());
        // Perform select aggregate query
        let result = db.execute("SELECT AGGREGATE max_field1 FROM test_table".to_string());
        match result {
            QueryResult::Value(v) => {
                match v {
                    Val::NumVal(5.0) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn aggregate_multi_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 num)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, 6)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, 11)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, 3)".to_string());
        // Add aggregate
        db.execute("CREATE AGGREGATE max_sum = {sum = field1 + field2; if sum > current then sum else current} INIT field1 + field2 INTO test_table".to_string());
        // Perform select aggregate query
        let result = db.execute("SELECT AGGREGATE max_sum FROM test_table".to_string());
        match result {
            QueryResult::Value(v) => {
                match v {
                    Val::NumVal(12.0) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn aggregate_backwards_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 num)".to_string());
        // Add aggregate
        db.execute("CREATE AGGREGATE max_sum = {sum = field1 + field2; if sum > current then sum else current} INIT field1 + field2 INTO test_table".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, 6)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, 11)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, 3)".to_string());
        // Perform select aggregate query
        let result = db.execute("SELECT AGGREGATE max_sum FROM test_table".to_string());
        match result {
            QueryResult::Value(v) => {
                match v {
                    Val::NumVal(12.0) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn aggregate_backwards_2() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Add aggregate
        db.execute("CREATE AGGREGATE max_field1 = if field1 > current then field1 else current INIT field1 INTO test_table".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Perform select aggregate query
        let result = db.execute("SELECT AGGREGATE max_field1 FROM test_table".to_string());
        match result {
            QueryResult::Value(v) => {
                match v {
                    Val::NumVal(5.0) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn aggregate_no_init() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Add aggregate
        db.execute("CREATE AGGREGATE sum_field1 = if current === null then field1 else current + field1 INTO test_table".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Perform select aggregate query
        let result = db.execute("SELECT AGGREGATE sum_field1 FROM test_table".to_string());
        match result {
            QueryResult::Value(v) => {
                match v {
                    Val::NumVal(9.0) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn select_with_aggregate() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Aggregate
        db.execute("CREATE AGGREGATE max_field1 = if field1 > current then field1 else current INIT field1 INTO test_table".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Perform select query using aggregate
        let result = db.execute("SELECT * FROM test_table WHERE field1 < max_field1".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 2);
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn comp_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 bool, field3 bool)".to_string());
        // Aggregate
        db.execute("CREATE AGGREGATE max_field1 = if field1 > current then field1 else current INIT field1 INTO test_table".to_string());
        // Computation
        db.execute("CREATE COMP test_comp = 0 INTO test_table".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, true, true)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, true, false)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, false, false)".to_string());
        // Perform select query using aggregate
        let result = db.execute("SELECT COMP test_comp FROM test_table".to_string());
        match result {
            QueryResult::Value(v) => {
                match v {
                    Val::NumVal(0.0) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn comp_2() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 num)".to_string());
        // Add const
        db.execute("CREATE CONST max = fun a, b -> if a > b then a else b".to_string());
        // Add aggregate
        db.execute("CREATE AGGREGATE max_field1 = max(field1, current) INIT field1 INTO test_table".to_string());
        db.execute("CREATE AGGREGATE max_field2 = max(field2, current) INIT field2 INTO test_table".to_string());
        // Add computation
        db.execute("CREATE COMP sum_max = max_field1 + max_field2 INTO test_table".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, 6)".to_string());
        db.execute("INSERT INTO test_table VALUES (1, 11)".to_string());
        db.execute("INSERT INTO test_table VALUES (3, 3)".to_string());
        // Perform select query using aggregate
        let result = db.execute("SELECT COMP sum_max FROM test_table".to_string());
        match result {
            QueryResult::Value(v) => {
                match v {
                    Val::NumVal(16.0) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn comp_avg_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 num)".to_string());
        // Add aggregate
        db.execute("CREATE AGGREGATE sum_field1 = current + field1 INIT field1 INTO test_table".to_string());
        db.execute("CREATE AGGREGATE count = current + 1 INIT 1 INTO test_table".to_string());
        // Add computation
        db.execute("CREATE COMP avg_field1 = sum_field1 / count INTO test_table".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, 6)".to_string());
        db.execute("INSERT INTO test_table VALUES (6, 11)".to_string());
        db.execute("INSERT INTO test_table VALUES (8, 3)".to_string());
        // Perform select query using aggregate
        let result = db.execute("SELECT COMP avg_field1 FROM test_table".to_string());
        match result {
            QueryResult::Value(v) => {
                match v {
                    Val::NumVal(x) => assert_eq!(x as i32, 6),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn comp_avg_2() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 num)".to_string());
        // Add aggregate
        db.execute("CREATE AGGREGATE avg_field1_closure = {avg = ((current(0) * current(1)) + field1) / (current(1) + 1); counter = current(1) + 1; fun i -> if i then counter else avg} INIT {avg = field1; counter = 1; fun i -> if i then counter else avg} INTO test_table".to_string());
        // Add computation
        db.execute("CREATE COMP avg_field1 = avg_field1_closure(0) INTO test_table".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, 6)".to_string());
        db.execute("INSERT INTO test_table VALUES (6, 11)".to_string());
        db.execute("INSERT INTO test_table VALUES (8, 3)".to_string());
        // Perform select query using aggregate
        let result = db.execute("SELECT COMP avg_field1 FROM test_table".to_string());
        match result {
            QueryResult::Value(v) => {
                match v {
                    Val::NumVal(x) => assert_eq!(x as i32, 6),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn tuple_ag() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num, field2 num)".to_string());
        // Add aggregate
        db.execute("CREATE AGGREGATE counters = [current.0 + 1, current.1 + 2, current.2 + 3] INIT [1, 2, 3] INTO test_table".to_string());
        // Add computation
        db.execute("CREATE COMP counter = counters.2 INTO test_table".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5, 6)".to_string());
        db.execute("INSERT INTO test_table VALUES (15, 11)".to_string());
        db.execute("INSERT INTO test_table VALUES (10, 11)".to_string());
        // Perform select query using aggregate
        let result = db.execute("SELECT COMP counter FROM test_table".to_string());
        match result {
            QueryResult::Value(v) => {
                match v {
                    Val::NumVal(x) => assert_eq!(x as i32, 9),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn compress_num_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num none)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5)".to_string());
        db.execute("INSERT INTO test_table VALUES (6)".to_string());
        db.execute("INSERT INTO test_table VALUES (8)".to_string());
        db.execute("INSERT INTO test_table VALUES (13)".to_string());
        db.execute("INSERT INTO test_table VALUES (2)".to_string());
        db.execute("INSERT INTO test_table VALUES (5)".to_string());
        // Perform select query using aggregate
        let result = db.execute("SELECT * FROM test_table".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 6);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(5.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    if i == 4 {
                        match row[0] {
                            Val::NumVal(2.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn compress_num_2() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num bitmap)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5)".to_string());
        db.execute("INSERT INTO test_table VALUES (6)".to_string());
        db.execute("INSERT INTO test_table VALUES (8)".to_string());
        db.execute("INSERT INTO test_table VALUES (13)".to_string());
        db.execute("INSERT INTO test_table VALUES (2)".to_string());
        db.execute("INSERT INTO test_table VALUES (5)".to_string());
        // Perform select query using aggregate
        let result = db.execute("SELECT * FROM test_table".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 6);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(5.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    if i == 4 {
                        match row[0] {
                            Val::NumVal(2.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn compress_num_3() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num runlen)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5)".to_string());
        db.execute("INSERT INTO test_table VALUES (6)".to_string());
        db.execute("INSERT INTO test_table VALUES (8)".to_string());
        db.execute("INSERT INTO test_table VALUES (13)".to_string());
        db.execute("INSERT INTO test_table VALUES (2)".to_string());
        db.execute("INSERT INTO test_table VALUES (5)".to_string());
        // Perform select query using aggregate
        let result = db.execute("SELECT * FROM test_table".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 6);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(5.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    if i == 4 {
                        match row[0] {
                            Val::NumVal(2.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn compress_num_4() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num xor)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5)".to_string());
        db.execute("INSERT INTO test_table VALUES (6)".to_string());
        db.execute("INSERT INTO test_table VALUES (8)".to_string());
        db.execute("INSERT INTO test_table VALUES (13)".to_string());
        db.execute("INSERT INTO test_table VALUES (2)".to_string());
        db.execute("INSERT INTO test_table VALUES (5)".to_string());
        // Perform select query using aggregate
        let result = db.execute("SELECT * FROM test_table".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 6);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(5.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    if i == 4 {
                        match row[0] {
                            Val::NumVal(2.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn compress_str_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 str none)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES ('hello')".to_string());
        db.execute("INSERT INTO test_table VALUES ('my')".to_string());
        db.execute("INSERT INTO test_table VALUES ('name')".to_string());
        db.execute("INSERT INTO test_table VALUES ('is')".to_string());
        db.execute("INSERT INTO test_table VALUES ('hello')".to_string());
        db.execute("INSERT INTO test_table VALUES ('name')".to_string());
        // Perform select query using aggregate
        let result = db.execute("SELECT * FROM test_table".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 6);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match &row[0] {
                            Val::StrVal(s) => assert_eq!(s, "hello"),
                            _ => assert!(false)
                        }
                    }
                    if i == 5 {
                        match &row[0] {
                            Val::StrVal(s) => assert_eq!(s, "name"),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn compress_str_2() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 str bitmap)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES ('hello')".to_string());
        db.execute("INSERT INTO test_table VALUES ('my')".to_string());
        db.execute("INSERT INTO test_table VALUES ('name')".to_string());
        db.execute("INSERT INTO test_table VALUES ('is')".to_string());
        db.execute("INSERT INTO test_table VALUES ('hello')".to_string());
        db.execute("INSERT INTO test_table VALUES ('name')".to_string());
        // Perform select query using aggregate
        let result = db.execute("SELECT * FROM test_table".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 6);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match &row[0] {
                            Val::StrVal(s) => assert_eq!(s, "hello"),
                            _ => assert!(false)
                        }
                    }
                    if i == 5 {
                        match &row[0] {
                            Val::StrVal(s) => assert_eq!(s, "name"),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn compress_str_3() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 str runlen)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES ('hello')".to_string());
        db.execute("INSERT INTO test_table VALUES ('my')".to_string());
        db.execute("INSERT INTO test_table VALUES ('name')".to_string());
        db.execute("INSERT INTO test_table VALUES ('is')".to_string());
        db.execute("INSERT INTO test_table VALUES ('hello')".to_string());
        db.execute("INSERT INTO test_table VALUES ('name')".to_string());
        // Perform select query using aggregate
        let result = db.execute("SELECT * FROM test_table".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 6);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match &row[0] {
                            Val::StrVal(s) => assert_eq!(s, "hello"),
                            _ => assert!(false)
                        }
                    }
                    if i == 5 {
                        match &row[0] {
                            Val::StrVal(s) => assert_eq!(s, "name"),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn recompress_num_1() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num none)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5)".to_string());
        db.execute("INSERT INTO test_table VALUES (6)".to_string());
        db.execute("INSERT INTO test_table VALUES (8)".to_string());
        db.execute("INSERT INTO test_table VALUES (13)".to_string());
        db.execute("INSERT INTO test_table VALUES (2)".to_string());
        db.execute("INSERT INTO test_table VALUES (5)".to_string());
        // Recompress
        db.execute("COMPRESS test_table (field1) (xor)".to_string());
        // Perform select query using aggregate
        let result = db.execute("SELECT * FROM test_table".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 6);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(5.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    if i == 4 {
                        match row[0] {
                            Val::NumVal(2.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
    #[test]
    fn recompress_num_2() -> Result<(), String> {
        // Setup
        let mut db = Database::new();
        // Create table
        db.execute("CREATE TABLE test_table (field1 num none)".to_string());
        // Insert values into table
        db.execute("INSERT INTO test_table VALUES (5)".to_string());
        db.execute("INSERT INTO test_table VALUES (6)".to_string());
        db.execute("INSERT INTO test_table VALUES (8)".to_string());
        db.execute("INSERT INTO test_table VALUES (13)".to_string());
        db.execute("INSERT INTO test_table VALUES (2)".to_string());
        db.execute("INSERT INTO test_table VALUES (5)".to_string());
        // Recompress
        db.execute("COMPRESS test_table (field1) xor".to_string());
        // Perform select query using aggregate
        let result = db.execute("SELECT * FROM test_table".to_string());
        match result {
            QueryResult::Table(t) => {
                assert_eq!(t.len(), 6);
                let mut i: usize = 0;
                for row in t.iter() {
                    if i == 0 {
                        match row[0] {
                            Val::NumVal(5.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    if i == 4 {
                        match row[0] {
                            Val::NumVal(2.0) => assert!(true),
                            _ => assert!(false)
                        }
                    }
                    i += 1;
                }
            },
            _ => assert!(false)
        }
        Ok(())
    }
}