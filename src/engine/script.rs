pub mod script {
    use core::f64;
    use std::rc::Rc;
    use crate::sqlscript::parser::parser::parsetree::*;
    #[derive(Clone)]
    pub struct Frame {
        data: Vec<(String, Val)>,
    }
    impl Frame {
        pub fn push(&mut self, name: &String, data: &Val) {
            self.data.push((name.clone(), data.clone()))
        }
        pub fn get(&self, name: &String) -> Option<Val> {
            let mut found = None;
            for i in (0..self.data.len()).rev() {
                if self.data[i].0 == *name {
                    found = Some(self.data[i].1.clone());
                }
            };
            found
        }
        pub fn new() -> Frame {
            Frame {
                data: Vec::new()
            }
        }
        pub fn contains(&self, name: &String) -> bool {
            for i in 0..self.data.len() {
                if self.data[i].0 == *name {
                    return true;
                }
            };
            false
        }
        pub fn data(&self) -> &Vec<(String, Val)> {
            &self.data
        }
    }
    #[derive(Clone)]
    pub struct Environment {
        frames: Vec<Frame>
    }
    impl Environment {
        pub fn new() -> Environment {
            Environment {
                frames: Vec::new()
            }
        }
        pub fn compress(&self) -> Frame {
            let mut new_frame = Frame::new();
            for i in 0..(self.frames.len()) {
                let frame_data = self.frames[i].data();
                for j in 0..(frame_data.len()) {
                    let data = &frame_data[j];
                    if !(new_frame.contains(&data.0)) {
                        new_frame.push(&data.0, &data.1)
                    }
                }
            };
            new_frame
        }
        pub fn new_frame(&mut self) {
            self.frames.push(Frame::new())
        }
        pub fn pop_frame(&mut self) {
            self.frames.pop();
        }
        pub fn push_frame(&mut self, frame: Frame) {
            self.frames.push(frame)
        }
        pub fn push(&mut self, name: &String, data: &Val) {
            if self.frames.len() <= 0 {
                self.new_frame()
            };
            let frames_len = self.frames.len();
            self.frames[frames_len - 1].push(name, data)
        }
        pub fn get(&mut self, name: &String) -> Option<Val> {
            let frames_len = self.frames.len();
            for i in (0..frames_len).rev() {
                let retrived_value = self.frames[i].get(name);
                match retrived_value {
                    Some(_) => return retrived_value,
                    _ => continue
                }
            };
            None
        }
    }
    fn to_bool(val: &Val) -> Val {
        match val {
            Val::BoolVal(x) => Val::BoolVal(*x),
            Val::NumVal(x) => Val::BoolVal(*x != 0.0),
            Val::NullVal => Val::BoolVal(false),
            Val::UndefVal => Val::BoolVal(false),
            Val::StrVal(x) => Val::BoolVal(x != ""),
            _ => panic!("Unexpected value")
        }
    }
    fn extract_bool(val: &Val) -> bool {
        match val {
            Val::BoolVal(x) => *x,
            _ => panic!("Unexpected value")
        }
    }
    fn to_num(val: &Val) -> Val {
        match val {
            Val::BoolVal(x) => Val::NumVal(if *x {1.0} else {0.0}),
            Val::NumVal(x) => Val::NumVal(*x),
            Val::NullVal => Val::NumVal(0.0),
            Val::UndefVal => Val::NumVal(f64::NAN),
            Val::StrVal(x) => {
                match x.parse::<f64>() {
                    Ok(v) => Val::NumVal(v),
                    _ => Val::NumVal(f64::NAN)
                }
            },
            _ => panic!("Unexpected value")
        }
    }
    fn extract_num(val: &Val) -> f64 {
        match val {
            Val::NumVal(x) => *x,
            _ => panic!("Unexpected value")
        }
    }
    fn to_str(val: &Val) -> Val {
        match val {
            Val::BoolVal(x) => Val::StrVal(if *x {"true".to_string()} else {"false".to_string()}),
            Val::NumVal(x) => Val::StrVal(x.to_string()),
            Val::NullVal => Val::StrVal("null".to_string()),
            Val::UndefVal => Val::StrVal("undefined".to_string()),
            Val::StrVal(x) => Val::StrVal(x.clone()),
            _ => panic!("Unexpected value")
        }
    }
    fn extract_str(val: &Val) -> String {
        match val {
            Val::StrVal(x) => x.clone(),
            _ => panic!("Unexpected value")
        }
    }
    fn eq(a: &Val, b: &Val) -> bool {
        match (a, b) {
            (Val::BoolVal(av), Val::BoolVal(bv)) => av == bv,
            (Val::NumVal(av), Val::NumVal(bv)) => av == bv,
            (Val::StrVal(av), Val::StrVal(bv)) => av == bv,
            (Val::NullVal, Val::NullVal) 
            | (Val::UndefVal, Val::UndefVal)
            | (Val::NullVal, Val::UndefVal)
            | (Val::UndefVal, Val::NullVal) => true,
            (Val::BoolVal(_), _) => eq(&to_num(a), b),
            (_, Val::BoolVal(_)) => eq(a, &to_num(b)),
            (Val::NumVal(_), Val::StrVal(_)) => eq(a, &to_num(b)),
            (Val::StrVal(_), Val::NumVal(_)) => eq(&to_num(a), b),
            _ => false
        }
    }
    fn stricteq(a: &Val, b: &Val) -> bool {
        match (a, b) {
            (Val::BoolVal(av), Val::BoolVal(bv)) => av == bv,
            (Val::NumVal(av), Val::NumVal(bv)) => av == bv,
            (Val::StrVal(av), Val::StrVal(bv)) => av == bv,
            (Val::NullVal, Val::NullVal) 
            | (Val::UndefVal, Val::UndefVal) => true,
            _ => false
        }
    }
    fn eval_block(block: &Block, env: &mut Environment) -> Val {
        // Match type of block
        match block {
            Block::StmtBlock(id, e1, b2) => {
                let v1 = eval(e1.as_ref(), env);
                env.push(id, &v1);
                eval_block(b2.as_ref(), env)
            },
            Block::ExprBlock(e1) => eval(e1.as_ref(), env)
        }
    }
    pub fn eval(script: &Expr, env: &mut Environment) -> Val {
        match script {
            Expr::BopExpr(e1, bop, e2) => {
                let v1 = eval(e1.as_ref(), env);
                let v2 = eval(e2.as_ref(), env);
                match bop {
                    // Arithmetic
                    BopType::PlusBop => {
                        match (&v1, &v2) {
                            (Val::StrVal(s1), Val::StrVal(s2)) => Val::StrVal(format!("{}{}", s1, s2)),
                            _ => Val::NumVal(extract_num(&to_num(&v1)) + extract_num(&to_num(&v2)))
                        }
                    },
                    BopType::MinusBop => Val::NumVal(extract_num(&to_num(&v1)) - extract_num(&to_num(&v2))),
                    BopType::TimesBop => Val::NumVal(extract_num(&to_num(&v1)) * extract_num(&to_num(&v2))),
                    BopType::DivBop => Val::NumVal(extract_num(&to_num(&v1)) / extract_num(&to_num(&v2))),
                    // Comparison
                    BopType::EqBop => Val::BoolVal(eq(&v1, &v2)),
                    BopType::StrEqBop => Val::BoolVal(stricteq(&v1, &v2)),
                    BopType::GtBop => {
                        match (&v1, &v2) {
                            (Val::StrVal(s1), Val::StrVal(s2)) => Val::BoolVal(s1 > s2),
                            _ => Val::BoolVal(extract_num(&to_num(&v1)) > extract_num(&to_num(&v2)))
                        }
                    },
                    BopType::GteBop => {
                        match (&v1, &v2) {
                            (Val::StrVal(s1), Val::StrVal(s2)) => Val::BoolVal(s1 >= s2),
                            _ => Val::BoolVal(extract_num(&to_num(&v1)) >= extract_num(&to_num(&v2)))
                        }
                    },
                    BopType::LtBop => {
                        match (&v1, &v2) {
                            (Val::StrVal(s1), Val::StrVal(s2)) => Val::BoolVal(s1 < s2),
                            _ => Val::BoolVal(extract_num(&to_num(&v1)) < extract_num(&to_num(&v2)))
                        }
                    },
                    BopType::LteBop => {
                        match (&v1, &v2) {
                            (Val::StrVal(s1), Val::StrVal(s2)) => Val::BoolVal(s1 <= s2),
                            _ => Val::BoolVal(extract_num(&to_num(&v1)) <= extract_num(&to_num(&v2)))
                        }
                    },
                    // Logical
                    BopType::LogAndBop => if extract_bool(&to_bool(&v1)) { v2 } else { v1 },
                    BopType::LogOrBop => if extract_bool(&to_bool(&v1)) { v1 } else { v2 },
                    _ => panic!("Unimplemented")
                }
            },
            Expr::CondExpr(e1, e2, e3) => {
                let v1 = eval(e1.as_ref(), env);
                if extract_bool(&to_bool(&v1)) { eval(e2.as_ref(), env) } else { eval(e3.as_ref(), env) }
            },
            Expr::UopExpr(uop, e1) => {
                let v1 = eval(e1.as_ref(), env);
                match uop {
                    UopType::NegUop => Val::NumVal(- extract_num(&to_num(&v1))),
                    UopType::NotUop => Val::BoolVal(! extract_bool(&to_bool(&v1)))
                }
            }
            Expr::ValExpr(v1) => v1.clone(),
            Expr::FunExpr(il, e1) => Val::ClosureVal(env.compress(), il.clone(), e1.clone()),
            Expr::CallExpr(e1, el) => {
                let v1 = eval(e1.as_ref(), env);
                match &v1 {
                    Val::ClosureVal(fr, il, body) => {
                        // Make sure args match parameters
                        if el.len() != il.len() { panic!("Call error") };
                        // New frame
                        let mut arg_frame = Frame::new();
                        // Add args to new frame
                        for i in 0..el.len() { arg_frame.push(&il[i], &eval(&el[i], env)); }
                        // Push closure frame to environment
                        env.push_frame(fr.clone());
                        // Push args frame to environment
                        env.push_frame(arg_frame);
                        // Evaluate function body
                        let retval = eval(body.as_ref(), env);
                        // Pop stack frames
                        env.pop_frame();
                        env.pop_frame();
                        // Return
                        retval
                    },
                    _ => panic!("Unexpected value")
                }
            },
            Expr::BlockExpr(block) => {
                // Push stack frame
                env.new_frame();
                // Evaluate block
                let v1 = eval_block(block, env);
                // Pop stack frame
                env.pop_frame();
                v1
            },
            Expr::IdentExpr(id) => {
                // Get id from environment
                let val = env.get(id);
                // Check value
                match val {
                    Some(v1) => v1,
                    _ => panic!("Bad variable")
                }
            },
            _ => panic!("Unimplemented")
        }
    }
}

#[cfg(test)]
mod test_script {
    use crate::sqlscript::parser::parser::{Parser, parsetree};
    use super::script::*;
    #[test]
    fn basic_addition() -> Result<(), String> {
        // Setup
        let test_input: String = "432 + 5".to_string();
        let mut test_environment = Environment::new();
        let mut test_parser = Parser::new(test_input);
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(437.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        test_environment.push(&"x".to_string(), &parsetree::Val::NumVal(5.0));
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(5.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(1.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(3.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(6.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::StrVal(s) => assert_eq!(s, "Hello World"),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(33.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(5.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::UndefVal => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::UndefVal => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NullVal => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(20.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(10.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(10.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(30.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(50.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::BoolVal(true) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::BoolVal(false) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::BoolVal(true) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::BoolVal(false) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::BoolVal(true) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(0.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(x) => assert!(x.is_nan()),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(0.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(-5.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(5.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(-4.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(8.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(8.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(8.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(8.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(15.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::BoolVal(true) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(10.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(0.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(15.0) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::BoolVal(false) => assert!(true),
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
        let ast = parsetree::Expr::BlockExpr(test_parser.parse_script());
        // Evaluate input
        let test_val = eval(&ast, &mut test_environment);
        // Check output value
        match test_val {
            parsetree::Val::NumVal(6.0) => assert!(true),
            _ => assert!(false)
        }
        Ok(())
    }
}