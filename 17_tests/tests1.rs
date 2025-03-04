// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
	let num1 = 4;
	let num2 = 7;
	println!("Is {} even? {}", num1, is_even(num1)); 
	println!("Is {} even? {}", num2, is_even(num2));
}

#[cfg(test)]
mod tests {
    // TODO: Import `is_even`. You can use a wildcard to import everything in
    // the outer module.

    #[test]
    fn you_can_assert() {
        // TODO: Test the function `is_even` with some values.
        assert!(is_even(2));  
        assert!(!is_even(3)); 
    }
}