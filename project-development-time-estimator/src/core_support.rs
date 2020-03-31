/* ** Core Support ** */

use std::any::Any;

pub trait Object {
    fn type_name(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}

pub fn type_name(x: &dyn Object) -> &str {
    x.type_name()
}

pub fn is_of_type<T: 'static>(x: &dyn Object) -> bool {
    x.as_any().is::<T>()
}

impl Object for f32 {
    fn type_name(&self) -> &str {"f32"}
    fn as_any(&self) -> &dyn Any {self}
}

impl Object for i32 {
    fn type_name(&self) -> &str {"i32"}
    fn as_any(&self) -> &dyn Any {self}
}

impl Object for i64 {
    fn type_name(&self) -> &str {"i64"}
    fn as_any(&self) -> &dyn Any {self}
}

impl Object for f64 {
    fn type_name(&self) -> &str {"f64"}
    fn as_any(&self) -> &dyn Any {self}
}

impl Object for String {
    fn type_name(&self) -> &str {"String"}
    fn as_any(&self) -> &dyn Any {self}
}

/*
example:
    println!("{}", core_support::type_name(&data));
    println!("{}", core_support::is_of_type::<f64>(&data));
*/
