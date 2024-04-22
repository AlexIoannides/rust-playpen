// Taken from Chapter 13 of The Rust Programming Language
// --> closures and iterators

fn main() {
    let x = "foo";

    // at the most informal level
    let y = {
        if x == "foo" { // inherits x from the enclosing environment
            1
        } else {
            0
        }
    };
    println!("y = {y}");

    // at the most formal level (in this instance acking like an anonymous function)
    let f = |x: f32| -> f32 {
        x * x
    };
    println!("f(2.0) = {}", f(2.0));
    
    // advanced examples
    let hector = Person{ first_name: "Hector", second_name: None, age: 11};
    println!("Hector {}", hector.second_name.unwrap_or_else(|| {"Ioannides"}));

    let me = Person::new("Alex", "Ioannides", 44);
    let bee = Person::new("Bianca", "Ioannides", 46);

    let mut us = vec![me, bee, hector];
    let our_ages: Vec<_> = us.iter().map(|e| e.age ).collect(); // lazy by default - need collect to force evaluation
    let combined_age: u32 = our_ages.iter().sum();
    println!("Our combined age is {combined_age}");

    us.sort_by_key(|e| e.age);
    let youngest = &us[0];
    println!("The yougest is {}", youngest.first_name);
}

struct Person {
    first_name: &'static str,  // not using String just to be difficult...
    second_name: Option<&'static str>,
    age: u32,
}

impl Person {
    fn new(first_name: &'static str, second_name: &'static str, age: u32) -> Person {
        Person{ first_name, second_name: Some(second_name), age }
    }
}
