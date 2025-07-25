#![allow(unused_variables)]
#![allow(dead_code)]

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!(
            "(Read more from {}...)",
            self.summarize_author(),
        )
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        String::from("some great author")
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
//
// Obs.:
//
// pub fn notify<T: Summary>(item: &T) {
// Is equiv. to:
// pub fn notify(item: &impl Summary) {
//
// use one or another for different situations. See the docs for
// more details.
//
// Multiple trait bound is possible with:
// &(impl Sometrait + Othertrait)
// or
// <T: Sometrait + Othertrait>
//
// Clearer syntax for multiple trait bounds:
//
// fn some_function<T: Display + Clone, U: Clone + Debug>(
//     t: &T, u: &U
// ) -> i32 { ... }
//
// is the same as
//
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// { ... }
//

// Returning traits:
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}

// Using all of this it's possible to conditionally implement traits
// see the docs for more details and search for blanket
// implementations.

