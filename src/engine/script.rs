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
            for i in (self.data.len()-1)..0 {
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
            for i in 0..(self.data.len()-1) {
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
            for i in frames_len..0 {
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
}