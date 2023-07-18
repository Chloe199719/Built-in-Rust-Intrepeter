use crate::token::Token;
pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

pub trait Statement: Node + std::fmt::Debug {
    fn statement_node(&self);
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait Expression: Node + std::fmt::Debug {
    fn expression_node(&self);
    fn as_any(&self) -> &dyn std::any::Any;
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
    fn string(&self) -> String {
        let mut out = String::new();
        for statement in &self.statements {
            out.push_str(&statement.string());
        }
        out
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
}


#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String
}

impl Expression for  Identifier {
     fn expression_node(&self) {
        
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        self.value.clone()
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
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
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
