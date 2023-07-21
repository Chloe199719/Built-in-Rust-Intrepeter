use std::io::{BufRead, Write};
use crate::evaluator;
use crate::{lexer::Lexer,  parser:: Parser};
const MONKEY_FACE:&str = r#" 
         __,__
   .--. .-" "-. .--.
  / .. \/ .-. .-. \/ .. \
 | | '| / Y \ |' | |
 | \ \ \ 0 | 0 / / / |
 \ '- ,\.-"""""""-./, -' /
''-' /_ ^ ^ _\ '-''
| \._ _./ |
\ \ '~' / /
'._ '-=-' _.'
'-----'
`
"#;
const PROMT: &str = ">> ";

pub fn  start<R: BufRead, W: Write>(reader: &mut R, writer: &mut W) {
    loop {
        write!(writer, "{}", PROMT).unwrap();
        writer.flush().unwrap();
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let  l = Lexer::new(line);
        let mut parser = Parser::new( l);
        let program = parser.parse_program();
        if parser.errors.len() != 0 {
            print_parse_errors(writer, parser.errors);
            continue;
        }

        let evaluated =  evaluator::eval(&program);

      
            write!(writer, "{}", evaluated.inspect()).unwrap();
            write!(writer, "\n").unwrap();
        

       
    }
}

fn print_parse_errors<W: Write>(writer: &mut W, errors: Vec<String>) {
    write!(writer, "{}", MONKEY_FACE).unwrap();
    write!(writer, "Woops! We ran into some monkey business here!\n").unwrap();
    for error in errors {
        write!(writer, "\t{}\n", error).unwrap();
    }
}