// Taken from Chapter 11 of The Rust Programming Language
// --> example of embedded unit tests in library code

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn sub(left: usize, right: usize) -> usize {
    right - left
}

pub fn mult(left: usize, right: usize) -> usize {
    left * right
}

pub fn div(numerator: usize, denominator: usize) -> usize {
    if denominator == 0 {
        panic!("divide by zero error!");
    }
    numerator / denominator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn subtraction() -> Result<(), String> {
        if sub(2, 3) == 1 {
            Ok(())
        } else {
            Err(String::from("Invalid subtraction"))
        }

    }

    #[test]
    fn test_multiplication() {
        let result = mult(3, 2);
        assert_eq!(result, 6);
        assert_ne!(result, 7);
    }

    #[test]
    fn test_division() {
        let numerator = 2;
        let demoninator = 2;
        let result = div(numerator, demoninator);
        assert_eq!(result, 1, "division failed: {} / {} != {}", numerator, demoninator, result);
    }

    #[test]
    #[should_panic]
    fn division_by_zero() {
        div(2, 0);
    }

}
