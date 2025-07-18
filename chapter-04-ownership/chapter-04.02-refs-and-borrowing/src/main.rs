#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    ///////////////////////////////////////////////////////////////
    // Passing a reference as argument doesn't move the variable,
    // so it's not needed to return it back at the end of inner
    // scope.
    //
    // Unlike a pointer, references guarantees to point to a valid
    // value for the life of that reference.
    //
    // The act of passing a reference as argument is called
    // "borrowing", as it doesn't transfer ownership.
    //
    let s1 = String::from("Hello");
    let len = calculate_length(&s1); // This is called "borrowing"
    println!("The length of '{s1}' is {len}.");


    // It's not possible to modify a variable that's borrowed!
    // As variables are, by default, immutable, so are references.
    // let s = String::from("Hello");
    // try_to_change(&s); // This would throw an error.

    let mut s = String::from("Hello");
    change(&mut s);
    println!("{s}");

    ///////////////////////////////////////////////////////////////
    // Important: each mutable variable can only have one mutable
    // reference at a time.
    //
    // This restriction prevent data race problems. See docs for
    // more details.
    //
    let mut s = String::from("Hello");
    let r1 = &mut s;

    // let r2 = &mut s; // This would throw an error at compile time
    // println!("{}, {}", r1, r2);
    println!("r1 = {r1}");

    //
    // Another similar restriction is in this case:
    // also is prohibited to have both mutable and immutable
    // references of a value at the same time.
    //
    // It prevents unexpected behavior. Users of immutable refs
    // doesn't expect it's value to be mutated.
    //
    let mut s = String::from("Hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG problem
    // println!("{}, {} and {}", s1, s2, s3); // throws error

    // Important observation: scopes of references end at the last
    // time the ref is used.
    // This means that the above example could be rewrote, with no
    // problem, as:
    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;
    println!("r1 {}, r2 {}", r1, r2);
    // Here, as r1 and r2 are not used anymore, they are out of
    // scope.

    let r3 = &mut s;
    println!("r3 {r3}");

    //
    // Cool point of rust: it prevent dangling pointers at compile
    // time:
    // let reference_to_nothing = dangle();
    // This is related to lifetimes, which will be covered at a
    // later moment in this course.
    //
    // But there is an alternative:
    let reference = no_dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn try_to_change(some_string: &String) {
    // some_string.push_str(", world!"); // This throws an error.
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

/*fn dangle() -> &String {
    let s = String::from("Hello");
    &s
    // This tries to return a reference to a value which will be
    // deallocated here.
}*/

fn no_dangle() -> String {
    let s = String::from("Hello");
    s
    // This moves the ownership out, nothing is deallocated.
}

