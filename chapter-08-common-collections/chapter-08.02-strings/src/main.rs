#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

fn main() {
    // Curiosity: String type is implemented as a wrapper around
    // a vector of bytes.
    let mut s = String::new();

    let data = "initial contents of any Display object";
    let s = data.to_string();
    let s = "initial data".to_string();
    let s = String::from("Something...");

    // Updating a String:
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // s2 is just borrowed
    println!("s2 is {s2}"); // s2 still valid here

    // Adding single char:
    let mut s = String::from("lo");
    s.push('l');

    // Concatenation with +:
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // This way s1 will be moved, s2 borrowed.
    // It must be this way because of the method's signature:
    // fn add(self, s: &str) -> String { ... }

    // Also we have format! for more complicated cases:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    // Obs.: format! doesn't takes ownership of any of its
    // parameters.

    // Indexing into Strings:
    let s1 = String::from("hi");
    // let h = s1[0]; // This would throw an error.
    // Rust Strings doesn't support indexing because, internally, it
    // stores strings UTF-8 encoded, which means that chars may
    // use from 1 to 4 bytes in memory, making indexing complicated.
    // Also, rust requires indexing operations to be O(1), but it's
    // not possible to do with variable char sizes.

    // Slicing:
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // let s = &hello[0..1]; // This would panic (middle of char)
    println!("{s}");


    // Iterating over Strings:
    for c in "Зд".chars() {
        println!("{c}"); // Prints each char
    }

    for b in "Зд".bytes() {
        println!("{b}"); // Prints each byte as integer
    }
}




























