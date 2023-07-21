use std::rc::Rc;

use crate::token::Token;
pub enum NodeType {
    Program,
    Statement,
    Expression,
    Identifier,
    LetStatement,
    ReturnStatement,
    ExpressionStatement,
    IntegerLiteral,
    PrefixExpression,
    InfixExpression,
    Boolean,
    IfExpression,
    BlockStatement,
    FunctionLiteral,
    CallExpression
}


pub trait Node  {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
    fn as_any(&self) -> &dyn std::any::Any;
    fn node_type(&self) -> NodeType;
}

pub trait Statement: Node + std::fmt::Debug + std::any::Any    {
    fn statement_node(&self);
    fn as_node(&self) -> &dyn Node ;
  
    // fn as_any(&self) -> &dyn std::any::Any;
}

pub trait Expression: Node + std::fmt::Debug + std::any::Any  {
    fn as_node(&self) -> &dyn Node ;

    fn expression_node(&self);
    // fn as_any(&self) -> &dyn std::any::Any;
}

#[derive(Debug)]
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
    fn string(&self) -> String {
        let mut out = String::new();
        for statement in &self.statements {
            out.push_str(&statement.string());
        }
        out
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn node_type(&self) -> NodeType {
        NodeType::Program
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

    fn as_node(&self) -> &dyn Node {
        self
    }
    // fn as_any(&self) -> &dyn std::any::Any {
    //     self
    // }
}
impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ");
        out.push_str(&self.name.string());
        out.push_str(" = ");
        if self.value.string() != "" {
            out.push_str(&self.value.string());
        }  
        out.push_str(";");
        out
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn node_type(&self) -> NodeType {
        NodeType::LetStatement
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

    fn as_node(&self) -> &dyn Node {
        self
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ");
        if self.return_value.string() != "" {
            out.push_str(&self.return_value.string());
        }  
        out.push_str(";");
        out
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn node_type(&self) -> NodeType {
        NodeType::ReturnStatement
    }

}


#[derive(Debug,Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String
}

impl Expression for  Identifier {
     fn expression_node(&self) {
        
    }

    fn  as_node(&self) -> &dyn Node {
        self
    }
    // fn as_any(&self) -> &dyn std::any::Any {
    //     self
    // }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        self.value.clone()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn node_type(&self) -> NodeType {
        NodeType::Identifier
    }
}
#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Box<dyn Expression>
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {
        
    }

    fn as_node(&self) -> &dyn Node {
        self
    }
    // fn as_any(&self) -> &dyn std::any::Any {
    //     self
    // }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        if self.expression.string() != "" {
            self.expression.string()
        } else {
            String::from("")
        }
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn node_type(&self) -> NodeType {
        NodeType::ExpressionStatement
    }
}

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {
        
    }

    fn as_node(&self) -> &dyn Node {
        self
    }
    // fn as_any(&self) -> &dyn std::any::Any {
    //     self
    // }
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        self.token.literal.clone()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn node_type(&self) -> NodeType {
        NodeType::IntegerLiteral
    }
    
}

#[derive(Debug)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<dyn Expression>
}
impl Expression for PrefixExpression {
    fn expression_node(&self) {
        
    }

    fn as_node(&self) -> &dyn Node {
        self
    }

    // fn as_any(&self) -> &dyn std::any::Any {
    //     self
    // }

    
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push('(');
        out.push_str(&self.operator);
        out.push_str(&self.right.string());
        out.push(')');
        out
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn node_type(&self) -> NodeType {
        NodeType::PrefixExpression
    }
    
}
#[derive(Debug)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<dyn Expression>,
    pub operator: String,
    pub right: Box<dyn Expression>
}
impl Expression for InfixExpression {
    fn expression_node(&self) {
        
    }

    fn as_node(&self) -> &dyn Node {
        self
    }
    // fn as_any(&self) -> &dyn std::any::Any {
    //     self
    // }
    
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push('(');
        out.push_str(&self.left.string());
        out.push(' ');
        out.push_str(&self.operator);
        out.push(' ');
        out.push_str(&self.right.string());
        out.push(')');
        out
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn node_type(&self) -> NodeType {
        NodeType::InfixExpression
    }
    
}
#[derive(Debug)]
pub struct Boolean {
    pub token: Token,
    pub value: bool
}
impl Expression for Boolean {
    fn expression_node(&self) {
        
    }
    // fn as_any(&self) -> &dyn std::any::Any {
    //     self
    // }

    fn as_node(&self) -> &dyn Node {
        self
    }
    
}

impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        self.token.literal.clone()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn node_type(&self) -> NodeType {
        NodeType::Boolean
    }
    
}

#[derive(Debug)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Box<dyn Expression>,
    pub consequence: Box <dyn Statement>,
    pub alternative: Option<Box <dyn Statement>>
}
impl Expression for IfExpression {
    fn expression_node(&self) {
        
    }

    fn as_node(&self) -> &dyn Node {
        self
    }
    // fn as_any(&self) -> &dyn std::any::Any {
    //     self
    // }

    
}

impl Node for IfExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("if");
        out.push_str(&self.condition.string());
        out.push_str(" ");
        out.push_str(&self.consequence.string());
        if let Some(alt) = &self.alternative {
            out.push_str("else ");
            out.push_str(&alt.string());
        }
        out
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn node_type(&self) -> NodeType {
        NodeType::IfExpression
    }
    
}

#[derive(Debug)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Box<dyn Statement>>
}
impl Statement for BlockStatement {
    fn statement_node(&self) {
        
    }

    fn as_node(&self) -> &dyn Node {
        self
    }
    // fn as_any(&self) -> &dyn std::any::Any {
    //     self
    // }

    
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        let mut out = String::new();
        for s in &self.statements {
            out.push_str(&s.string());
        }
        out
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn node_type(&self) -> NodeType {
        NodeType::BlockStatement
    }
    
}

#[derive(Debug)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: Rc<Box<dyn Statement>>
}
impl Expression for FunctionLiteral {
    fn expression_node(&self) {
        
    }

    fn as_node(&self) -> &dyn Node {
        self
    }


    // fn as_any(&self) -> &dyn std::any::Any {
    //     self
    // }

    
}
impl FunctionLiteral {
    pub fn get_parameters(&self) -> Vec<Identifier> {
        self.parameters.clone()
    }

}
impl Node for FunctionLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        let mut out = String::new();
        let mut params = Vec::new();
        for p in &self.parameters {
            params.push(p.string());
        }
        out.push_str(&self.token_literal());
        out.push('(');
        out.push_str(&params.join(", "));
        out.push_str(") ");
        out.push_str(&self.body.string());
        out
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn node_type(&self) -> NodeType {
        NodeType::FunctionLiteral
    }
    
}

#[derive(Debug)]
pub struct CallExpression {
    pub token: Token,
    pub function: Box<dyn Expression>,
    pub arguments: Vec<Box<dyn Expression>>
}

impl Expression for CallExpression {
    fn expression_node(&self) {
        
    }
    // fn as_any(&self) -> &dyn std::any::Any {
    //     self
    // }
    fn as_node(&self) -> &dyn Node {
        self
    }

    
}

impl Node for CallExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        let mut out = String::new();
        let mut args = Vec::new();
        for a in &self.arguments {
            args.push(a.string());
        }
        out.push_str(&self.function.string());
        out.push('(');
        out.push_str(&args.join(", "));
        out.push(')');
        out
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn node_type(&self) -> NodeType {
        NodeType::CallExpression
    }
    
}







#[cfg(test)]

mod test {
    use super::*;
    use crate:: token::TokenType;
    #[test]
    fn test_string() {
       let program = Program {
           statements: vec![
               Box::new(LetStatement {
                   token: Token::new(TokenType::LET, "let"),
                   name: Identifier {
                       token: Token::new(TokenType::IDENT, "myVar"),
                       value: String::from("myVar")
                   },
                   value: Box::new(Identifier {
                       token: Token::new(TokenType::IDENT, "anotherVar"),
                       value: String::from("anotherVar")
                   })
               })
           ]
       };
         assert_eq!(program.string(), "let myVar = anotherVar;");

    }
}
