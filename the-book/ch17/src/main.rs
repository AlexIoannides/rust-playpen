// Taken from Chapter 17 of The Rust Programming Language
// --> elements of object-oriented programming in Rust

fn main() {
    // dynamic dispatch based on trait implementation (analagous to interface implementation)
    let me = Person::new(String::from("Alex"), String::from("Ioannides"));
    let rabbit = Pet::new(String::from("Kiki"));
    meet(&me);
    meet(&rabbit);
}

fn meet(e: &dyn Greeting) {
    e.greet();
}

trait Greeting {
    fn greet(&self);
}

struct Person(String, String);

impl Person {
    fn new(first_name: String, second_name: String) -> Person {
        Person(first_name, second_name)
    }
}

impl Greeting for Person {
    fn greet(&self) {
        println!("Hello, said {} {}", self.0, self.1);
    }
}

struct Pet(String);

impl Pet {
    fn new(name: String) -> Pet {
        Pet(name)
    }
}

impl Greeting for Pet {
    fn greet(&self) {
        println!("Hello, squeaked {}", self.0);
    }
}
