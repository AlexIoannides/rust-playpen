// Taken from Chapter 19 of The Rust Programming Language
use std::ops::Add;

fn main() {
    // 1st class functions
    let y = g(|x|{x * x}, 2);
    println!("g(f(x)) = {y}");

    // using traits with placeholder types
    let r = Kiki{ cute: true};
    println!("Hop {}", r.hop());
    println!("Is rabbit cute? {}", r.cute);

    // operator overloading
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

// 1st class functions
fn g(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x) + x
}

// type aliases and placeholder types
type _SomeTypeAlias = Vec<Kiki>; // a type alias

trait Dentoid {
    type Rabbit; // undeclared type alias

    fn hop(&self) -> Self::Rabbit;
}

struct Kiki {
    cute: bool,
}

impl Dentoid for Kiki {
    type Rabbit = &'static str; // crystalised type alias

    fn hop(&self) -> Self::Rabbit {
        "to it!"
    }
}

// operator overloading
#[derive(Debug, Copy, Clone, PartialEq)] // automatically generate
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
