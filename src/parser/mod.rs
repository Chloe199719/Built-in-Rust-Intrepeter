#![allow(dead_code)]
#![allow(unused_imports)]
use std::collections::HashMap;
use std::rc::Rc;

use crate::token::{Token, self};
use crate::ast;
use crate::lexer::Lexer;
use crate::ast::Node;


pub type PrefixParseFn = fn(&mut Parser) -> Option<Box<dyn ast::Expression>>;
type InfixParseFn = fn(&mut Parser,Option<Box<dyn ast::Expression>>) -> Option<Box<dyn ast::Expression>>;

#[derive(Clone)]
pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
  pub  errors: Vec<String>,
    precedence: HashMap<token::TokenType, Precedence>,
    perfix_parse_fns: HashMap<token::TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<token::TokenType, InfixParseFn>,
}

#[derive(Clone,PartialEq,PartialOrd,)]
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
            precedence: HashMap::new(),
            perfix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };
        p.create_precedences_map();
        p.register_prefix(token::TokenType::IDENT, Parser::parse_identifier_expression);
        p.register_prefix(token::TokenType::INT, Parser::parse_integer_literal);
        p.register_prefix(token::TokenType::BANG, Parser::parse_prefix_expression);
        p.register_prefix(token::TokenType::MINUS, Parser::parse_prefix_expression);
        p.register_prefix(token::TokenType::TRUE, Parser::parse_boolean);
        p.register_prefix(token::TokenType::FALSE, Parser::parse_boolean);
        p.register_prefix(token::TokenType::LPAREN, Parser::parse_grouped_expression);
        p.register_prefix(token::TokenType::IF, Parser::parse_if_expression);
        p.register_prefix(token::TokenType::FUNCTION, Parser::parse_function_literal);

        p.register_infix(token::TokenType::MINUS, Parser::parse_infix_expression);
        p.register_infix(token::TokenType::PLUS, Parser::parse_infix_expression);
        p.register_infix(token::TokenType::SLASH, Parser::parse_infix_expression);
        p.register_infix(token::TokenType::ASTERISK, Parser::parse_infix_expression);
        p.register_infix(token::TokenType::EQ, Parser::parse_infix_expression);
        p.register_infix(token::TokenType::NOT_EQ, Parser::parse_infix_expression);
        p.register_infix(token::TokenType::LT, Parser::parse_infix_expression);
        p.register_infix(token::TokenType::GT, Parser::parse_infix_expression);
        p.register_infix(token::TokenType::LPAREN, Parser::parse_call_expression);
          

        p.next_token();
        p.next_token();
        p
    }
    
    fn create_precedences_map (&mut self){
        self.precedence.insert(token::TokenType::EQ, Precedence::EQUALS);
        self.precedence.insert(token::TokenType::NOT_EQ, Precedence::EQUALS);
        self.precedence.insert(token::TokenType::LT, Precedence::LESSGREATER);
        self.precedence.insert(token::TokenType::GT, Precedence::LESSGREATER);
        self.precedence.insert(token::TokenType::PLUS, Precedence::SUM);
        self.precedence.insert(token::TokenType::MINUS, Precedence::SUM);
        self.precedence.insert(token::TokenType::SLASH, Precedence::PRODUCT);
        self.precedence.insert(token::TokenType::ASTERISK, Precedence::PRODUCT);
        self.precedence.insert(token::TokenType::LPAREN, Precedence::CALL);
    }

    fn register_prefix(&mut self, token_type: token::TokenType, parse_fn: fn(&mut Parser) -> Option<Box<dyn ast::Expression>>) {
        self.perfix_parse_fns.insert(token_type, parse_fn);
    }

    pub fn register_infix(&mut self, token_type: token::TokenType, func:fn(&mut Parser,Option<Box<dyn ast::Expression>>) -> Option<Box<dyn ast::Expression>> ) {
        self.infix_parse_fns.insert(token_type, func);
    }

    fn peek_precedence(&self) -> Precedence {
        match self.precedence.get(&self.peek_token.type_) {
            Some(p) => p.clone(),                                                                                                                                                                                                                   
            None => Precedence::LOWEST,
        }                                                                                                               
    }

    fn parse_infix_expression(&mut self, left: Option<Box<dyn ast::Expression>>) -> Option<Box<dyn ast::Expression>> {
    
         let  token = self.cur_token.clone();
        let  operator = self.cur_token.literal.clone();
       
        let precedence = self.cur_precedence();
        self.next_token();
       let expresion_right = self.parse_expression(precedence).unwrap();
       let expression = ast::InfixExpression {
           token,
           operator,
           left: left.unwrap(),
           right: expresion_right,
        };
        Some(Box::new(expression))
    }

    fn cur_precedence(&self) -> Precedence {
        match self.precedence.get(&self.cur_token.type_) {
            Some(p) => p.clone(),
            None => Precedence::LOWEST,
        }
    }
    fn parse_call_expression(&mut self, function: Option<Box<dyn ast::Expression>>) -> Option<Box<dyn ast::Expression>> {
        let token = self.cur_token.clone();
        let arguments = self.parse_call_arguments();
        let expression = ast::CallExpression {
            token,
            function: function.unwrap(),
            arguments,
        };
        Some(Box::new(expression))
    }

    fn parse_call_arguments(&mut self) -> Vec<Box<dyn ast::Expression>> {
        let mut args = Vec::new();
        if self.peek_token_is(token::TokenType::RPAREN) {
            self.next_token();
            return args;
        }
        self.next_token();
        args.push(self.parse_expression(Precedence::LOWEST).unwrap());
        while self.peek_token_is(token::TokenType::COMMA) {
            self.next_token();
            self.next_token();
            args.push(self.parse_expression(Precedence::LOWEST).unwrap());
        }
        if !self.expect_peek(token::TokenType::RPAREN) {
            return Vec::new();
        }
        args
    }


    fn parse_function_literal(&mut self) -> Option<Box<dyn ast::Expression>> {
        let token = self.cur_token.clone();
        if !self.expect_peek(token::TokenType::LPAREN) {
            return None;
        }
        let parameters = self.parse_function_parameters();
        if !self.expect_peek(token::TokenType::LBRACE) {
            return None;
        }
        let body = self.parse_block_statement().unwrap();
        let expression = ast::FunctionLiteral {
            token,
            parameters,
            body: Rc::new(body),
        };
        Some(Box::new(expression))
    }

    fn parse_function_parameters(&mut self) -> Vec<ast::Identifier> {
        let mut identifiers = Vec::new();
        if self.peek_token_is(token::TokenType::RPAREN) {
            self.next_token();
            return identifiers;
        }
        self.next_token();
        let ident = ast::Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };
        identifiers.push(ident);
        while self.peek_token_is(token::TokenType::COMMA) {
            self.next_token();
            self.next_token();
            let ident = ast::Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            };
            identifiers.push(ident);
        }
        if !self.expect_peek(token::TokenType::RPAREN) {
            return Vec::new();
        }
        identifiers
    }

    fn parse_if_expression(&mut self) -> Option<Box<dyn ast::Expression>> {
        let token = self.cur_token.clone();
        if !self.expect_peek(token::TokenType::LPAREN) {
            return None;
        }
        self.next_token();
        let condition = self.parse_expression(Precedence::LOWEST);
        if !self.expect_peek(token::TokenType::RPAREN) {
            return None;
        }
        if !self.expect_peek(token::TokenType::LBRACE) {
            return None;
        }
        let consequence = self.parse_block_statement();
        let mut alternative = None;
        if self.peek_token_is(token::TokenType::ELSE) {
            self.next_token();
            if !self.expect_peek(token::TokenType::LBRACE) {
                return None;
            }
            alternative = self.parse_block_statement();
        }
        Some(Box::new(ast::IfExpression {
            token,
            condition: condition.unwrap(),
            consequence:   consequence.unwrap(),
            alternative,
        }))
    }
    fn parse_block_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        let token = self.cur_token.clone();
        let mut statements = Vec::new();
        self.next_token();
        while !self.cur_token_is(token::TokenType::RBRACE) && !self.cur_token_is(token::TokenType::EOF) {
            let stmt = self.parse_statement();
            if stmt.is_some() {
                statements.push(stmt.unwrap());
            }
            self.next_token();
        }
        Some(Box::new(ast::BlockStatement {
            token,
            statements,
        }))
    }

    fn parse_grouped_expression(&mut self) -> Option<Box<dyn ast::Expression>> {
        self.next_token();
        let exp = self.parse_expression(Precedence::LOWEST);
        if !self.expect_peek(token::TokenType::RPAREN) {
            return None;
        }
        exp
    }

    fn parse_boolean(&mut self) -> Option<Box<dyn ast::Expression>> {
        Some(Box::new(ast::Boolean {
            token: self.cur_token.clone(),
            value: self.cur_token_is(token::TokenType::TRUE),
        }))
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

    fn parse_statement(&mut self) -> Option <Box<dyn ast::Statement>> {
        // println!("parse_statement: {:?}", self.cur_token.type_);
        match self.cur_token.type_ {
            token::TokenType::LET => self.parse_let_statement(),
            token::TokenType::RETURN => self.prase_return_statement(),
            _ => self.parse_expression_statement(), 
        }
    }

    fn parse_expression_statement(&mut self) -> Option <Box<dyn ast::Statement>> {
        // println!("parse_expression_statement: {:?}", self.cur_token.type_);
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

    fn parse_expression(&mut self, precedence: Precedence) -> Option <Box<dyn ast::Expression>> {
        // println!("parse_expression: {:?}", self.cur_token.type_);
        let prefix = self.perfix_parse_fns.get(&self.cur_token.type_);
        let mut left_expression = match prefix {
            Some(prefix_fn) => prefix_fn(self),
            None => {
                // println!("no prefix parse function for {:?} found", self.cur_token.type_);
                self.errors.push(format!("no prefix parse function for {:?} found", self.cur_token.type_));
                return None;
            },
        };
        while !self.peek_token_is(token::TokenType::SEMICOLON) && precedence < self.peek_precedence() {
            let copy_self = self.clone();
            let peek_token = self.peek_token.type_.clone();
            let infix_fn = match copy_self.infix_parse_fns.get(&peek_token){
                Some(infix_fn) => infix_fn,
                None => return left_expression,
            };
            self.next_token();
            left_expression = infix_fn(self, left_expression);
        }
        left_expression
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
        }
        self.next_token();

         let stmt = ast::LetStatement {
            token,
            name,
            value: self.parse_expression(Precedence::LOWEST).unwrap(),
        };
        if self.peek_token_is(token::TokenType::SEMICOLON) {
            self.next_token();
        };
   
     
        Some(Box::new(stmt))
    }
    
    fn prase_return_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        let token = self.cur_token.clone();
        self.next_token();
        let stmt = ast::ReturnStatement {
            token,
            return_value: self.parse_expression(Precedence::LOWEST).unwrap(),
        };
        while !self.cur_token_is(token::TokenType::SEMICOLON) {
            self.next_token();
        }
        Some(Box::new(stmt))
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
    pub enum Literal {
        IntegerLiteral(i64),
        Boolean(bool),        
    }

    fn test_literal_expresion(x: &Box<dyn ast::Expression>, expected: Literal){
        match expected {
            Literal::IntegerLiteral(i) => {
                match x.as_any().downcast_ref::<ast::IntegerLiteral>() {
                    Some(literal) => assert_eq!(literal.value, i),
                    None => panic!("s not IntegerLiteral. got={}", x.token_literal()),
                };
            }
            Literal::Boolean(b) => {
                match x.as_any().downcast_ref::<ast::Boolean>() {
                    Some(literal) => assert_eq!(literal.value, b),
                    None => panic!("s not IntegerLiteral. got={}", x.token_literal()),
                };
            }
        }
    }

    fn test_infix_expression(x: &dyn ast::Expression, left: i64, operator: String, right: i64){
        let infix = match x.as_any().downcast_ref::<ast::InfixExpression>() {
            Some(infix) => infix,
            None => panic!("s not InfixExpression. got={}", x.token_literal()),
        };

        test_literal_expresion(&infix.left, Literal::IntegerLiteral(left));
        assert_eq!(infix.operator, operator);
        test_literal_expresion(&infix.right, Literal::IntegerLiteral(right));
    }

    fn test_identifier(x:&dyn ast::Expression, expected: String){
        let ident = match x.as_any().downcast_ref::<ast::Identifier>() {
            Some(ident) => ident,
            None => panic!("s not Identifier. got={}", x.token_literal()),
        };
        assert_eq!(ident.value, expected);
        assert_eq!(ident.token_literal(), expected);
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

    fn test_let_statements() {
        let input = vec![("let x = 5;", "x", 5), ("let y = 10;", "y", 10), ("let foobar = 838383;", "foobar", 838383)];
        for (_, tt) in input.iter().enumerate() {
            let l = Lexer::new(tt.0.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(&p);
            assert_eq!(program.statements.len(), 1);
            let stmt = &program.statements[0];
            test_let_statement(stmt, tt.1);
        }

    }
    // #[test]
    // #[should_panic]
    // fn test_2(){
    //     test_let_statements("
    //     let x 5;
    //     let = 10;
    //     let 838383;
    //     ")
    // }
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
            // println!("{:#?}", return_stmt);
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
    #[test]
    fn test_parsing_inflix_expresion(){
        let tests = vec![
            ("5 + 5;", 5, "+", 5),
            ("5 - 5;", 5, "-", 5),
            ("5 * 5;", 5, "*", 5),
            ("5 / 5;", 5, "/", 5),
            ("5 > 5;", 5, ">", 5),
            ("5 < 5;", 5, "<", 5),
            ("5 == 5;", 5, "==", 5),
            ("5 != 5;", 5, "!=", 5),
        ];
        for (input, left, operator, right) in tests {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(&p);
            assert_eq!(program.statements.len(), 1);
            // println!("{:#?}", program);
            let stmt = &program.statements[0];
            let expression = match stmt.as_any().downcast_ref::<ast::ExpressionStatement>() {
                Some(stmt) => match stmt.expression.as_any().downcast_ref::<ast::InfixExpression>() {
                    Some(expression) => expression,
                    None => panic!("s not InfixExpression. got={}", stmt.token_literal()),
                },
                None => panic!("s not ExpressionStatement. got={}", stmt.token_literal()),
            };
            match expression.left.as_any().downcast_ref::<ast::IntegerLiteral>() {
                Some(literal) => assert_eq!(literal.value, left),
                None => panic!("s not IntegerLiteral. got={}", stmt.token_literal()),
            };
            assert_eq!(expression.operator, operator);
            match expression.right.as_any().downcast_ref::<ast::IntegerLiteral>() {
                Some(literal) => assert_eq!(literal.value, right),
                None => panic!("s not IntegerLiteral. got={}", stmt.token_literal()),
            };
                
            
         
        }
    }
    #[test]
    fn test_operator_precedence_parsing(){
        let tests = vec![
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            ("3 + 4 * 5 == 3 * 1 + 4 * 5", "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"),
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            ("(5 + 5) * 2", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
            ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
            ("add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))", "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))"),
            ("add(a + b + c * d / f + g)", "add((((a + b) + ((c * d) / f)) + g))"),
           


        ];
        for (input, expected) in tests {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program();
            // println!("{:#?}", program.string());
            check_parser_errors(&p);
            assert_eq!(program.string(), expected);
        }
    }
    #[test]
    fn test_boolean_expression(){
        let tests = vec![
            ("true;", true),
            ("false;", false),
        ];
        for (input, value) in tests {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(&p);
            assert_eq!(program.statements.len(), 1);
            let stmt = &program.statements[0];
            let expression = match stmt.as_any().downcast_ref::<ast::ExpressionStatement>() {
                Some(stmt) => match stmt.expression.as_any().downcast_ref::<ast::Boolean>() {
                    Some(expression) => expression,
                    None => panic!("s not Boolean. got={}", stmt.token_literal()),
                },
                None => panic!("s not ExpressionStatement. got={}", stmt.token_literal()),
            };
            assert_eq!(expression.value, value);
        }
    }
    #[test]
    fn test_parsing_inflix_expresions(){
        let tests = vec![
            ("true", "true"),
            ("false", "false"),
            ("3 > 5", "(3 > 5)"),
            ("3 < 5", "(3 < 5)"),
            ("3 == 5", "(3 == 5)"),
            ("3 != 5", "(3 != 5)"),
            ("true == true", "(true == true)"),
            ("true != false", "(true != false)"),
            ("false == false", "(false == false)"),
        ];
        for (input, expected) in tests {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program();
            // println!("{:#?}", program.string());
            check_parser_errors(&p);
            assert_eq!(program.string(), expected);
        }
    }
    #[test]
    fn test_if_expression(){
        let input = "if (x < y) { x }";
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);
        assert_eq!(program.statements.len(), 1);
        let stmt = &program.statements[0];
        let expression = match stmt.as_any().downcast_ref::<ast::ExpressionStatement>() {
            Some(stmt) => match stmt.expression.as_any().downcast_ref::<ast::IfExpression>() {
                Some(expression) => expression,
                None => panic!("s not IfExpression. got={}", stmt.token_literal()),
            },
            None => panic!("s not ExpressionStatement. got={}", stmt.token_literal()),
        };
        assert_eq!(expression.condition.string(), "(x < y)");
        let consequence = match expression.consequence.as_any().downcast_ref::<ast::BlockStatement>() {
            Some(stmt) => stmt,
            None => panic!("s not BlockStatement. got={}", stmt.token_literal()),
        
            
        };
        assert_eq!(consequence.statements.len(), 1);
        let consequence = &consequence.statements[0];
        let consequence = match consequence.as_any().downcast_ref::<ast::ExpressionStatement>() {
            Some(stmt) => match stmt.expression.as_any().downcast_ref::<ast::Identifier>() {
                Some(expression) => expression,
                None => panic!("s not Identifier. got={}", stmt.token_literal()),
            },
            None => panic!("s not ExpressionStatement. got={}", stmt.token_literal()),
        };
        assert_eq!(consequence.value, "x");
        match expression.alternative {
            Some(_) => panic!("expression.alternative was not None. got="),
            None => {}
        }
            
        }
        
    #[test]
    fn test_if_else_expression(){
        let input = "if (x < y) { x } else { y }";
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);
        assert_eq!(program.statements.len(), 1);
        let stmt = &program.statements[0];
        let expression = match stmt.as_any().downcast_ref::<ast::ExpressionStatement>() {
            Some(stmt) => match stmt.expression.as_any().downcast_ref::<ast::IfExpression>() {
                Some(expression) => expression,
                None => panic!("s not IfExpression. got={}", stmt.token_literal()),
            },
            None => panic!("s not ExpressionStatement. got={}", stmt.token_literal()),
        };
        assert_eq!(expression.condition.string(), "(x < y)");
        let consequence = match expression.consequence.as_any().downcast_ref::<ast::BlockStatement>() {
            Some(stmt) => stmt,
            None => panic!("s not BlockStatement. got={}", stmt.token_literal()),
        
            
        };
        assert_eq!(consequence.statements.len(), 1);
        let consequence = &consequence.statements[0];
        let consequence = match consequence.as_any().downcast_ref::<ast::ExpressionStatement>() {
            Some(stmt) => match stmt.expression.as_any().downcast_ref::<ast::Identifier>() {
                Some(expression) => expression,
                None => panic!("s not Identifier. got={}", stmt.token_literal()),
            },
            None => panic!("s not ExpressionStatement. got={}", stmt.token_literal()),
        };
        assert_eq!(consequence.value, "x");
        let alternative = match expression.alternative.as_ref().unwrap().as_any().downcast_ref::<ast::BlockStatement>() {
            Some(stmt) => stmt,
            None => panic!("s not BlockStatement. got={}", stmt.token_literal()),
        
            
        };
        assert_eq!(alternative.statements.len(), 1);
        let alternative = &alternative.statements[0];
        let alternative = match alternative.as_any().downcast_ref::<ast::ExpressionStatement>() {
            Some(stmt) => match stmt.expression.as_any().downcast_ref::<ast::Identifier>() {
                Some(expression) => expression,
                None => panic!("s not Identifier. got={}", stmt.token_literal()),
            },
            None => panic!("s not ExpressionStatement. got={}", stmt.token_literal()),
        };
        assert_eq!(alternative.value, "y");
            
        
    }
    
    #[test]
    fn test_function_literal_parsing(){
        let input = "fn(x, y) { x + y; }";
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);
        assert_eq!(program.statements.len(), 1);
        let stmt = &program.statements[0];
        let expression = match stmt.as_any().downcast_ref::<ast::ExpressionStatement>() {
            Some(stmt) => match stmt.expression.as_any().downcast_ref::<ast::FunctionLiteral>() {
                Some(expression) => expression,
                None => panic!("s not FunctionLiteral. got={}", stmt.token_literal()),
            },
            None => panic!("s not ExpressionStatement. got={}", stmt.token_literal()),
        };
        assert_eq!(expression.parameters.len(), 2);
        assert_eq!(expression.parameters[0].string(), "x");
        assert_eq!(expression.parameters[1].string(), "y");
        let body = match expression.body.as_any().downcast_ref::<ast::BlockStatement>() {
            Some(stmt) => stmt,
            None => panic!("s not BlockStatement. got={}", stmt.token_literal()),
        
            
        };
        assert_eq!(body.statements.len(), 1);
        let body = &body.statements[0];
        let body = match body.as_any().downcast_ref::<ast::ExpressionStatement>() {
            Some(stmt) => match stmt.expression.as_any().downcast_ref::<ast::InfixExpression>() {
                Some(expression) => expression,
                None => panic!("s not InfixExpression. got={}", stmt.token_literal()),
            },
            None => panic!("s not ExpressionStatement. got={}", stmt.token_literal()),
        };
        assert_eq!(body.string(), "(x + y)");
    }
    #[test]
    fn test_function_parameters_parsing(){
        let tests = vec![
            ("fn() {};", vec![]),
            ("fn(x) {};", vec!["x"]),
            ("fn(x, y, z) {};", vec!["x", "y", "z"]),
        ];
        for (input, expected_params) in tests {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(&p);
            let stmt = &program.statements[0];
            let expression = match stmt.as_any().downcast_ref::<ast::ExpressionStatement>() {
                Some(stmt) => match stmt.expression.as_any().downcast_ref::<ast::FunctionLiteral>() {
                    Some(expression) => expression,
                    None => panic!("s not FunctionLiteral. got={}", stmt.token_literal()),
                },
                None => panic!("s not ExpressionStatement. got={}", stmt.token_literal()),
            };
            assert_eq!(expression.parameters.len(), expected_params.len());
            for (i, ident) in expected_params.iter().enumerate() {
                assert_eq!(expression.parameters[i].string(), *ident);
            }
        }
    }
    #[test]
    fn test_call_expression_parsing(){
        let input = "add(1, 2 * 3, 4 + 5);";
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);
        assert_eq!(program.statements.len(), 1);
        let stmt = &program.statements[0];
        let expression = match stmt.as_any().downcast_ref::<ast::ExpressionStatement>() {
            Some(stmt) => match stmt.expression.as_any().downcast_ref::<ast::CallExpression>() {
                Some(expression) => expression,
                None => panic!("s not CallExpression. got={}", stmt.token_literal()),
            },
            None => panic!("s not ExpressionStatement. got={}", stmt.token_literal()),
        };
        assert_eq!(expression.function.string(), "add");
        assert_eq!(expression.arguments.len(), 3);
        assert_eq!(expression.arguments[0].string(), "1");
        assert_eq!(expression.arguments[1].string(), "(2 * 3)");
        assert_eq!(expression.arguments[2].string(), "(4 + 5)");
    }

    #[test]
    fn test_program(){
        let input = "let x = 2;
        let y = 3;
        let foobar = 1234567890;
        x + y + foobar;
        let x = fn(x, y) {
            x + y;
        };
        ";
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);
        assert_eq!(program.statements.len(), 5);
        println!("{:#?}", program);

    }

  
}
