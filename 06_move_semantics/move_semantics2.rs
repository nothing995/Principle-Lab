fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
     
    let vec0 = vec![22, 44, 66];
    println!("Original vector: {:?}", vec0);


    let vec1 = fill_vec(vec0.clone());
    println!("Updated vector: {:?}", vec1);

    if vec0 == vec![22, 44, 66] && vec1 == vec![22, 44, 66, 88] {
        println!("Test passed: Function works as expected!");
    } else {
        println!("Test failed: Unexpected result.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        let vec1 = fill_vec(vec0.clone());

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
