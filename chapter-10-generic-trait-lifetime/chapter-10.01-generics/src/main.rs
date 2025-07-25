#![allow(unused_variables)]
#![allow(dead_code)]

use std::cmp::PartialOrd;

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Just an example with structs:
struct Point<T> {
    x: T,
    y: T,
}

struct MixedPoint<T, U> {
    x: T,
    y: U,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let integer_point = Point { x: 1, y: 2 };
    let float_point = Point { x: 1.0, y: 2.0 };
    // let error_point = Point { x: 1.0, y: 2 };
    let mixed_point = MixedPoint { x: 1.0, y: 2 };
}

