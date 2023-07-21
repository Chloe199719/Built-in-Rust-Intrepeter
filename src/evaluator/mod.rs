use std::rc::Rc;

use crate::ast::{self, Expression};
use crate::ast::Statement;
use crate::object;
use crate::ast::Node;

    pub fn eval(node: &dyn Node) -> Rc<Box<dyn object::Object>> {
        match node.node_type() {
            ast::NodeType::Program => eval_program(&Rc::new(&node.as_any().downcast_ref::<ast::Program>().unwrap().statements)),
            ast::NodeType::ExpressionStatement => eval(node.as_any().downcast_ref::<ast::ExpressionStatement>().unwrap().expression.as_node()),
            ast::NodeType::IntegerLiteral =>Rc::new(Box::new(object::Integer{value:node.as_any().downcast_ref::<ast::IntegerLiteral>().unwrap().value})),
            ast::NodeType::Boolean => bool_to_boolean_object(Some(node.as_any().downcast_ref::<ast::Boolean>().unwrap().value)),
            ast::NodeType::PrefixExpression => {
          
                let right = eval(node.as_any().downcast_ref::<ast::PrefixExpression>().unwrap().right.as_node());
                return eval_perfix_expresion(&node.as_any().downcast_ref::<ast::PrefixExpression>().unwrap().operator, right);

            },
            ast::NodeType::InfixExpression => {
                let left = eval(node.as_any().downcast_ref::<ast::InfixExpression>().unwrap().left.as_node());
                let right = eval(node.as_any().downcast_ref::<ast::InfixExpression>().unwrap().right.as_node());

                return eval_infix_expression(&node.as_any().downcast_ref::<ast::InfixExpression>().unwrap().operator, left, right);
               
            },
            ast::NodeType::BlockStatement => eval_block_statements(&Rc::new(&node.as_any().downcast_ref::<ast::BlockStatement>().unwrap().statements)),
            ast::NodeType::IfExpression => eval_if_expression(node.as_any().downcast_ref::<ast::IfExpression>().unwrap().as_node()),
            ast::NodeType::ReturnStatement => {
                let value = eval(node.as_any().downcast_ref::<ast::ReturnStatement>().unwrap().return_value.as_node());
                return Rc::new(Box::new(object::Return{value}));
            }
            _ => panic!("Not implemented yet")
            
        }
    }


    fn eval_block_statements(statements: &Rc<&Vec<Box<dyn Statement>>>) -> Rc<Box<dyn object::Object>>{
        let mut result: Rc<Box<dyn object::Object>> = Rc::new(Box::new(object::Null{}));
        for statement in statements.iter() {
            result = eval(statement.as_node());
            if result.object_type() == object::ObjectType::RETURN && result.object_type() != object::ObjectType::NULL {
                return result
            }
        }
        return result;



    }


    fn eval_if_expression(node: &dyn Node) -> Rc<Box<dyn object::Object>>{
        let condition = eval(node.as_any().downcast_ref::<ast::IfExpression>().unwrap().condition.as_node());

        if is_truthy(condition) {
            return eval(node.as_any().downcast_ref::<ast::IfExpression>().unwrap().consequence.as_node());
        } else {
            match node.as_any().downcast_ref::<ast::IfExpression>().unwrap().alternative  {
                Some(_) => eval(node.as_any().downcast_ref::<ast::IfExpression>().unwrap().alternative.as_ref().unwrap().as_node()),
                None => Rc::new(Box::new(object::Null{})),
            }
    
        }

    }

    fn is_truthy (obj: Rc<Box<dyn object::Object>>) -> bool {
        match obj.object_type() {
            object::ObjectType::BOOLEAN => {
                let boolean = obj.as_any().downcast_ref::<object::Boolean>().unwrap();
                return boolean.value;
            },
            object::ObjectType::NULL => false,
            _ => true,
        }
    }

    fn eval_infix_expression(operator: &str, left: Rc<Box<dyn object::Object>>, right: Rc<Box<dyn object::Object>>) ->Rc< Box<dyn object::Object>> {
        match (left.object_type(), right.object_type()) {
            (object::ObjectType::INTEGER, object::ObjectType::INTEGER) => {
                let left_value = left.as_any().downcast_ref::<object::Integer>().unwrap();
                let right_value = right.as_any().downcast_ref::<object::Integer>().unwrap();
                return eval_integer_infix_expression(operator, left_value.value, right_value.value);
            },
            (object::ObjectType::BOOLEAN, object::ObjectType::BOOLEAN) => {
                let left_value = left.as_any().downcast_ref::<object::Boolean>().unwrap();
                let right_value = right.as_any().downcast_ref::<object::Boolean>().unwrap();
                return eval_boolean_infix_expression(operator, left_value.value, right_value.value);
            },
            _ => panic!("Not implemented yet")
        }
    }

    fn eval_integer_infix_expression(operator: &str, left: i64, right: i64) -> Rc<Box<dyn object::Object>> {
        match operator {
            "+" => Rc::new(Box::new(object::Integer{value: left + right})),
            "-" => Rc::new(Box::new(object::Integer{value: left - right})),
            "*" => Rc::new(Box::new(object::Integer{value: left * right})),
            "/" => Rc::new(Box::new(object::Integer{value: left / right})),
            "<" => bool_to_boolean_object(Some(left < right)),
            ">" => bool_to_boolean_object(Some(left > right)),
            "==" => bool_to_boolean_object(Some(left == right)),
            "!=" => bool_to_boolean_object(Some(left != right)),
            _ => panic!("Not implemented yet")
        }
    }

    fn eval_boolean_infix_expression(operator: &str, left: bool, right: bool) -> Rc<Box<dyn object::Object>> {
        match operator {
            "==" => bool_to_boolean_object(Some(left == right)),
            "!=" => bool_to_boolean_object(Some(left != right)),
            _ => panic!("Not implemented yet")
        }
    }


    pub  fn eval_program(statements: &Rc<&Vec<Box<dyn Statement>>>) -> Rc<Box<dyn object::Object>> {
        let mut result: Rc<Box<dyn object::Object>> = Rc::new(Box::new(object::Null{}));
        for statement in statements.iter() {
            // println!("{:?}", statement);
            result = eval(statement.as_node());
            if result.object_type() == object::ObjectType::RETURN {
                return result.as_any().downcast_ref::<object::Return>().unwrap().value();
            }
        }
        return result;
    }

    pub fn bool_to_boolean_object(input: Option<bool>) -> Rc<Box<dyn object::Object>> {
        match input {
            Some(true) => Rc::new(Box::new(object::Boolean{value: true})),
            Some(false) => Rc::new(Box::new(object::Boolean{value: false})),
            None => Rc::new(Box::new(object::Null{})),
        }
            
    }

    pub fn eval_perfix_expresion(operator: &str, right: Rc<Box<dyn object::Object>>) -> Rc<Box<dyn object::Object>> {
        match operator {
            "!" => eval_bang_operator_expression(right),
            "-" => eval_minus_prefix_operator_expression(right),
            _ => panic!("Not implemented yet")
        }
    }

    fn eval_bang_operator_expression(right: Rc<Box<dyn object::Object>>) -> Rc<Box<dyn object::Object>> {
        match right.object_type() {
            object::ObjectType::BOOLEAN => {
                let boolean = right.as_any().downcast_ref::<object::Boolean>().unwrap();
                return bool_to_boolean_object(Some(!boolean.value));
            }
            object::ObjectType::NULL => bool_to_boolean_object(None),
            _ => bool_to_boolean_object(Some(false)),
        }
    }

    fn eval_minus_prefix_operator_expression(right: Rc<Box<dyn object::Object>>) -> Rc<Box<dyn object::Object>> {
        match right.object_type() {
            object::ObjectType::INTEGER => {
                let value = right.as_any().downcast_ref::<object::Integer>().unwrap();
                return Rc::new(Box::new(object::Integer{value: -value.value}));
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
    fn test_eval(input: &str) -> Rc<Box<dyn object::Object>>{
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
         return  eval(&program)


    }

    fn test_integer_object(obj: Rc<Box<dyn object::Object>>, expected: i64) {
        let result = obj.as_any().downcast_ref::<object::Integer>().unwrap();
        assert_eq!(result.value, expected);
    }
    
    #[test]

    fn test_eval_boolean_expression (){
        let tests = vec![
            ("true", true),
            ("false", false),
            ("1 < 2", true),
            ("1 > 2", false),
            ("1 < 1", false),
            ("1 > 1", false),
            ("1 == 1", true),
            ("1 != 1", false),
            ("1 == 2", false),
            ("1 != 2", true),
            ("true == true", true),
            ("false == false", true),
            ("true == false", false),
            ("true != false", true),
            ("false != true", true),
            ("(1 < 2) == true", true),
            ("(1 < 2) == false", false),
            ("(1 > 2) == true", false),
            ("(1 > 2) == false", true),
        ];
        for (input, expected) in tests {
            let evaluated = test_eval(input);
            test_boolean_object(evaluated, expected);
        }

    }
    fn test_boolean_object(obj: Rc<Box<dyn object::Object>>, expected: bool) {
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

    #[test]
    fn test_if_else_expressions(){
        let tests = vec![
            ("if (true) { 10 }", Some(10)),
            ("if (false) { 10 }", None),
            ("if (1) { 10 }", Some(10)),
            ("if (1 < 2) { 10 }", Some(10)),
            ("if (1 > 2) { 10 }", None),
            ("if (1 > 2) { 10 } else { 20 }", Some(20)),
            ("if (1 < 2) { 10 } else { 20 }", Some(10)),
        ];
        for (input, expected) in tests {
            let evaluated = test_eval(input);
            match expected {
                Some(x) => test_integer_object(evaluated, x),
                None => test_null_object(evaluated),
            }
        }
    }
    fn test_null_object(obj: Rc<Box<dyn object::Object>>) {
        assert_eq!(obj.object_type(), object::ObjectType::NULL);
    }
    #[test]
    fn test_return_statements(){
        let tests = vec![
            ("return 10;", 10),
            ("return 10; 9;", 10),
            ("return 2 * 5; 9;", 10),
            ("9; return 2 * 5; 9;", 10),
            (
                r#"
                if (10 > 1) {
                    if (10 > 1) {
                        return 10;
                    }
                    return 1;
                }
                "#,
                10,
            ),
        ];
        for (input, expected) in tests {
            let evaluated = test_eval(input);
            test_integer_object(evaluated, expected);
        }

    }
}
