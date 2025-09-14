// Taken from Chapter 10 of The Rust Programming Language
// --> define an interface using the Shape trait
// --> define a print_area function that will work with any generic type that implements the Shape trait

trait Shape {
    fn area(&self) -> f32;
}


struct Circle {
    radius: f32
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        3.141 * self.radius
    }
}


struct Square {
    width: f32
}

impl Shape for Square {
    fn area(&self) -> f32 {
        self.width * self.width
    }
}


fn print_area<T: Shape>(s: &T) {
    println!("area = {}", s.area());
}


fn main() {
    // traits and generics
    let s0 = Circle{radius: 2.25};
    let s1 = Square{width: 1.25};
    print_area(&s0);
    print_area(&s1);

    // lifetimes
    let msg_longest;
    let msg_one = String::from("foo");
    {
        let msg_two = String::from("smeg");
        msg_longest = longest(&msg_one, &msg_two);
        println!("longest message = {msg_longest}");
    }
}


// example of lifetime syntax needed to determine LOWEST bound on scope-based lifetime
// without this, it is not possible for the borrow checker to determine the shortest relevent scope (lifetime)
fn longest<'a>(s: &'a String, t: &'a String) -> &'a String { 
    if s.len() > t.len() {
        s
    } else {
        t
    }
}
