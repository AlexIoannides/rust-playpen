// Taken from Chapter 5 of The Rust Programming Language

struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn square(width: f32) -> Self {
        Self {width: width, height: width}
    }
}

fn main() {
    let r1 = Rectangle{width: 1.2, height: 1.8};
    println!("r1 = {} x {}", r1.width, r1.height);
    println!("area(r1) = {}", r1.area());

    let r2 = Rectangle::square(2.0);
    println!("r2 = {} x {}", r2.width, r2.height);
    println!("area(r1) = {}", r2.area());
}
