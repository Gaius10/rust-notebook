#![allow(unused_variables)]
#![allow(dead_code)]

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let _a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let _b = Cons(3, Rc::clone(&_a));
    let _c = Cons(4, Rc::clone(&_a)); // This wouldn't be possible using Box<T>

    // Important convention:
    //
    // In this above case, we could've called a.clone instead of Rc::clone, but
    // is convention to use Rc::clone in this kind of situations.
    //
    // Difference is that most implementations of .clone() makes deep copies of
    // data inside variable, which takes much time. On the other side, Rc::clone
    // only increments the reference counter without making a deep copy, which
    // is faster.
    //
    // So writing Rc::clone rather than a.clone makes easy to visually distinguish
    // between deep and shallow copies.

    // Printing things to understand what is happening...
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("Count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("Count after creating c = {}", Rc::strong_count(&a));
    }

    println!("Count after c goes out of scope = {}", Rc::strong_count(&a));
}

