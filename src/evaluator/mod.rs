use crate::ast;
use crate::ast::Statement;
use crate::object;
use crate::ast::Node;

    pub fn eval(node: &dyn Node) -> Box<dyn object::Object> {
        match node.node_type() {
            ast::NodeType::Program => eval_statements(&node.as_any().downcast_ref::<ast::Program>().unwrap().statements),
            ast::NodeType::ExpressionStatement => eval(node.as_any().downcast_ref::<ast::ExpressionStatement>().unwrap().expression.as_node()),
            ast::NodeType::IntegerLiteral =>Box::new(object::Integer{value:node.as_any().downcast_ref::<ast::IntegerLiteral>().unwrap().value}),
            _ => panic!("Not implemented yet")
            
        }
    }
    pub  fn eval_statements(statements: &Vec<Box<dyn Statement>>) -> Box<dyn object::Object> {
        let mut result: Box<dyn object::Object> = Box::new(object::Null{});
        for statement in statements {
            // println!("{:?}", statement);
            result = eval(statement.as_node());
        }
        return result;
    }



#[cfg(test)]

mod test {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::object;


    #[test]
    fn test_eval_integer_expression(){
        let tests = vec![
            ("5", 5),
            ("10", 10),
            // ("-5", -5),
            // ("-10", -10),
            // ("5 + 5 + 5 + 5 - 10", 10),
            // ("2 * 2 * 2 * 2 * 2", 32),
            // ("-50 + 100 + -50", 0),
            // ("5 * 2 + 10", 20),
            // ("5 + 2 * 10", 25),
            // ("20 + 2 * -10", 0),
            // ("50 / 2 * 2 + 10", 60),
            // ("2 * (5 + 10)", 30),
            // ("3 * 3 * 3 + 10", 37),
            // ("3 * (3 * 3) + 10", 37),
            // ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
        ];
        for (input, expected) in tests {
            let evaluated = test_eval(input);
            test_integer_object(evaluated, expected);
        }
    }
    fn test_eval(input: &str) -> Box<dyn object::Object>{
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
         return  eval(&program)


    }

    fn test_integer_object(obj: Box<dyn object::Object>, expected: i64) {
        let result = obj.as_any().downcast_ref::<object::Integer>().unwrap();
        assert_eq!(result.value, expected);
    }

}
