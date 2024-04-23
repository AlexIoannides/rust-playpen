// Taken from Chapter 15 of The Rust Programming Language
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;
use crate::List::{Cons, Nil};

fn main() {
    // revisiting pointers and references
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y); // need to dereference in order to get value (don't need to do this in function calls via syntacitc sugar)

    // storing data on the heap with Box
    let int_on_the_heap = Box::new(5);
    println!("Straight from the heap: {}", int_on_the_heap);
    f(&int_on_the_heap); // borrowing
    g(int_on_the_heap); // taking ownership and dropping out-of-scope

    // enabling recursive types
    // let _cons_list = Cons(1, Cons(2, Cons(3, Nil))); // not viable
    let cons_list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    match cons_list {
        Cons(a, _b) => println!("Cons({a}, Cons(...)"),
        Nil => println!("Nil"),
    }

    // playing with our DIY smart-pointer
    let q = 5;
    let r = MyBox::new(5);
    assert_eq!(5, q);
    assert_eq!(5, *r); // only works because we implemented Deref

    let r = q + *r; // `*` calls deref implicitly. Note, `q + r` won't compile as the Add trait hasn't been defined.
    println!("r = q + *r = {r}");
    h(&r); // test implicit dereferencing

    let s = MyBox::new(42);
    mem::drop(s); // the only way to drop early --> forces a call to self.drop that cannot be called directly

    // playing with reference counting
    let m = Rc::new(42);
    println!("m = {m}; n_refs = {}", Rc::strong_count(&m));
    eat_ref(Rc::clone(&m));
    eat_ref(Rc::clone(&m));
    println!("m = {m}; n_refs = {}", Rc::strong_count(&m));

    // playing with mutable references
    let n = Rc::new(RefCell::new(42));
    println!("n = {}; n_refs = {}", *n.borrow(), Rc::strong_count(&n));
    mut_ref(n.clone()); // can also use .clone(), but convention is to use `Rc::clone` to distinguish it from standard clone methods
    mut_ref(n.clone());
    println!("n = {}; n_refs = {}", *n.borrow(), Rc::strong_count(&n));

    // note, r goes out of scope here, so we should see the drop trait called and it's message printed
}

fn f(x: &Box<i32>) {
    println!("borrowed unboxing, x = {}", x); // can use like a canonical reference as Box implements Deref trait
}

fn g(x: Box<i32>) {
    println!("owned unboxing, x = {}", x); // automatic cleanup as Box implements the Drop trait
}

fn h(x: &i32) {
    println!("implicit dereferencing test, x = {x}");
}

fn eat_ref(x: Rc<i32>) {
    println!("eating a new reference, x = {x}; n_refs = {}", Rc::strong_count(&x));
}

fn mut_ref(x: Rc<RefCell<i32>>) {
    println!("@t=0, x = {}; n_refs = {}", x.borrow(), Rc::strong_count(&x));
    *x.borrow_mut() += 1;
    println!("@t=1, x = {}; n_refs = {}", x.borrow(), Rc::strong_count(&x));
}

enum List {
    // Cons(i32, List), // cannot determine size for List at compile time
    Cons(i32, Box<List>), // smart-pointer for the second tuple element constrains size in memory (to just the pointer)
    Nil,
}

// defining our own version on Box (on the stack)
use std::ops::{Deref, Drop};

struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl <T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl <T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("It's all over!");
    }
}
