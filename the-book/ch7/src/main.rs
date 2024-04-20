// Taken from Chapter 7 of The Rust Programming Language
// -> refacoring Chatper 6 examples into sub-modules

use ch7::shapes::Shape;
use ch7::utils;

fn main() {
    // basic enums
    let s0 = Shape::circle(1.0);
    let s1 = Shape::rectangle(1.0, 1.0);
    utils::print_shape(&s0);
    utils::print_shape(&s1);

    Shape::iam(&s0);

    // options
    println!("some_calc = {}", utils::some_calc(1.0, None));
    println!("some_calc = {}", utils::some_calc(1.0, Some(0.75)));

    let x = utils::square(Some(2.0));
    if let Some(e) = x {
        println!("x ^ 2 = {e}");
    }
}
