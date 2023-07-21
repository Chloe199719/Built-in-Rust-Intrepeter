use crate::ast;
use crate::ast::Statement;
use crate::object;
use crate::ast::Node;

    pub fn eval(node: &dyn Node) -> Box<dyn object::Object> {
        match node.node_type() {
            ast::NodeType::Program => eval_statements(&node.as_any().downcast_ref::<ast::Program>().unwrap().statements),
            ast::NodeType::ExpressionStatement => eval(node.as_any().downcast_ref::<ast::ExpressionStatement>().unwrap().expression.as_node()),
            ast::NodeType::IntegerLiteral =>Box::new(object::Integer{value:node.as_any().downcast_ref::<ast::IntegerLiteral>().unwrap().value}),
            ast::NodeType::Boolean => bool_to_boolean_object(Some(node.as_any().downcast_ref::<ast::Boolean>().unwrap().value)),
            ast::NodeType::PrefixExpression => {
          
                let right = eval(node.as_any().downcast_ref::<ast::PrefixExpression>().unwrap().right.as_node());
                return eval_perfix_expresion(&node.as_any().downcast_ref::<ast::PrefixExpression>().unwrap().operator, right);

            }

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

    pub fn bool_to_boolean_object(input: Option<bool>) -> Box<dyn object::Object> {
        match input {
            Some(true) => Box::new(object::Boolean{value: true}),
            Some(false) => Box::new(object::Boolean{value: false}),
            None => Box::new(object::Null{}),
        }
            
    }

    pub fn eval_perfix_expresion(operator: &str, right: Box<dyn object::Object>) -> Box<dyn object::Object> {
        match operator {
            "!" => eval_bang_operator_expression(right),
            "-" => eval_minus_prefix_operator_expression(right),
            _ => panic!("Not implemented yet")
        }
    }

    fn eval_bang_operator_expression(right: Box<dyn object::Object>) -> Box<dyn object::Object> {
        match right.object_type() {
            object::ObjectType::BOOLEAN => {
                let boolean = right.as_any().downcast_ref::<object::Boolean>().unwrap();
                return bool_to_boolean_object(Some(!boolean.value));
            }
            object::ObjectType::NULL => bool_to_boolean_object(None),
            _ => bool_to_boolean_object(Some(false)),
        }
    }

    fn eval_minus_prefix_operator_expression(right: Box<dyn object::Object>) -> Box<dyn object::Object> {
        match right.object_type() {
            object::ObjectType::INTEGER => {
                let value = right.as_any().downcast_ref::<object::Integer>().unwrap();
                return Box::new(object::Integer{value: -value.value});
            }
            _ => panic!("Not implemented yet")
        }
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
            ("-5", -5),
            ("-10", -10),
            ("5 + 5 + 5 + 5 - 10", 10),
            ("2 * 2 * 2 * 2 * 2", 32),
            ("-50 + 100 + -50", 0),
            ("5 * 2 + 10", 20),
            ("5 + 2 * 10", 25),
            ("20 + 2 * -10", 0),
            ("50 / 2 * 2 + 10", 60),
            ("2 * (5 + 10)", 30),
            ("3 * 3 * 3 + 10", 37),
            ("3 * (3 * 3) + 10", 37),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
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
    
    #[test]

    fn test_eval_boolean_expression (){
        let tests = vec![
            ("true", true),
            ("false", false),
            // ("1 < 2", true),
            // ("1 > 2", false),
            // ("1 < 1", false),
            // ("1 > 1", false),
            // ("1 == 1", true),
            // ("1 != 1", false),
            // ("1 == 2", false),
            // ("1 != 2", true),
            // ("true == true", true),
            // ("false == false", true),
            // ("true == false", false),
            // ("true != false", true),
            // ("false != true", true),
            // ("(1 < 2) == true", true),
            // ("(1 < 2) == false", false),
            // ("(1 > 2) == true", false),
            // ("(1 > 2) == false", true),
        ];
        for (input, expected) in tests {
            let evaluated = test_eval(input);
            test_boolean_object(evaluated, expected);
        }

    }
    fn test_boolean_object(obj: Box<dyn object::Object>, expected: bool) {
        let result = obj.as_any().downcast_ref::<object::Boolean>().unwrap();
        assert_eq!(result.value, expected);
    }

    #[test]

    fn test_bang_operator(){
        let tests = vec![
            ("!true", false),
            ("!false", true),
            ("!5", false),
            ("!!true", true),
            ("!!false", false),
            ("!!5", true),
        ];
        for (input, expected) in tests {
            let evaluated = test_eval(input);
            // println!("{:?}", evaluated.inspect());
            test_boolean_object(evaluated, expected);
        }

    }
}
