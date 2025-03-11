trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement the trait `AppendBar` for a vector of strings.
// `append_bar` should push the string "Bar" into the vector.
impl AppendBar for Vec<String>{
	fn append_bar(mut self ) -> Self {
		self.push(String::from("Bar"));
		self } }
fn main() {
 
  let mut foo = vec![String::from("Foo")].append_bar();
    println!("Last element: {}", foo.pop().unwrap());
    println!("Second last element: {}", foo.pop().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}