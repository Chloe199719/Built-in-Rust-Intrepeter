use std::io::{BufRead, Write};

use crate::{lexer::Lexer, token::TokenType};

const PROMT: &str = ">> ";

pub fn  start<R: BufRead, W: Write>(reader: &mut R, writer: &mut W) {
    loop {
        write!(writer, "{}", PROMT).unwrap();
        writer.flush().unwrap();
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let mut l = Lexer::new(line);
        let mut tok = l.next_token();
        while tok.type_ != TokenType::EOF {
            write!(writer, "{:?}\n", tok).unwrap();
            tok = l.next_token();
        }
    }
}