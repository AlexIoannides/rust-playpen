// Taken from Chapter 3 of The Rust Programming Language
const THIS_IS_THE_CONSTANT: i32 = 42;


fn main() {
    // scalar types, function calls and compound expressions
    let x; // statement
    x = 1.0; // expression (returns value)

    let y = 1.6;
    let z = add(x, y);
    println!("add({x}, {y}) = {z}");

    answer_to_ultimate_question();

    let p = {
        let x = 1.0; // defined in local scope (closure)
        x + y
    };
    println!("p = {p}");

    let q = if THIS_IS_THE_CONSTANT == 42 {42} else {0};
    println!("q = {q}");

    // compound primitive types
    let the_tup = (1, 'a', true);
    
    let (a, b, c) = the_tup; // deconstruction
    println!("a = {a} is an i32, which is the same as the_tup.0 = {}", the_tup.0);
    println!("b = {b} is a char, which is the same as the_tup.1 = {}", the_tup.1);
    println!("c = {c} is a bool, which is the same as the_tup.2 = {}", the_tup.2);

    let the_array: [i32; 5] = [1, 2, 3, 4, 5];

    // loops
    println!("the_array has {} elements:", the_array.len());
    let mut n = 0;
    for e in the_array {
        println!("-> the_array[{n}] = {e}");
        n += 1;
    }

    n = 0;
    let ans = loop {
        if n == THIS_IS_THE_CONSTANT {
            break n;
        } else {
            n += 1;
        }
    };
    println!("I have the answer! It is {ans}");

    'main_loop: loop {
        for n in (0..THIS_IS_THE_CONSTANT).rev() {
            if n == 0 {
                println!("BOOM!");
                break 'main_loop;
            };
        }
    }

}


fn add(x: f32, y: f32) -> f32 {
    x + y
}


fn answer_to_ultimate_question() {
    println!("answer_to_ultimate_question = {THIS_IS_THE_CONSTANT}")
}
