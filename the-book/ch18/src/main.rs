// Taken from Chapter 18 of The Rust Programming Language
// --> not entirely unlike the pattern matching capabilities of Python and Scala
fn main() {
    // only matching on arm
    let e = Some(5);
    if let Some(x) = e {
        println!("x = {x}");
    }

    // patterns in assignment
    let (x, y, z) = (1, 2, 3);
    println!("[x={x}, y={y}, z={z}]");

    // conditional while loops
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for loop enumeration
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // union of patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    match x {
        1..=3 => println!("one to three"),
        _ => println!("anything"),
    }

    // extra match guards
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    // destructuring
    let p = Point{ x: 0.0, y: 1.0 };
    let Point{ x: a, y: b } = p;
    println!("Point({a}, {b})");

}

struct Point {
    x: f32,
    y: f32,
}
