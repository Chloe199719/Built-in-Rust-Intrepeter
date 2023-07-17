use crate::token::Token;
pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node + std::fmt::Debug {
    fn statement_node(&self);
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait Expression: Node + std::fmt::Debug {
    fn expression_node(&self);
}


pub struct Program {
    pub statements: Vec<Box<dyn Statement>>
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if let Some(statement) = self.statements.first() {
            statement.token_literal()
        } else {
            String::from("")
        }
    }
}
#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Box<dyn Expression >
}
impl Statement for  LetStatement {
    fn statement_node(&self) {
        
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Box<dyn Expression>
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {
        
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}


#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String
}

impl Expression for  Identifier {
     fn expression_node(&self) {
        
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
