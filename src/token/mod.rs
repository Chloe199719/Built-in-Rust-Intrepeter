#[allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT, // add, foobar, x, y, ...
    INT, // 1343456
    
    // Operators
    ASSIGN, // =
    PLUS, // +
    MINUS, // -
    BANG, // !
    ASTERISK, // *
    SLASH, // /

    LT, // <
    GT, // >

    EQ, // ==
    NOT_EQ, // !=


    // Delimiters

    COMMA, // ,
    SEMICOLON, // ;

    LPAREN, // (
    RPAREN, // )
    LBRACE, // {
    RBRACE, // }

    // Keywords
    FUNCTION, // fn
    LET, // let
    TRUE, // true
    FALSE, // false
    IF, // if
    ELSE, // else
    RETURN, // return


}
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub type_: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(type_: TokenType, literal: &str) -> Token {
        Token {
            type_,
            literal: literal.to_string(),
        }
    }   
}