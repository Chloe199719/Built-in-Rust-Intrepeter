#[allow(dead_code)]
#[allow(unused_imports)]
use std::collections::HashMap;

use crate::token::{Token, self};
use crate::ast;
use crate::lexer::Lexer;
use crate::ast::Node;


pub type PrefixParseFn = fn(&mut Parser) -> Option<Box<dyn ast::Expression>>;


type InfixParseFn = fn(Option<Box<dyn ast::Expression>>) -> Option<Box<dyn ast::Expression>>;

#[derive(Clone)]
pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
    perfix_parse_fns: HashMap<token::TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<token::TokenType, InfixParseFn>,
}
pub enum Precedence {
    LOWEST,
    EQUALS, // ==
    LESSGREATER, // > or <
    SUM, // +
    PRODUCT, // *
    PREFIX, // -X or !X
    CALL, // myFunction(X)    
}



impl Parser {
    pub fn new (lexer: Lexer) -> Parser {
        let mut p = Parser {
            lexer,
            cur_token: Token::new(token::TokenType::EOF, ""),
            peek_token: Token::new(token::TokenType::EOF, ""),
            errors: Vec::new(),
            perfix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };
         p.register_prefix(token::TokenType::IDENT, Parser::parse_identifier_expression);
         p.register_prefix(token::TokenType::INT, Parser::parse_integer_literal);
         p.register_prefix(token::TokenType::BANG, Parser::parse_prefix_expression);
         p.register_prefix(token::TokenType::MINUS, Parser::parse_prefix_expression);

        p.next_token();
        p.next_token();
        p
    }
    fn register_prefix(&mut self, token_type: token::TokenType, parse_fn: fn(&mut Parser) -> Option<Box<dyn ast::Expression>>) {
        self.perfix_parse_fns.insert(token_type, parse_fn);
    }
    pub fn register_infix(&mut self, token_type: token::TokenType, func: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, func);
    }
  fn parse_identifier_expression(&mut self) -> Option<Box<dyn ast::Expression>> {
        Some(Box::new(ast::Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }))
    }
    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }
    pub fn peek_error(&mut self, t: token::TokenType) {
        let msg = format!("expected next token to be {:?}, got {:?} instead", t, self.peek_token.type_);
        self.errors.push(msg);
    }
    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }
    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program {
            statements: Vec::new(),
        };
        while self.cur_token.type_ != token::TokenType::EOF {
            let stmt = self.parse_statement();
            match stmt {
                Some(x) => program.statements.push(x),
                None => (),
            }
            self.next_token();
        }
        program
    } 
    // fn parse_identifier(&self) -> Option <Box<dyn ast::Expression>> {
    //     Some(Box::new(ast::Identifier {
    //         token: self.cur_token.clone(),
    //         value: self.cur_token.literal.clone(),
    //     }))
    // }
    fn parse_statement(&mut self) -> Option <Box<dyn ast::Statement>> {
        match self.cur_token.type_ {
            token::TokenType::LET => self.parse_let_statement(),
            token::TokenType::RETURN => self.prase_return_statement(),
            _ => self.parse_expression_statement(), 
        }
    }
    fn parse_expression_statement(&mut self) -> Option <Box<dyn ast::Statement>> {
        let stmt = ast::ExpressionStatement {
            token: self.cur_token.clone(),
            expression: self.parse_expression(Precedence::LOWEST).unwrap(),
        };
        if self.peek_token_is(token::TokenType::SEMICOLON) {
            self.next_token();
        }
        Some(Box::new(stmt))
    }
    fn parse_integer_literal(&mut self) -> Option <Box<dyn ast::Expression>> {
        let value = self.cur_token.literal.parse::<i64>();
       let value = match value {
            Ok(x) => x,
            Err(_) => {
                self.errors.push(format!("could not parse {:?} as integer", self.cur_token.literal));
                return None;
            },
       };
        let lit = ast::IntegerLiteral {
            token: self.cur_token.clone(),
            value
        };
        Some(Box::new(lit))
    }
  fn parse_expression(&mut self, _precedence: Precedence) -> Option <Box<dyn ast::Expression>> {
    let prefix = self.perfix_parse_fns.get(&self.cur_token.type_);
    match prefix {
        Some(prefix_fn) => prefix_fn(self),
        None => {
            self.errors.push(format!("no prefix parse function for {:?} found", self.cur_token.type_));
            return None;
        },
    }
}
    
    pub fn parse_prefix_expression(&mut self) -> Option<Box<dyn ast::Expression>> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        self.next_token();


        let expresion = ast::PrefixExpression {
            token,
            operator,
            right: self.parse_expression(Precedence::PREFIX).unwrap(),
        };
        Some(Box::new(expresion))
    }

    fn parse_let_statement(&mut self) -> Option <Box<dyn ast::Statement>> {
        let token = self.cur_token.clone();

        if !self.expect_peek(token::TokenType::IDENT) {
            return None;
        }
        let name = ast::Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek(token::TokenType::ASSIGN) {
            return None;
        }   let stmt = ast::LetStatement {
            token,
            name,
            value: Box::new(ast::Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            }),
        };
        while !self.cur_token_is(token::TokenType::SEMICOLON) {
            self.next_token();
        }
     
        Some(Box::new(stmt))
    }
    fn prase_return_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        let token = self.cur_token.clone();
        self.next_token();
        while !self.cur_token_is(token::TokenType::SEMICOLON) {
            self.next_token();
        }
        Some(Box::new(ast::ReturnStatement {
            token,
            return_value: Box::new(ast::Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            }),
        }))
    }

    fn cur_token_is(&self, t: token::TokenType) -> bool {
        self.cur_token.type_ == t
    }
    fn peek_token_is(&self, t: token::TokenType) -> bool {
        self.peek_token.type_ == t
    }
    fn expect_peek(&mut self, t: token::TokenType) -> bool {
        if self.peek_token_is(t.clone()) {
            self.next_token();
            
            true
        } else {
            self.peek_error(t.clone());
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexer::Lexer;
    use crate::ast;
 
    fn test_let_statements(input: &str) {
       
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        
        let program = p.parse_program();
        // check_parser_errors(&p);
        check_parser_errors(&p);
        assert_eq!(program.statements.len(), 3);
        let tests = vec!["x", "y", "foobar"];
        for (i, tt) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            test_let_statement(stmt, tt);
        }
    }
    fn test_let_statement(s: &Box<dyn ast::Statement>, name: &str) {
        assert_eq!(s.token_literal(), "let");
         let let_stmt = match s.as_any().downcast_ref::<ast::LetStatement>() {
            Some(stmt) => stmt,
            None => panic!("s not LetStatement. got={}", s.token_literal()),
        };
        // println!("{:#?}", let_stmt);
        assert_eq!(let_stmt.name.value, name);
        assert_eq!(let_stmt.name.token_literal(), name);

       
    }

    fn check_parser_errors(p: &Parser) {
        let errors = p.errors();
        if errors.len() == 0 {
            return;
        }
        println!("parser has {} errors", errors.len());
        for msg in errors {
            println!("parser error: {}", msg);
        }
        panic!();
    }
    #[test]
    fn test_1(){
        test_let_statements("
        let x = 5;
        let y = 10;
        let foobar = 838383;
        ")
    } 
    #[test]
    #[should_panic]
    fn test_2(){
        test_let_statements("
        let x 5;
        let = 10;
        let 838383;
        ")
    }
    #[test]
    fn test_return_statements() {
        let input = "
        return 5;
        return 10;
        return 993322;
        ";
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);
        assert_eq!(program.statements.len(), 3);
       
        for stmt in program.statements {
            let return_stmt = match stmt.as_any().downcast_ref::<ast::ReturnStatement>() {
                Some(stmt) => stmt,
                None => panic!("s not ReturnStatement. got={}", stmt.token_literal()),
            };
            println!("{:#?}", return_stmt);
            assert_eq!(return_stmt.token_literal(), "return");
        }
    }
  #[test]
    fn test_identifier_expresion() {
        let input = "foobar;";
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);
        assert_eq!(program.statements.len(), 1);
        let stmt = &program.statements[0];
        let ident = match stmt.as_any().downcast_ref::<ast::ExpressionStatement>() {
            Some(stmt) => match stmt.expression.as_any().downcast_ref::<ast::Identifier>() {
                Some(ident) => ident,
                None => panic!("s not Identifier. got={}", stmt.token_literal()),
            },
            None => panic!("s not ExpressionStatement. got={}", stmt.token_literal()),
        };
        assert_eq!(ident.value, "foobar");
        assert_eq!(ident.token_literal(), "foobar");
    }
    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);
        assert_eq!(program.statements.len(), 1);
        let stmt = &program.statements[0];
        let literal = match stmt.as_any().downcast_ref::<ast::ExpressionStatement>() {
            Some(stmt) => match stmt.expression.as_any().downcast_ref::<ast::IntegerLiteral>() {
                Some(literal) => literal,
                None => panic!("s not IntegerLiteral. got={}", stmt.token_literal()),
            },
            None => panic!("s not ExpressionStatement. got={}", stmt.token_literal()),
        };
        assert_eq!(literal.value, 5);
        assert_eq!(literal.token_literal(), "5");
    }
    #[test]
    fn test_perfix_operator() {
        let tests = vec![
            ("!5;", "!", 5),
            ("-15;", "-", 15),
    
        ];
        for (input, operator, value) in tests {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(&p);
            assert_eq!(program.statements.len(), 1);
            let stmt = &program.statements[0];
            let expression = match stmt.as_any().downcast_ref::<ast::ExpressionStatement>() {
                Some(stmt) => match stmt.expression.as_any().downcast_ref::<ast::PrefixExpression>() {
                    Some(expression) => expression,
                    None => panic!("s not PrefixExpression. got={}", stmt.token_literal()),
                },
                None => panic!("s not ExpressionStatement. got={}", stmt.token_literal()),
            };
            assert_eq!(expression.operator, operator);
            match expression.right.as_any().downcast_ref::<ast::IntegerLiteral>() {
                Some(literal) => assert_eq!(literal.value, value),
                None => panic!("s not IntegerLiteral. got={}", stmt.token_literal()),
            };
                
            
         
        }
    }
}
