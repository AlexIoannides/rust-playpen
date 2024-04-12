// Taken from Chapter 6 of The Rust Programming Language

enum Shape {
    Circle {radius: f32},
    Triangle {base: f32, angles: [f32; 3]},
    Rectangle {width: f32, height: f32},
}

impl Shape {
    fn iam(&self) {
        match self {
            Shape::Circle{..} => println!("I am a circle!"),
            Shape::Triangle{..} => println!("I am a triangle!"),
            Shape::Rectangle{..} => println!("I am a rectangle!!"),
        }
    }
}

fn main() {
    // basic enums
    let s0 = Shape::Circle{radius: 1.0};
    let s1 = Shape::Triangle{base: 1.0, angles: [60., 60., 60.]};
    let s2 = Shape::Rectangle{width: 1.0, height: 1.0};
    print_shape(&s0);
    print_shape(&s1);
    print_shape(&s2);

    Shape::iam(&s0);

    // options
    let x = square(&Some(2.0));
    if let Some(e) = x {
        println!("x ^ 2 = {e}");
    }
}

fn print_shape(e: &Shape) {
    match e {
        Shape::Circle{radius} => println!("circle radius = {}", radius),
        Shape::Triangle{base, ..} => println!("triangle base = {}", base),
        Shape::Rectangle{width, height} => println!("rectangle width x height = {} x {}", width, height),
    };
}

fn square(x: &Option<f32>) -> Option<f32> {
    match x {
        Some(e) => return Some(e * e),
        None => return None,
    }
}