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
    pub fn execute(script: &Expr, env: &mut Environment) -> () {
        match script {
            Expr::BopExpr(e1, bop, e2) => {
                let v1 = execute(e1.as_ref(), env);
                let v2 = execute(e2.as_ref(), env);
            }
            _ => panic!("Unimplemented")
        }
    }
}