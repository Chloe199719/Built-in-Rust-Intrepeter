use std::rc::Rc;

use crate::object;

pub fn len(args: Vec<Rc<Box<dyn object::Object>>>) -> Rc<Box<dyn object::Object>> {
    if args.len() != 1 {
        return Rc::new(Box::new(object::Null{}));
    }

    match args[0].as_any().downcast_ref::<object::StringValue>() {
        Some(s) => Rc::new(Box::new(object::Integer{value: s.value.len() as i64})),
        None => Rc::new(Box::new(object::Null{}))
    }
}