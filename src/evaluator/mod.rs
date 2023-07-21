use std::rc::Rc;

use crate::ast::{self, Expression};
use crate::ast::Statement;
use crate::envoriment::Environment;
use crate::object::{self, Object};
use crate::ast::Node;

impl Environment {
    pub fn eval(&mut self ,node: &dyn Node) -> Rc<Box<dyn object::Object>> {
        match node.node_type() {
            ast::NodeType::Program => self.eval_program(&node.as_any().downcast_ref::<ast::Program>().unwrap().statements),
            ast::NodeType::ExpressionStatement => self.eval(node.as_any().downcast_ref::<ast::ExpressionStatement>().unwrap().expression.as_node()),
            ast::NodeType::IntegerLiteral =>Rc::new(Box::new(object::Integer{value:node.as_any().downcast_ref::<ast::IntegerLiteral>().unwrap().value})),
            ast::NodeType::Boolean => self.bool_to_boolean_object(Some(node.as_any().downcast_ref::<ast::Boolean>().unwrap().value)),
            ast::NodeType::PrefixExpression => {
          
                let right = self.eval(node.as_any().downcast_ref::<ast::PrefixExpression>().unwrap().right.as_node());
                if self.is_error(right.clone()) {
                    return right;
                }
                return self.eval_perfix_expresion(&node.as_any().downcast_ref::<ast::PrefixExpression>().unwrap().operator, right);

            },
            ast::NodeType::InfixExpression => {
                let left = self.eval(node.as_any().downcast_ref::<ast::InfixExpression>().unwrap().left.as_node());
                if self.is_error(left.clone()) {
                    return left;
                }
                let right = self.eval(node.as_any().downcast_ref::<ast::InfixExpression>().unwrap().right.as_node());
                if self.is_error(right.clone()) {
                    return right;
                }
                
                return self.eval_infix_expression(&node.as_any().downcast_ref::<ast::InfixExpression>().unwrap().operator, left, right);
                
            },
            ast::NodeType::BlockStatement => self.eval_block_statements(&node.as_any().downcast_ref::<ast::BlockStatement>().unwrap().statements),
            ast::NodeType::IfExpression => self.eval_if_expression(node.as_any().downcast_ref::<ast::IfExpression>().unwrap().as_node()),
            ast::NodeType::ReturnStatement => {
                let value = self.eval(node.as_any().downcast_ref::<ast::ReturnStatement>().unwrap().return_value.as_node());
                if self.is_error(value.clone()) {
                    return value;
                }
                return Rc::new(Box::new(object::Return{value}));
            }
            ast::NodeType::LetStatement =>{
                let value = self.eval(node.as_any().downcast_ref::<ast::LetStatement>().unwrap().value.as_node());
                if self.is_error(value.clone()) {
                    return value;
                }

                self.set(node.as_any().downcast_ref::<ast::LetStatement>().unwrap().name.value.as_str(), value);

                return Rc::new(Box::new(object::Null{}));
            }
            ast::NodeType::Identifier => {
                let value = self.get(node.as_any().downcast_ref::<ast::Identifier>().unwrap().value.as_str());
                if value.is_some() {
                    return value.unwrap();
                }
                let builtin = self.builtins.get(node.as_any().downcast_ref::<ast::Identifier>().unwrap().value.as_str());
                if builtin.is_some() {
                    return Rc::new(Box::new(object::Builtin{func: builtin.unwrap().func.clone()}));
                }


                return self.new_error(&format!("identifier not found: {}", node.as_any().downcast_ref::<ast::Identifier>().unwrap().value));
            }
            ast::NodeType::FunctionLiteral => {
                let parameters = Rc::new(Box::new(node.as_any().downcast_ref::<ast::FunctionLiteral>().unwrap().parameters.clone()));
                let body = node.as_any().downcast_ref::<ast::FunctionLiteral>().unwrap().body.clone();
                return Rc::new(Box::new(object::Function{parameters, body, env: self.clone()}));
            }
            ast::NodeType::CallExpression =>{
                let function = self.eval(node.as_any().downcast_ref::<ast::CallExpression>().unwrap().function.as_node());
                if self.is_error(function.clone()) {
                    return function;
                }
                let args = self.eval_expressions(&node.as_any().downcast_ref::<ast::CallExpression>().unwrap().arguments);

                if args.len() == 1 && self.is_error(args[0].clone()) {
                    return args[0].clone();
                }

                return self.apply_function(function, &args);
          
            }
            ast::NodeType::StringLiteral => Rc::new(Box::new(object::StringValue{value: node.as_any().downcast_ref::<ast::StringLiteral>().unwrap().value.clone()})),
            _ => panic!("Not implemented yet")
            
        }
    }

    fn apply_function(&self , obj: Rc<Box<dyn object::Object>>, args: &Vec<Rc<Box< dyn Object>>>) -> Rc<Box<dyn object::Object>>{
        let function = obj.as_any().downcast_ref::<object::Function>();
        let function = match function {
            Some(x) => x,
            None => match obj.as_any().downcast_ref::<object::Builtin>() {
                Some(x) => return (x.func)(args.clone()),
                None => return self.new_error(&format!("not a function: {}", obj.object_type())),
            
                
            }
        };
        let mut extended_env = self.extend_function_env(function, args);
        let evaluated = extended_env.eval(function.body.as_node());
        return self.unwrap_return_value(evaluated);

    }

    fn extend_function_env(&self, function: &object::Function, args: &Vec<Rc<Box<dyn object::Object>>>) -> Environment {
        let mut env = Environment::new_enclosed_environment(function.env.store.clone());
        for (i, param) in function.parameters.iter().enumerate() {
            env.set(param.string().as_str(), args[i].clone());
        }
        env
    }
    fn unwrap_return_value(&self, obj: Rc<Box<dyn object::Object>>) -> Rc<Box<dyn object::Object>> {
        match obj.object_type() {
            object::ObjectType::RETURN => {
                let return_value = obj.as_any().downcast_ref::<object::Return>().unwrap();
                return return_value.value.clone();
            },
            _ => return obj,
        }
    }


    fn eval_expressions(&mut self , args: &Vec<Box< dyn Expression>>) -> Vec<Rc<Box<dyn object::Object>>>{
        let mut  result: Vec<Rc<Box<dyn object::Object>>> = Vec::new();
        
        for e in args.iter() {
            let evaluated = self.eval(e.as_node());
            if self.is_error(evaluated.clone()) {
                return vec![evaluated];
            }
            result.push(evaluated);
        }

        return result;
    }
  
    fn  new_error(&self ,message: &str) -> Rc<Box<dyn object::Object>> {
        Rc::new(Box::new(object::Error{message: message.to_string()}))
    }

    fn is_error(&self, obj: Rc<Box<dyn object::Object>>) -> bool {
        obj.object_type() == object::ObjectType::ERROR
    }
    fn eval_block_statements(&mut self, statements: &Vec<Box<dyn Statement>>) -> Rc<Box<dyn object::Object>>{
        let mut result: Rc<Box<dyn object::Object>> = Rc::new(Box::new(object::Null{}));
        for statement in statements.iter() {
            result = self.eval(statement.as_node());
            match result.object_type() {
               object::ObjectType::RETURN | object::ObjectType::ERROR => return result,
               _ => continue,
               
           }
        }
        return result;


        
    }
    
    pub  fn eval_program(&mut self ,statements: &Vec<Box<dyn Statement>>) -> Rc<Box<dyn object::Object>> {
        let mut result: Rc<Box<dyn object::Object>> = Rc::new(Box::new(object::Null{}));
        for statement in statements.iter() {
            // println!("{:?}", statement);
            result = self.eval(statement.as_node());
            // println!("{:?}", result.inspect());
            result = match result.object_type() {
                object::ObjectType::RETURN =>return  result.as_any().downcast_ref::<object::Return>().unwrap().value(),
                object::ObjectType::ERROR => return result,
                _ => result,
            };
           
        }
        return result;
    }
    
    fn eval_if_expression(&mut self, node: &dyn Node) -> Rc<Box<dyn object::Object>>{
        let condition = self.eval(node.as_any().downcast_ref::<ast::IfExpression>().unwrap().condition.as_node());
        if self.is_error(condition.clone()) {
            return condition;
        }
        if self.is_truthy(condition) {
            return self.eval(node.as_any().downcast_ref::<ast::IfExpression>().unwrap().consequence.as_node());
        } else {
            match node.as_any().downcast_ref::<ast::IfExpression>().unwrap().alternative  {
                Some(_) => self.eval(node.as_any().downcast_ref::<ast::IfExpression>().unwrap().alternative.as_ref().unwrap().as_node()),
                None => Rc::new(Box::new(object::Null{})),
            }
            
        }
        
    }
    
    fn is_truthy (&self, obj: Rc<Box<dyn object::Object>>) -> bool {
        match obj.object_type() {
            object::ObjectType::BOOLEAN => {
                let boolean = obj.as_any().downcast_ref::<object::Boolean>().unwrap();
                return boolean.value;
            },
            object::ObjectType::NULL => false,
            _ => true,
        }
    }

    fn eval_infix_expression(&self, operator: &str, left: Rc<Box<dyn object::Object>>, right: Rc<Box<dyn object::Object>>) ->Rc< Box<dyn object::Object>> {
        match (left.object_type(), right.object_type()) {
            (object::ObjectType::INTEGER, object::ObjectType::INTEGER) => {
                let left_value = left.as_any().downcast_ref::<object::Integer>().unwrap();
                let right_value = right.as_any().downcast_ref::<object::Integer>().unwrap();
                return self.eval_integer_infix_expression(operator, left_value, right_value);
            },
            (object::ObjectType::BOOLEAN, object::ObjectType::BOOLEAN) => {
                let left_value = left.as_any().downcast_ref::<object::Boolean>().unwrap();
                let right_value = right.as_any().downcast_ref::<object::Boolean>().unwrap();
                return self.eval_boolean_infix_expression(operator, left_value, right_value);
            },
            (object::ObjectType::STRING, object::ObjectType::STRING) => {
                let left_value = left.as_any().downcast_ref::<object::StringValue>().unwrap();
                let right_value = right.as_any().downcast_ref::<object::StringValue>().unwrap();
                return self.eval_string_infix_expression(operator, left_value, right_value);
            },

            (object::ObjectType::INTEGER , _)=>{
                return  self.new_error(&format!("type mismatch: {} {} {}", left.object_type(), operator, right.object_type()));
            }
            (object::ObjectType::BOOLEAN , _)=>{
                return  self.new_error(&format!("type mismatch: {} {} {}", left.object_type(), operator, right.object_type()));
            }
            _ => self.new_error(&format!("unknown operator: {} {} {}", left.object_type(), operator, right.object_type()))
        }
    }

    fn eval_string_infix_expression(&self, operator: &str, left: &object::StringValue, right: &object::StringValue) -> Rc<Box<dyn object::Object>> {
        match operator {
            "+" => {
                let left_value = left.value.clone();
                let right_value = right.value.clone();
                return Rc::new(Box::new(object::StringValue{value: left_value + right_value.as_str()}));
            },
            _ => self.new_error(&format!("unknown operator: {} {} {}", left.object_type(), operator, right.object_type()))
        }
    }

    fn eval_integer_infix_expression(&self, operator: &str, left: &object::Integer, right:  &object::Integer) -> Rc<Box<dyn object::Object>> {
        let right_value = right.value;
        let left_value = left.value;
        match operator {
            "+" => Rc::new(Box::new(object::Integer{value: left_value + right_value})),
            "-" => Rc::new(Box::new(object::Integer{value: left_value - right_value})),
            "*" => Rc::new(Box::new(object::Integer{value: left_value * right_value})),
            "/" => Rc::new(Box::new(object::Integer{value: left_value / right_value})),
            "<" => self.bool_to_boolean_object(Some(left_value < right_value)),
            ">" => self.bool_to_boolean_object(Some(left_value > right_value)),
            "==" => self.bool_to_boolean_object(Some(left_value == right_value)),
            "!=" => self.bool_to_boolean_object(Some(left_value != right_value)),
            _ => self.new_error( &format!("unknown operator: {} {} {}", left.object_type(), operator, right.object_type()))
        }
    }

    fn eval_boolean_infix_expression(&self,operator: &str, left: &object::Boolean, right: &object::Boolean) -> Rc<Box<dyn object::Object>> {
        let right_value = right.value;
        let left_value = left.value;
        match operator {
            "==" => self.bool_to_boolean_object(Some(left_value == right_value)),
            "!=" => self.bool_to_boolean_object(Some(left_value != right_value)),
            _ => self.new_error( &format!("unknown operator: {} {} {}", left.object_type(), operator, right.object_type()))
        }
    }



    pub fn bool_to_boolean_object(&self, input: Option<bool>) -> Rc<Box<dyn object::Object>> {
        match input {
            Some(true) => Rc::new(Box::new(object::Boolean{value: true})),
            Some(false) => Rc::new(Box::new(object::Boolean{value: false})),
            None => Rc::new(Box::new(object::Null{})),
        }
            
    }

    pub fn eval_perfix_expresion(&self, operator: &str, right: Rc<Box<dyn object::Object>>) -> Rc<Box<dyn object::Object>> {
        match operator {
            "!" => self.eval_bang_operator_expression(right),
            "-" => self.eval_minus_prefix_operator_expression(right),
            _ => self.new_error(&format!("unknown operator: {}{}", operator, right.object_type())),
        }
    }

    fn eval_bang_operator_expression(&self, right: Rc<Box<dyn object::Object>>) -> Rc<Box<dyn object::Object>> {
        match right.object_type() {
            object::ObjectType::BOOLEAN => {
                let boolean = right.as_any().downcast_ref::<object::Boolean>().unwrap();
                return self.bool_to_boolean_object(Some(!boolean.value));
            }
            object::ObjectType::NULL => self.bool_to_boolean_object(None),
            _ => self.bool_to_boolean_object(Some(false)),
        }
    }

    fn eval_minus_prefix_operator_expression(&self ,right: Rc<Box<dyn object::Object>>) -> Rc<Box<dyn object::Object>> {
        match right.object_type() {
            object::ObjectType::INTEGER => {
                let value = right.as_any().downcast_ref::<object::Integer>().unwrap();
                return Rc::new(Box::new(object::Integer{value: -value.value}));
            }
            _ => self.new_error(&format!("unknown operator: -{}", right.object_type())),
        }
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
        let mut  env = Environment::new();
         return  env.eval(&program);


    }

    fn test_integer_object(obj: Rc<Box<dyn object::Object>>, expected: i64) {
        println!("{:?}", obj.inspect()
    );
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
    #[test]
    fn test_error_handling (){
        let tests = vec![
            ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
            ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
            ("-true", "unknown operator: -BOOLEAN"),
            ("true + false;", "unknown operator: BOOLEAN + BOOLEAN"),
            ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
            ("if (10 > 1) { true + false; }", "unknown operator: BOOLEAN + BOOLEAN"),
            // (
            //     r#"
            //     if (10 > 1) {
            //         if (10 > 1) {
            //             return true + false;
            //         }
            //         return 1;
            //     }
            //     "#,
            //     "unknown operator: BOOLEAN + BOOLEAN",
            // ),
            // ("foobar", "identifier not found: foobar"),
            (r#""Hello" - "World""#, "unknown operator: STRING - STRING"),
            // (
            //     r#"
            //     {
            //         let foobar = 1;
            //         foobar
            //     }
            //     "#,
            //     "identifier not found: foobar",
            // ),
            ("foobar", "identifier not found: foobar"),
        ];
        for (input, expected) in tests {
            
            let evaluated = test_eval(input);

            let err = evaluated.as_any().downcast_ref::<object::Error>().unwrap();
            assert_eq!(err.message, expected);
        }
    }

    #[test]
    fn test_let_statements(){
        let input = vec![
            ("let a = 5; a;", 5),
            ("let a = 5 * 5; a;", 25),
            ("let a = 5; let b = a; b;", 5),
            ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
        ];
        for (input, expected) in input {
            let evaluated = test_eval(input);
            test_integer_object(evaluated, expected);
        }
    }
    #[test]

    fn test_function_object(){
        let input = "fn(x) { x + 2; };";
        let evaluated = test_eval(input);
        let fn_obj = evaluated.as_any().downcast_ref::<object::Function>().unwrap();
        assert_eq!(fn_obj.parameters.len(), 1);
        assert_eq!(fn_obj.parameters[0].string(), "x");
        assert_eq!(fn_obj.body.string(), "(x + 2)");
    }
    #[test]

    fn test_function_application(){
        let tests = vec![
            ("let identity = fn(x) { x; }; identity(5);", 5),
            ("let identity = fn(x) { return x; }; identity(5);", 5),
            ("let double = fn(x) { x * 2; }; double(5);", 10),
            ("let add = fn(x, y) { x + y; }; add(5, 5);", 10),
            ("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20),
            ("fn(x) { x; }(5)", 5),
        ];
        for (input, expected) in tests {
            let evaluated = test_eval(input);
            test_integer_object(evaluated, expected);
        }
    }
    #[test]
    fn test_closures(){
        let input = r#"
        let newAdder = fn(x) {
            fn(y) { x + y };
        };
        
        let addTwo = newAdder(2);
        addTwo(2);
        "#;
        let evaluated = test_eval(input);
        test_integer_object(evaluated, 4);
    }
    #[test]
    fn test_string_literal(){
        let input = r#""Hello World!""#;
        let evaluated = test_eval(input);
        let str_obj = evaluated.as_any().downcast_ref::<object::StringValue>().unwrap();
        assert_eq!(str_obj.value, "Hello World!");
    }
    #[test]
    fn test_string_concatenation(){
        let input = r#""Hello" + " " + "World!""#;
        let evaluated = test_eval(input);
        let str_obj = evaluated.as_any().downcast_ref::<object::StringValue>().unwrap();
        assert_eq!(str_obj.value, "Hello World!");
    }
    #[test]

    fn test_builtin_functions(){
        let tests = vec![
            (r#"len("")"#, 0),
            (r#"len("four")"#, 4),
            (r#"len("hello world")"#, 11),
            // (r#"len(1)"#, "argument to `len` not supported, got INTEGER"),
            // (r#"len("one", "two")"#, "wrong number of arguments. got=2, want=1"),
        ];
        for (input, expected) in tests {
            let evaluated = test_eval(input);
            match expected {
                0 => test_integer_object(evaluated, 0),
                4 => test_integer_object(evaluated, 4),
                11 => test_integer_object(evaluated, 11),
                _ => panic!("unhandled case. got={}", expected),
            }
        }
    }
}
