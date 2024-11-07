pub mod script {
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
    pub fn execute(script: &Expr, env: &mut Environment) -> Val {
        match script {
            Expr::BopExpr(e1, bop, e2) {
                let v1 = execute(e1.as_ref(), env);
                let v2 = execute(e2.as_ref(), env);
            }
            _ => panic!("Unimplemented")
        }
    }
}