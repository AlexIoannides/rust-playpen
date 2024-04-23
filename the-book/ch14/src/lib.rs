//! # Chapter 14
//! 
//! Chapter is all about Cargo.

/// Adds two numbers
/// # Examples
/// 
/// ```
/// let x = 1;
/// let y = 2;
/// let result = ch14::add(x, y);
/// 
/// assert_eq!(3, result);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
