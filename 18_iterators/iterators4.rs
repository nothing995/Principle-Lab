fn factorial(num: u64) -> u64 {
    (1..=num).fold(1, |acc, x| acc * x)
}

fn main() {
    // Optionally test the factorial function here if you want.
    println!("Factorial of 5 is: {}", factorial(5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
