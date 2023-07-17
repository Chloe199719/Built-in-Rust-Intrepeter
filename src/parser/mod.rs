use crate::token::{Token, self};
use crate::ast;
use crate::lexer::Lexer;
use crate::ast::Node;

#[derive(Clone)]
pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new (lexer: Lexer) -> Parser {
        let mut p = Parser {
            lexer,
            cur_token: Token::new(token::TokenType::EOF, ""),
            peek_token: Token::new(token::TokenType::EOF, ""),
            errors: Vec::new(),
        };
        p.next_token();
        p.next_token();
        p
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
        match self.cur_token.type_ {
            token::TokenType::LET => self.parse_let_statement(),
            _ => None,
        }
    }
    fn parse_let_statement(&mut self) -> Option <Box<dyn ast::Statement>> {
        let token = self.cur_token.clone();
    //   let stmt = ast::LetStatement {
    //       token: self.cur_token.clone(),
    //       name: ast::Identifier {
    //           token: self.cur_token.clone(),
    //           value: self.cur_token.literal.clone(),
    //       },
    //       value: Box::new(ast::Identifier {
    //           token: self.cur_token.clone(),
    //           value: self.cur_token.literal.clone(),
    //       }),
    //   };

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
}