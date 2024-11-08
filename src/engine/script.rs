pub mod script {
    use core::f64;

    use crate::sqlscript::parser::parser::parsetree::*;
    pub struct Frame {
        names: Vec<String>,
        data: Vec<Val>
    }
    impl Frame {
        pub fn push(&mut self, name: &String, data: &Val) {
            self.names.push(name.clone());
            self.data.push(data.clone())
        }
        pub fn get(&self, name: &String) -> Option<Val> {
            let mut found = None;
            for i in (self.names.len()-1)..0 {
                if self.names[i] == *name {
                    found = Some(self.data[i].clone());
                }
            };
            found
        }
        pub fn new() -> Frame {
            Frame {
                names: Vec::new(),
                data: Vec::new()
            }
        }
    }
    pub struct Environment {
        frames: Vec<Frame>
    }
    impl Environment {
        pub fn new() -> Environment {
            Environment {
                frames: Vec::new()
            }
        }
        pub fn push_frame(&mut self) {
            self.frames.push(Frame::new())
        }
        pub fn pop_frame(&mut self) {
            self.frames.pop();
        }
        pub fn push(&mut self, name: &String, data: &Val) {
            if self.frames.len() <= 0 {
                self.push_frame()
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
    pub fn execute(script: &Expr, env: &mut Environment) -> Val {
        match script {
            Expr::BopExpr(e1, bop, e2) => {
                let v1 = execute(e1.as_ref(), env);
                let v2 = execute(e2.as_ref(), env);
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
                let v1 = execute(e1.as_ref(), env);
                if extract_bool(&to_bool(&v1)) { execute(e2.as_ref(), env) } else { execute(e3.as_ref(), env) }
            },
            _ => panic!("Unimplemented")
        }
    }
}