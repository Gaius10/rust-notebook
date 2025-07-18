#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

fn main() {
    //////////////////////////////////////////////////////////////
    // Example problem:
    //
    // Take a string of words separated by spaces and return the
    // first word found there.  
    //
    let mut s = String::from("Hello");
    let word = first_word(&s);
    s.clear(); // From now on, `word` is outdated.
    // do_something(&s, word); // BUG!
    // As `word` isn't connected to `s` at all, Rust compiler can't
    // even throw an error. The bugs just happens.

    //
    // This problem becomes bigger as the context is more complex.
    // For instance, if a new function `second_word` is implemented
    // we'll have another variable floating around that needs to be
    // synced with `s`.
    //
    // Solution: slices.
    //
    // A string slice is a reference to a part of a String.
    let s = String::from("Hello world!");
    let hello = &s[0..5];
    let world = &s[6..11];
    // Internally, Rust stores a reference to the starting point of 
    // the slice and it's size.
    let mut s = String::from("Hello world!");
    let f_word = cool_first_word(&s);
    // s.clear(); // This would throw an error.
                  // (`s` already borrowed as immutable)
    println!("cool first word: {f_word}");

    // Obs.: string literals are slices pointing to a part of
    // the binary.
    let s: &str = "Hello, world!";
    let f_word = cool_first_word(s);
    println!("cool first word from literal: {f_word}");

    //
    // Other slices:
    // String slices are a string specific slice type, but there
    // are others.
    //
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    // Nothing very complicated to comment, the work here is similar
    // to string slices.
    // It's possible to make slices of any collection.
    // Mode details about collections at chapter 8
}

//
// Will return the index of the first space encountered.
//
// This implementation is not the best, because the returned number
// is not directly associated with `s`, so if `s` changes, the
// returned number becomes outdated.
//
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// Typing `s` as &str allows it to receive both &String and &str
// types as argument.
fn cool_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

