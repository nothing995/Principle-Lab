// Calculates the power of 2 using a bit shift.
// `1 << n` is equivalent to "2 to the power of n".
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // You can optionally experiment here.
	let exp1 = 3;
	let exp2 = 5;
	println!("2^{} = {}", exp1, power_of_2(exp1)); // Should print: 2^3 = 8
	println!("2^{} = {}", exp2, power_of_2(exp2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // TODO: Test the function `power_of_2` with some values.
        assert_eq!(power_of_2(0), 1);   
        assert_eq!(power_of_2(1), 2); 
        assert_eq!(power_of_2(3), 8);  
        assert_eq!(power_of_2(5), 32)
    }
}