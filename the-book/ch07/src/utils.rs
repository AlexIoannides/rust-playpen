// module for all utility functions
use crate::shapes::Shape;


pub fn print_shape(e: &Shape) {
    match e {
        Shape::Circle{radius} => println!("circle radius = {}", radius),
        Shape::Rectangle{width, height} => println!("rectangle width x height = {} x {}", width, height),
    };
}

pub fn square(x: Option<f32>) -> Option<f32> {
    match x {
        Some(e) => return Some(e * e),
        None => return None,
    }
}

pub fn some_calc(x: f32, c: Option<f32>) -> f32 {
    match c {
        Some(c) => 3.141 * x * c,
        None => 3.141 * x,
    }
}
