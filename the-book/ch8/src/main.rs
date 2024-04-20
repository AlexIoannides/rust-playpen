// Taken from Chapter 8 of The Rust Programming Language

use std::collections::HashMap;

fn main() {
    // vectors!
    let mut v0: Vec<i32> = Vec::new();
    v0.push(1);
    v0.push(2);
    v0.push(3);

    let mut count = 0;
    println!("v0:");
    for e in &v0 { // note, that we use &v everywhere so that we borrow and don't take ownership
        println!("  v0[{count}] = {e}");
        count += 1;
    }

    let v1 = vec![1, 2, 3];
    count = 0;
    println!("\nv1:");
    for e in 0..v1.len() {
        println!("  v1[{count}] = {}", &v1[e]);
        count += 1;
    }

    // Hash Maps
    let mut dict = HashMap::new();
    dict.insert("foo", "bar");
    dict.insert("smeg", "head");
    println!("\ndict:");
    for (k, v) in &dict {
        println!("  {k}: {v}");
    }

    println!("\ndict.get(\"smeg\")...");
    match dict.get("smeg") {
        Some(e) => println!("  smeg={e}"),
        None => println!("  no smeg here!")
    }
}
