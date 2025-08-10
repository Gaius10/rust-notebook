#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // Box abstraction provides only the indirection of keeping
    // data on the heap instead of the stack, nothing more than
    // that.
    let b = Box::new(5);
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
