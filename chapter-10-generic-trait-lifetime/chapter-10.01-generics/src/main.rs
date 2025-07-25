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

// Obs.: it could be like:
// impl Point<char> { ... }
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Fun complicated case:
impl<X1, Y1> MixedPoint<X1, Y1> {
    fn mixup<X2, Y2>(
        self,
        other: MixedPoint<X2, Y2>
    ) -> MixedPoint<X1, Y2> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let integer_point = Point { x: 1, y: 2 };
    // Error: undefined function
    // integer_point.distance_from_origin();

    // Works well
    let float_point = Point { x: 1.0, y: 2.0 };
    float_point.distance_from_origin();

    // let error_point = Point { x: 1.0, y: 2 };

    let mixed_point = MixedPoint { x: 1.0, y: 2 };

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = MixedPoint { x: 1, y: 2.0 };
    let p2 = MixedPoint { x: 5, y: 8.0 };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

