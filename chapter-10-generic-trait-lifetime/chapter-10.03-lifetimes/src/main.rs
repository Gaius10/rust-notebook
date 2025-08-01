#![allow(unused_variables)]
#![allow(dead_code)]

//
// Generic lifetime parameters describes how the lifetimes of the
// parameters and of the return type relates to each other.
//
// Here, we express: "the returned reference will be valid as long
// as both the parameters are valid."
//
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    // Example with two equal lifetimes
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("The longest string is '{result}'");

    // Example with two different lifetimes
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is '{result}'");
    }

    // Example with an ended lifetime:
    /*let string1 = String::from("long string is long");
    let result: &str;

    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }

    println!("The longest string is '{result}'");*/

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{i:?}");

    // static lifetime:
    let s: &'static str = "I have a static lifetime";
    // `s` lives as long as the entire application.
    // Have caution with error messages that suggests using this,
    // they may be the result of tries of creating dangling refs.
}

// In some cases, Rust compiler can infer lifetimes:
// fn first_word<'a>(s: &'a str) -> &'a str { ... };
// is the same as
// fn first_word(s: &str) -> &str { ... };
//
////////////////////////////////////////////////////////////////////
// Lifetime elision rules: (lifetime inference)
//
// - 1. Each reference parameter receives an lifetime parameter;
//          Ex.: fn foo<'a, 'b>(x: &'a i32, y: &'b i32) -> &i32
//
// - 2. If there is exatctly one input lifetime parameter, that
//      lifetime is assigned to all output lifetimer parameters;
//          Ex.: fn foo<'a>(x: &'a i32) -> &'a i32
//
// - 3. If there are multiple input lifetime parameters, but one of
//      them is &self or &mut self because this is a method, the
//      lifetime of self is assigned to all output lifetime
//      parameters.
//
// - Note: it's possible that in the future more lifetime elision
//  rules come up, as more patterns are discovered.
////////////////////////////////////////////////////////////////////

use std::fmt::Display;

fn longest_with_an_announcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}

