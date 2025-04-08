use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if *val == value {
            count += 1;
        }
    }
    count
}

// TODO: Implement the functionality of `count_for` but with an iterator instead
// of a `for` loop.
fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // `map` is a hash map with `String` keys and `Progress` values.
    // map = { "variables1": Complete, "from_str": None, … }
    map.values()
        .filter(|&&v| v == value)
        .count()
}

fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for map in collection {
        for val in map.values() {
            if *val == value {
                count += 1;
            }
        }
    }
    count
}

// TODO: Implement the functionality of `count_collection_for` but with an
// iterator instead of a `for` loop.
fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    // `collection` is a slice of hash maps.
    // collection = [{ "variables1": Complete, "from_str": None, … },
    //               { "variables2": Complete, … }, … ]
    collection.iter()
        .flat_map(|map| map.values())
        .filter(|&&v| v == value)
        .count()
}

fn main() {
    // Test the counting functionality in the main function.
    let map = {
        use Progress::*;
        let mut m = HashMap::new();
        m.insert(String::from("variables1"), Complete);
        m.insert(String::from("functions1"), Complete);
        m.insert(String::from("hashmap1"), Complete);
        m.insert(String::from("arc1"), Some);
        m.insert(String::from("as_ref_mut"), None);
        m.insert(String::from("from_str"), None);
        m
    };

    println!("Count of Complete exercises: {}", count_iterator(&map, Progress::Complete));
    println!("Count of Some exercises: {}", count_iterator(&map, Progress::Some));
    println!("Count of None exercises: {}", count_iterator(&map, Progress::None));

    let collection = vec![
        map.clone(),
        {
            use Progress::*;
            let mut m = HashMap::new();
            m.insert(String::from("variables2"), Complete);
            m.insert(String::from("functions2"), Complete);
            m.insert(String::from("if1"), Complete);
            m.insert(String::from("from_into"), None);
            m.insert(String::from("try_from_into"), None);
            m
        },
    ];

    println!(
        "Count of Complete exercises in collection: {}",
        count_collection_iterator(&collection, Progress::Complete)
    );
    println!(
        "Count of Some exercises in collection: {}",
        count_collection_iterator(&collection, Progress::Some)
    );
    println!(
        "Count of None exercises in collection: {}",
        count_collection_iterator(&collection, Progress::None)
    );
}
