use crate::token::{Token, TokenType};
use std::char;
#[derive(Debug, Clone)]
pub struct Lexer {
    input: String,
    position: usize, // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: u8, // current char under examination
}
impl Lexer {
    pub fn new (input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        l
    }
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    pub fn peak_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch {
            b'=' => {
                if self.peak_char() == b'=' {
                    self.read_char();
                    Token::new(TokenType::EQ, "==")
                } else {
                    Token::new(TokenType::ASSIGN, char::from_u32(self.ch as u32).unwrap().to_string().as_str())
                }
            }
            b';' => Token::new(TokenType::SEMICOLON, char::from_u32(self.ch as u32).unwrap().to_string().as_str()),
            b'(' => Token::new(TokenType::LPAREN, char::from_u32(self.ch as u32).unwrap().to_string().as_str()),
            b')' => Token::new(TokenType::RPAREN, char::from_u32(self.ch as u32).unwrap().to_string().as_str()),
            b',' => Token::new(TokenType::COMMA, char::from_u32(self.ch as u32).unwrap().to_string().as_str()),
            b'+' => Token::new(TokenType::PLUS, char::from_u32(self.ch as u32).unwrap().to_string().as_str()),
            b'{' => Token::new(TokenType::LBRACE, char::from_u32(self.ch as u32).unwrap().to_string().as_str()),
            b'}' => Token::new(TokenType::RBRACE, char::from_u32(self.ch as u32).unwrap().to_string().as_str()),
            b'-' => Token::new(TokenType::MINUS, char::from_u32(self.ch as u32).unwrap().to_string().as_str()),
            b'!' => {
                if self.peak_char() == b'=' {
                    self.read_char();
                    Token::new(TokenType::NOT_EQ, "!=")
                } else {
                    Token::new(TokenType::BANG, char::from_u32(self.ch as u32).unwrap().to_string().as_str())
                }
            }
            b'*' => Token::new(TokenType::ASTERISK, char::from_u32(self.ch as u32).unwrap().to_string().as_str()),
            b'/' => Token::new(TokenType::SLASH, char::from_u32(self.ch as u32).unwrap().to_string().as_str()),
            b'<' => Token::new(TokenType::LT, char::from_u32(self.ch as u32).unwrap().to_string().as_str()),
            b'>' => Token::new(TokenType::GT, char::from_u32(self.ch as u32).unwrap().to_string().as_str()),
            0 => Token::new(TokenType::EOF, ""),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let mut ident = String::new();
                while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
                    ident.push(char::from_u32(self.ch as u32).unwrap());
                    self.read_char();
                }
                match ident.as_str() {
                    "fn" => return Token::new(TokenType::FUNCTION, ident.as_str()),
                    "let" => return Token::new(TokenType::LET, ident.as_str()),
                    "true" => return Token::new(TokenType::TRUE, ident.as_str()),
                    "false" => return Token::new(TokenType::FALSE, ident.as_str()),
                    "if" => return Token::new(TokenType::IF, ident.as_str()),
                    "else" => return Token::new(TokenType::ELSE, ident.as_str()),
                    "return" => return Token::new(TokenType::RETURN, ident.as_str()),
                    _ => return Token::new(TokenType::IDENT, ident.as_str()),
                    
                }                   
                
            },
            b'0'..=b'9' => {
                let mut number = String::new();
                while self.ch.is_ascii_digit() {
                    number.push(char::from_u32(self.ch as u32).unwrap());
                    self.read_char();
                }
                return Token::new(TokenType::INT, number.as_str());
            },
            b'"' => {
                let mut string = String::new();
                self.read_char();
                while self.ch != b'"' && self.ch != 0 {
                    string.push(char::from_u32(self.ch as u32).unwrap());
                    self.read_char();
                }
                self.read_char();
                return Token::new(TokenType::STRING, string.as_str());
            },
            _ => Token::new(TokenType::ILLEGAL, char::from_u32(self.ch as u32).unwrap().to_string().as_str()),
        };
        self.read_char();
        tok
    }
    pub fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }
} 





#[cfg(test)]
mod test {
    use crate::token::TokenType;

    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let tests = vec![
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::PLUS, "+"),
            Token::new(TokenType::LPAREN, "("),
            Token::new(TokenType::RPAREN, ")"),
            Token::new(TokenType::LBRACE, "{"),
            Token::new(TokenType::RBRACE, "}"),
            Token::new(TokenType::COMMA, ","),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::EOF, ""),
        ];

        let mut l = Lexer::new(input.to_string());

        for tt in tests {
            let tok = l.next_token();
            assert_eq!(tok.type_, tt.type_);
            assert_eq!(tok.literal, tt.literal);
        }
    }

    #[test]
    fn test_next_token2(){
        let input = "let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        ";
        let tests = vec![
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "five"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::INT, "5"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "ten"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::INT, "10"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "add"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::FUNCTION, "fn"),
            Token::new(TokenType::LPAREN, "("),
            Token::new(TokenType::IDENT, "x"),
            Token::new(TokenType::COMMA, ","),
            Token::new(TokenType::IDENT, "y"),
            Token::new(TokenType::RPAREN, ")"),
            Token::new(TokenType::LBRACE, "{"),
            Token::new(TokenType::IDENT, "x"),
            Token::new(TokenType::PLUS, "+"),
            Token::new(TokenType::IDENT, "y"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::RBRACE, "}"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "result"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::IDENT, "add"),
            Token::new(TokenType::LPAREN, "("),
            Token::new(TokenType::IDENT, "five"),
            Token::new(TokenType::COMMA, ","),
            Token::new(TokenType::IDENT, "ten"),
            Token::new(TokenType::RPAREN, ")"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::EOF, ""),
        ];
        let mut l = Lexer::new(input.to_string());
        for tt in tests {
            let tok = l.next_token();
            assert_eq!(tok.type_, tt.type_);
            assert_eq!(tok.literal, tt.literal);
        }
    }
    #[test]
    fn test_next_token3(){
        let input = " let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
        ";
        let tests = vec![
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "five"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::INT, "5"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "ten"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::INT, "10"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "add"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::FUNCTION, "fn"),
            Token::new(TokenType::LPAREN, "("),
            Token::new(TokenType::IDENT, "x"),
            Token::new(TokenType::COMMA, ","),
            Token::new(TokenType::IDENT, "y"),
            Token::new(TokenType::RPAREN, ")"),
            Token::new(TokenType::LBRACE, "{"),
            Token::new(TokenType::IDENT, "x"),
            Token::new(TokenType::PLUS, "+"),
            Token::new(TokenType::IDENT, "y"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::RBRACE, "}"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "result"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::IDENT, "add"),
            Token::new(TokenType::LPAREN, "("),
            Token::new(TokenType::IDENT, "five"),
            Token::new(TokenType::COMMA, ","),
            Token::new(TokenType::IDENT, "ten"),
            Token::new(TokenType::RPAREN, ")"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::BANG, "!"),
            Token::new(TokenType::MINUS, "-"),
            Token::new(TokenType::SLASH, "/"),
            Token::new(TokenType::ASTERISK, "*"),
            Token::new(TokenType::INT, "5"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::INT, "5"),
            Token::new(TokenType::LT, "<"),
            Token::new(TokenType::INT, "10"),
            Token::new(TokenType::GT, ">"),
            Token::new(TokenType::INT, "5"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::EOF, ""),
        ];
        let mut l = Lexer::new(input.to_string());
        for tt in tests {
            let tok = l.next_token();
            // println!("tok: {:?}", tok);
            // println!("tt: {:?}", tt);
            assert_eq!(tok.type_, tt.type_);
            assert_eq!(tok.literal, tt.literal);
        }
    }
    #[test]
    fn test_next_token4 (){
        let input = "
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
        if (5 < 10) {
            return true;
        } else {
            return false;
        }";
        let tests = vec![
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "five"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::INT, "5"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "ten"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::INT, "10"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "add"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::FUNCTION, "fn"),
            Token::new(TokenType::LPAREN, "("),
            Token::new(TokenType::IDENT, "x"),
            Token::new(TokenType::COMMA, ","),
            Token::new(TokenType::IDENT, "y"),
            Token::new(TokenType::RPAREN, ")"),
            Token::new(TokenType::LBRACE, "{"),
            Token::new(TokenType::IDENT, "x"),
            Token::new(TokenType::PLUS, "+"),
            Token::new(TokenType::IDENT, "y"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::RBRACE, "}"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "result"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::IDENT, "add"),
            Token::new(TokenType::LPAREN, "("),
            Token::new(TokenType::IDENT, "five"),
            Token::new(TokenType::COMMA, ","),
            Token::new(TokenType::IDENT, "ten"),
            Token::new(TokenType::RPAREN, ")"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::BANG, "!"),
            Token::new(TokenType::MINUS, "-"),
            Token::new(TokenType::SLASH, "/"),
            Token::new(TokenType::ASTERISK, "*"),
            Token::new(TokenType::INT, "5"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::INT, "5"),
            Token::new(TokenType::LT, "<"),
            Token::new(TokenType::INT, "10"),
            Token::new(TokenType::GT, ">"),
            Token::new(TokenType::INT, "5"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::IF, "if"),
            Token::new(TokenType::LPAREN, "("),
            Token::new(TokenType::INT, "5"),
            Token::new(TokenType::LT, "<"),
            Token::new(TokenType::INT, "10"),
            Token::new(TokenType::RPAREN, ")"),
            Token::new(TokenType::LBRACE, "{"),
            Token::new(TokenType::RETURN, "return"),
            Token::new(TokenType::TRUE, "true"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::RBRACE, "}"),
            Token::new(TokenType::ELSE, "else"),
            Token::new(TokenType::LBRACE, "{"),
            Token::new(TokenType::RETURN, "return"),
            Token::new(TokenType::FALSE, "false"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::RBRACE, "}"),
            Token::new(TokenType::EOF, ""),
        ];
        let mut l = Lexer::new(input.to_string());
        for tt in tests {
            let tok = l.next_token();
            // println!("tok: {:?}", tok);
            // println!("tt: {:?}", tt);
            assert_eq!(tok.type_, tt.type_);
            assert_eq!(tok.literal, tt.literal);
        }
    }
    #[test]
    fn test_next_token5 (){
        let input = r#"
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        10 == 10
        10 != 9
        "foobar"
        "foo bar"
        "#;
        let tests = vec![
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "five"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::INT, "5"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "ten"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::INT, "10"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "add"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::FUNCTION, "fn"),
            Token::new(TokenType::LPAREN, "("),
            Token::new(TokenType::IDENT, "x"),
            Token::new(TokenType::COMMA, ","),
            Token::new(TokenType::IDENT, "y"),
            Token::new(TokenType::RPAREN, ")"),
            Token::new(TokenType::LBRACE, "{"),
            Token::new(TokenType::IDENT, "x"),
            Token::new(TokenType::PLUS, "+"),
            Token::new(TokenType::IDENT, "y"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::RBRACE, "}"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "result"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::IDENT, "add"),
            Token::new(TokenType::LPAREN, "("),
            Token::new(TokenType::IDENT, "five"),
            Token::new(TokenType::COMMA, ","),
            Token::new(TokenType::IDENT, "ten"),
            Token::new(TokenType::RPAREN, ")"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::BANG, "!"),
            Token::new(TokenType::MINUS, "-"),
            Token::new(TokenType::SLASH, "/"),
            Token::new(TokenType::ASTERISK, "*"),
            Token::new(TokenType::INT, "5"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::INT, "5"),
            Token::new(TokenType::LT, "<"),
            Token::new(TokenType::INT, "10"),
            Token::new(TokenType::GT, ">"),
            Token::new(TokenType::INT, "5"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::IF, "if"),
            Token::new(TokenType::LPAREN, "("),
            Token::new(TokenType::INT, "5"),
            Token::new(TokenType::LT, "<"),
            Token::new(TokenType::INT, "10"),
            Token::new(TokenType::RPAREN, ")"),
            Token::new(TokenType::LBRACE, "{"),
            Token::new(TokenType::RETURN, "return"),
            Token::new(TokenType::TRUE, "true"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::RBRACE, "}"),
            Token::new(TokenType::ELSE, "else"),
            Token::new(TokenType::LBRACE, "{"),
            Token::new(TokenType::RETURN, "return"),
            Token::new(TokenType::FALSE, "false"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::RBRACE, "}"),
            Token::new(TokenType::INT, "10"),
            Token::new(TokenType::EQ, "=="),
            Token::new(TokenType::INT, "10"),
            Token::new(TokenType::INT, "10"),
            Token::new(TokenType::NOT_EQ, "!="),
            Token::new(TokenType::INT, "9"),
            Token::new(TokenType::STRING, "foobar"),
            Token::new(TokenType::STRING, "foo bar"),
            Token::new(TokenType::EOF, ""),
        ];
        let mut l = Lexer::new(input.to_string());
        for tt in tests {
            let tok = l.next_token();
            // println!("tok: {:?}", tok);
            // println!("tt: {:?}", tt);
            assert_eq!(tok.type_, tt.type_);
            assert_eq!(tok.literal, tt.literal);
        }
    }
    

    
}