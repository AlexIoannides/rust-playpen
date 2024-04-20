// Taken from Chapter 9 of The Rust Programming Language

pub struct SomeValue {
    value: i32,
}

impl SomeValue {
    pub fn new(value: i32) -> SomeValue {
        if value < 0 || value > 100 {
            panic!("Value must be between 0 and 100, got {}.", value);  // unrecoverable
        }

        SomeValue { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn process_value(v: &SomeValue) -> Result<i32, &str> { // recoverable
    if 2 * v.value > 100 {
        return Err("2 x v > 100 - don't like this");
    }
    Ok(2 * v.value)
}

fn main() {
    let sv0 = SomeValue::new(42);
    let psv0 = match process_value(&sv0) {
        Ok(v) => v,
        Err(e) => {
            println!("setting output to 0 - {e}");
            0
        }
    };
    println!("process_value(..) = {psv0}");
    println!("process_value(..) = {}", process_value(&sv0).unwrap());

    let sv1 = SomeValue::new(52);
    let psv1 = match process_value(&sv1) {
        Ok(v) => v,
        Err(e) => {
            println!("setting output to 0 - {e}");
            0
        }
    };
    println!("process_value(..) = {psv1}");
    println!("process_value(..) = {}", process_value(&sv1).expect("nope: "));
}

// process_value(..) = 84
// process_value(..) = 84
// setting output to 0 - 2 x v > 100 - don't like this
// process_value(..) = 0
// thread 'main' panicked at src/main.rs:49:60:
// nope: : "2 x v > 100 - don't like this"
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace