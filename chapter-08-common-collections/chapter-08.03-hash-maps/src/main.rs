#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // HashMap::insert takes ownership of its params
    // It's also possible to borrow with &var
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // HashMap::get returns an Option<&V>
    let score = scores.get(&team_name)
                      .copied()
                      .unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Overwriting:
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    // Add if not exists:
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    // Update value based on the old one:
    let text = "Hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

    // By default, HashMap uses SipHash as hasing function.
    // It's secure, but not so fast, so it's possible to change
    // the hashing function. More about this in chapter 10.
}














