use std::{collections::HashMap, rc::Rc};

use crate::object;


#[derive( Clone)]
pub struct Environment {
    pub store: HashMap<String, Rc<Box<dyn object::Object>>>,
    pub outer: Option< HashMap<String, Rc<Box<dyn object::Object>>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn get(&self, name: &str) -> Option<Rc<Box<dyn object::Object>>> {
        println!("get: {}", name);
        match self.store.get(name) {
            Some(obj) => Some(obj.clone()),
            None =>  match &self.outer {
                Some(outer) => match outer.get(name) {
                    Some(obj) => Some(obj.clone()),
                    None => None
                },
                None => None
            }
        }
    }

    pub fn set(&mut self, name: &str, val: Rc<Box<dyn object::Object>>) {
        self.store.insert(name.to_string(), val);
    }

    pub fn new_enclosed_environment(outer: HashMap<String, Rc<Box<dyn object::Object>>>) -> Self {
        let mut env = Environment::new();
        env.outer = Some(outer);
        env
    }
    
}

impl object::Object for Environment {
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn object_type(&self) -> object::ObjectType {
        object::ObjectType::ENVIRONMENT
    }

    fn inspect(&self) -> String {
        let mut out = String::new();
        out.push_str("ENVIRONMENT\n");
        for (k, v) in self.store.iter() {
            out.push_str(&format!("{}: {}\n", k, v.inspect()));
        }
        out
    }

}