// Taken from Chapter 4 of The Rust Programming Language

fn main() {
    // ~~~~~~~~~~~~
    // ~~ SCOPES ~~
    // ~~~~~~~~~~~~

    // simple scopes using data on the stack
    let result_1 = {
        let x = 1;
        let y = x;  // this will copy x on the stack as it's a literal value (fixed size)
        x + y
    };
    // println!("x + y = {x} + {y} = {result_1}"); // this will cause a panic because x is no longer in scope
    println!("x + y = 1 + 1 = {result_1}");

    // more complex scopes using data on the heap
    let s1 = String::from("hello"); // create a string on the heap
    let s2 = s1; // this will NOT make a copy of s1
    // println!("{}, world!", s1); // this will panic as s1 is no longe in scope
    println!("{}, world!", s2); // this doesn't cause a panic as s2 currently owns the string on the heap 

    let s3 = s2.clone();
    println!("{}, world!", s3); // this is okay as we've made a copy of s2

    // ~~~~~~~~~~~~~~~
    // ~~ OWNERSHIP ~~
    // ~~~~~~~~~~~~~~~
    let p = 1;
    f_int(p);
    println!("p = {p}");

    let q0 = String::from("1");
    f_str0(q0);
    // println!("q = {q0}"); // this causes a panic as f_str has taken ownership of q

    let mut q1 = String::from("1");
    q1 = f_str1(q1);
    println!("q = {q1}");

    let q2 = String::from("1");
    f_str2(&q2);
    println!("q = {q2}");

    let mut q3 = String::from("1");
    f_str3(&mut q3);
    println!("q = {q3}");

    let mut q4 = String::from("1");
    // f_str4(&mut q4, &mut q4); // this will cause a panic as we're only allowed one mutable reference
    println!("q = {q4}");

    let mut q5 = String::from("1");
    // f_str5(&mut q5, &q5); // this will cause a panic as we're not permitted concurrent mutable and immutable references
    println!("q = {q5}");

    // ~~~~~~~~~~~~
    // ~~ SLICES ~~
    // ~~~~~~~~~~~~
    let s = "literally this";
    let first_word = &s[..9];
    println!("first_word = {first_word}");

    let a: [i32; 5] = [0, 1, 2, 3, 4];
    let last_element = &a[4..];
    println!("last_element = {}", last_element[0]);
}

fn f_int(x: i32) { // copies literal value
    println!("f(x) = {x}");
}

fn f_str0(x: String) { // take ownership of x
    println!("f(x) = {x}");
}

fn f_str1(x: String) -> String { // take ownership of x and returns it
    println!("f(x) = {x}");
    x
}

fn f_str2(x: &String) { // borrows x
    println!("f(x) = {x}");
}

fn f_str3(x: &mut String) { // borrows x and mutates it
    x.push_str(".0");
    println!("f(x) = {x}");
}

fn f_str4(x: &mut String, y: &mut String) { // tries to borrow x and mutate it twice in the same scope
    x.push_str(".0");
    y.push_str(".0");
    println!("f(x) = {x}");
    println!("f(y) = {y}");
}

fn f_str5(x: &mut String, y: &String) { // tries to borrow x and mutate it twice in the same scope
    x.push_str(".0");
    println!("f(x) = {x}");
    println!("f(y) = {y}");
}
