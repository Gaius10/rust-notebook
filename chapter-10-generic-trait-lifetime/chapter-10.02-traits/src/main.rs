#![allow(unused_variables)]
#![allow(dead_code)]

// TLDR:
// Traits are the almost-equivalent of PHP's interfaces

use aggregator::{SocialPost, Summary};

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("1 new social post: {}", post.summarize());
}

// Interesting point: we can't implement external traits on external
// types. See the docs for more details about the orphan's rule.
//
// From The Book:
//
// """
// But we can’t implement external traits on external types. For
// example, we can’t implement the Display trait on Vec<T> within 
// our aggregator crate because Display and Vec<T> are both defined
// in the standard library and aren’t local to our aggregator crate.
// This restriction is part of a property called coherence, and more
// specifically the orphan rule, so named because the parent type is
// not present. This rule ensures that other people’s code can’t
// break your code and vice versa. Without the rule, two crates
// could implement the same trait for the same type, and Rust
// wouldn’t know which implementation to use.
// """
//

