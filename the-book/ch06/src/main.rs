// Taken from Chapter 6 of The Rust Programming Language

enum Shape {
    Circle(CircleData),
    Rectangle(RectangleData),
}

// A simpler alternative to the above could be:
// enum Shape {
//     Circle{radius: f32},
//     Rectangle{width: f32, height: f32},
// }

struct CircleData {
    radius: f32,
}

struct RectangleData {
    width: f32,
    height: f32,
}

impl Shape {
    fn circle(radius: f32) -> Self {
        Self::Circle(CircleData{radius}) // note use of more compact less formal notation here
    }

    fn rectangle(width: f32, height: f32) -> Self {
        Self::Rectangle(RectangleData{width, height}) // note use of more compact less formal notation here
    }

    fn iam(&self) {
        match self {
            Shape::Circle(_) => println!("I am a circle!"),
            Shape::Rectangle(_) => println!("I am a rectangle!!"),
        }
    }
}

fn main() {
    // basic enums
    let s0 = Shape::circle(1.0);
    let s1 = Shape::rectangle(1.0, 1.0);
    print_shape(&s0);
    print_shape(&s1);

    Shape::iam(&s0);

    // options
    println!("some_calc = {}", some_calc(1.0, None));
    println!("some_calc = {}", some_calc(1.0, Some(0.75)));

    let x = square(Some(2.0));
    if let Some(e) = x {
        println!("x ^ 2 = {e}");
    }
}

fn print_shape(e: &Shape) {
    match e {
        Shape::Circle(circle_data) => println!("circle radius = {}", circle_data.radius),
        Shape::Rectangle(rect_data) => println!("rectangle width x height = {} x {}", rect_data.width, rect_data.height),
    };
}

fn square(x: Option<f32>) -> Option<f32> {
    match x {
        Some(e) => return Some(e * e),
        None => return None,
    }
}

fn some_calc(x: f32, c: Option<f32>) -> f32 {
    match c {
        Some(c) => 3.141 * x * c,
        None => 3.141 * x,
    }
}
