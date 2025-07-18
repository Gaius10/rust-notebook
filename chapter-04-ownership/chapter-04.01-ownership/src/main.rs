#![allow(unused_variables)]

fn main() {
    let s = "hello"; // This is a string literal.
                     // It's valid from here to the end of
                     // current scope.

    // In Rust, data which size is known at compile time
    // is stored on the stack. Otherwise, it goes to the heap
    // memory (so, the above example uses stack).

    // The whole ownership system is a way to manage data that
    // is store in the heap.

    // string literals aren't the best approach for all situations,
    // some of the reasons:
    // - they are immutable;
    // - sometimes we don't know it's value at compile time;

    // The String type:
    // This is a more flexible data type to handle strings. It uses
    // heap to store data, is mutable and doesn't need to be known
    // at compile time.
    let mut s = String::from("Hello");
    s.push_str(", world!"); // this kind of operation is not
                            // possible with string literals
    println!("{s}");

    // Ex.: both values goes to stack
    let x = 5;
    let y = x;

    // Ex.: not when using String type
    let s1 = String::from("Hello");
    let s2 = s1;
    // Obs.: in other languages, this operation could be called
    // "shallow copy", in rust, as s1 is invalidated, it's called
    // a "move" - s1 was moved into s2.

    // Obs. 2: Rust never creates deep copies automatically.

    // Obs. 3: part of the String data is stored on stack, but the
    // string itself (main part of data) is what goes to heap. See
    // docs for more details.

    // In this last example, s2 is not a copy of s1, but a reference
    // to the same part of memory. Also, s1 is invalidated in order
    // to avoid double free bugs.
    //
    // println!("{s1}, world!"); // This throws an error
    //
    // See the docs for more details.

    // Ex. where data is deeply copied in heap:
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");
    // For other kinds of objects, clone method may be an arbitrary
    // code.

    // Obs.: below example works (x is not invalidated)
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");
    // This happens because size of x is known at compile time, so
    // it's stored on stack and can by easily deeply copied.


    // Illustrating ownership and moves in functions:
    let s = String::from("Some string");
    let x = 4;
    takes_ownership(s);
    makes_copy(x);
    // println!("s = {s}"); // This would throw an error, as s is
                            // moved to inner scope.
    println!("x = {x}"); // This works, as x is copied on stack.

    // Returning ownership:
    let s1 = gives_ownership(); // value returned is moved to s1
    println!("Returned s1: {s1}");

    let s2 = String::from("Hello");
    println!("Created s2: {s2}");

    let s3 = takes_and_gives_ownership(s2);
    println!("Returned s2 (s3): {s3}");

    // Problem of ownership system: boring moves in and out of scope
    let s1 = String::from("Hello");
    let (s1, len) = calculate_len(s1);
    println!("The length of '{s1}' is {len}.");
    // Solution in the next chapter
}

fn takes_ownership(some_string: String) {
    println!("some_string = {some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("some_integer = {some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("Yours");
    some_string
}

fn takes_and_gives_ownership(some_string: String) -> String {
    println!("some taken string = {some_string}");
    some_string
}

fn calculate_len(some_string: String) -> (String, usize) {
    let len = some_string.len();
    (some_string, len)
}


