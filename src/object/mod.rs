use std::{fmt::Display, any, rc::Rc};
use crate::ast;
use crate::ast::Statement;
use crate::envoriment::Environment;
use ast::Node;
#[derive(Debug, PartialEq, Clone)]
pub enum ObjectType {
    INTEGER,
    BOOLEAN,
    NULL,
    RETURN,
    ERROR,

    ENVIRONMENT,
    FUNCTION,

    STRING,
    BUILTIN,

}
    
impl Display for ObjectType  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectType::INTEGER => write!(f, "INTEGER"),
            ObjectType::BOOLEAN => write!(f, "BOOLEAN"),
            ObjectType::NULL => write!(f, "NULL"),
            ObjectType::RETURN => write!(f, "RETURN"),
            ObjectType::ERROR => write!(f, "ERROR"),
            ObjectType::ENVIRONMENT => write!(f, "ENVIRONMENT"),
            ObjectType::FUNCTION => write!(f, "FUNCTION"),
            ObjectType::STRING => write!(f, "STRING"),
            ObjectType::BUILTIN => write!(f, "BUILTIN"),
        }
    }   
}



pub trait Object  {
    fn object_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
    fn as_any(&self) -> &dyn any::Any;

}

#[derive(Debug, PartialEq, Clone)]
pub struct Integer {
    pub value: i64
}

impl Object for Integer {
    fn object_type(&self) -> ObjectType {
        ObjectType::INTEGER
    }
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
    fn as_any(&self) -> &dyn any::Any {
        self
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Boolean {
    pub value: bool
}

impl Object for Boolean {
    fn object_type(&self) -> ObjectType {
        ObjectType::BOOLEAN
    }
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
    fn as_any(&self) -> &dyn any::Any {
        self
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct Null;

impl Object for Null {
    fn object_type(&self) -> ObjectType {
        ObjectType::NULL
    }
    fn inspect(&self) -> String {
        format!("null")
    }
    fn as_any(&self) -> &dyn any::Any {
        self
    }
}


pub struct Return {
    pub value: Rc<Box<dyn Object>>
}

impl Object for Return {
    fn object_type(&self) -> ObjectType {
        ObjectType::RETURN
    }
    fn inspect(&self) -> String {
        format!("{}", self.value.inspect())
    }
    fn as_any(&self) -> &dyn any::Any {
        self
    }
    
    
}
impl Return  {
    pub fn value(&self) ->Rc<Box<dyn Object>> {
        self.value.clone()
    }
}


pub struct Error {
    pub message: String
}

impl Object for Error {
    fn object_type(&self) -> ObjectType {
        ObjectType::ERROR
    }
    fn inspect(&self) -> String {
        format!("ERROR: {}", self.message)
    }
    fn as_any(&self) -> &dyn any::Any {
        self
    }
}


pub struct Function {
    pub parameters: Rc<Box<Vec<ast::Identifier>>>,
    pub body: Rc<Box<dyn Statement>>,
    pub env: Environment,
}

impl Object for Function {
    fn object_type(&self) -> ObjectType {
        ObjectType::FUNCTION
    }
    fn inspect(&self) -> String {
        let mut out = String::new();
        let mut params = Vec::new();
        for p in self.parameters.iter() {
            params.push(p.string());
        }
        out.push_str("fn");
        out.push_str("(");
        out.push_str(&params.join(", "));
        out.push_str(") {\n");
        out.push_str(&self.body.string());
        out.push_str("\n}");
        out
    }
    fn as_any(&self) -> &dyn any::Any {
        self
    }
    
}

pub struct StringValue {
    pub value: String,
}

impl Object for StringValue {
    fn object_type(&self) -> ObjectType {
        ObjectType::STRING
    }
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
    fn as_any(&self) -> &dyn any::Any {
        self
    }
    
}

#[derive(Debug, PartialEq, Clone)]
pub struct Builtin {
    pub func: fn(Vec<Rc<Box<dyn Object>>>) -> Rc<Box<dyn Object>>
}

impl Object for Builtin {
    fn object_type(&self) -> ObjectType {
        ObjectType::BUILTIN
    }
    fn inspect(&self) -> String {
        format!("builtin function")
    }
    fn as_any(&self) -> &dyn any::Any {
        self
    }
    
}