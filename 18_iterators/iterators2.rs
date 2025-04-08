// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| capitalize_first(word)).collect()
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|word| capitalize_first(word)).collect()
}

fn main() {
    // Test capitalize_first
    println!("{}", capitalize_first("hello")); // Should print "Hello"
    println!("{}", capitalize_first(""));      // Should print ""

    // Test capitalize_words_vector
    let words = vec!["hello", "world"];
    let capitalized_vector = capitalize_words_vector(&words);
    println!("{:?}", capitalized_vector); // Should print ["Hello", "World"]

    // Test capitalize_words_string
    let sentence_parts = vec!["hello", " ", "world"];
    let capitalized_string = capitalize_words_string(&sentence_parts);
    println!("{}", capitalized_string); // Should print "Hello World"
}


