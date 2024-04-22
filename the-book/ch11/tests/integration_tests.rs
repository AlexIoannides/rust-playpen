// Taken from Chapter 11 of The Rust Programming Language
// --> example of 'integrations' tests for library code that test publically exposed APIs
use ch11;

mod common;

#[test]
fn adds_two() {
    common::setup();
    assert_eq!(ch11::add(2, 2), 4)
}
