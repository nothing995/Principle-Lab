#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    DivideByZero,
    IntegerOverflow,
    NotDivisible,
}

fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    if b == 0 {
        Err(DivisionError::DivideByZero)
    } else if a == i64::MIN && b == -1 {
        Err(DivisionError::IntegerOverflow)
    } else if a % b != 0 {
        Err(DivisionError::NotDivisible)
    } else {
        Ok(a / b)
    }
}

fn result_with_list() -> Result<[i64; 4], DivisionError> {
    let numbers = [27, 297, 38502, 81];
    let mut result = [0; 4];
    for (i, value) in numbers.iter().enumerate() {
        result[i] = divide(*value, 27)?; // ✅ Dereference
    }
    Ok(result)
}

fn list_of_results() -> [Result<i64, DivisionError>; 4] {
    let numbers = [27, 297, 38502, 81];
    let mut result: [Result<i64, DivisionError>; 4] = [
        Ok(0),
        Ok(0),
        Ok(0),
        Ok(0),
    ];

    for (i, value) in numbers.iter().enumerate() {
        result[i] = divide(*value, 27);
    }

    result
}

fn main() {
    // Print single division
    match divide(81, 9) {
        Ok(value) => println!("Divide 81 by 9: {}", value),
        Err(e) => println!("Divide 81 by 9: {:?}", e),
    }

    // Print the unwrapped result list
    match result_with_list() {
        Ok(array) => println!("Result with list: {:?}", array),
        Err(e) => println!("Result with list: Error - {:?}", e),
    }

    // Print each result separately
    let list = list_of_results();
    print!("List of results: [");
    for (i, result) in list.iter().enumerate() {
        match result {
            Ok(val) => print!("{}", val),
            Err(e) => print!("{:?}", e),
        }
        if i < list.len() - 1 {
            print!(", ");
        }
    }
    println!("]");
}
