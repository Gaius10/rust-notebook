#![allow(unused_variables)]
#![allow(dead_code)]

use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(
        &self,
        user_preference: Option<ShirtColor>
    ) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Blue,
        ],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );


    //
    // When using closures, there is no need to annotate types (rare
    // exception cases).
    //
    // However, here's an example type annotated:
    //
    let _expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };


    // Few examples for syntax comparison:
    fn  _add_one_v1   (x: u32) -> u32 { x + 1 }
    let _add_one_v2 = |x: u32| -> u32 { x + 1 };
    // (error in this concrete case):
    // let _add_one_v3 = |x|             { x + 1 };
    // let _add_one_v4 = |x|               x + 1  ;

    // It's not possible to mix closure inferred types.
    // The following crashes:
    //
    // let example_closure = |x| x;
    //
    // let s = example_closure(String::from("hello"));
    // let n = example_closure(5);
    //
    // At `s` definition, Rust compiler will bind type String to
    // `x` in the closure, so the error happens in the next line.
    //

    //
    // As functions, closures can take mutable/immutable references
    // or the ownership of its arguments.
    //
    // Borrowing example:
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");


    // Mutably borrowing example:
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    // Throws error because list is borrowed by the closure
    // definition:
    //
    // println!("Before calling closure: {list:?}");
    //

    borrows_mutably();
    println!("After calling closure: {list:?}");


    // Moving values into closures: (keyword `move`)
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    //
    // The three traits that closures can implement:
    //
    // - FnOnce: all closures implement at least this trait, as all
    //           closures can be called. A closure that moves
    //           captured values implement only this trait;
    // - FnMut:  closures that capture mutable references (those can
    //           be called more than once);
    // - Fn:     closures that doesn't moves captured values and
    //           also doesn't mutates them.
    //

    // Getting closures as arguments:
    // impl<T> Option<T> {
    //     pub fn unwrap_or_else<F>(self, f: F) -> T
    //     where
    //         F: FnOnce() -> T
    //     {
    //         match self {
    //             Some(x) => x,
    //             None => f(),
    //         }
    //     }
    // }

    let mut list = [
        Rectangle { width: 10, height: 1  },
        Rectangle { width: 3,  height: 5  },
        Rectangle { width: 7,  height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{list:#?}");
}
