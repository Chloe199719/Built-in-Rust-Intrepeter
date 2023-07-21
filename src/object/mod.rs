use std::{fmt::Display, any};
#[derive(Debug, PartialEq, Clone)]
pub enum ObjectType {
    INTEGER,
    BOOLEAN,
    NULL
}
    
impl Display for ObjectType  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectType::INTEGER => write!(f, "INTEGER"),
            ObjectType::BOOLEAN => write!(f, "BOOLEAN"),
            ObjectType::NULL => write!(f, "NULL"),
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