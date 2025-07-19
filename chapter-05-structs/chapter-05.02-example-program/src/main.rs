#![allow(unused_variables)]
#[derive(Debug)] // Makes Rectangle printable with :? specifier
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // dbg! returns ownership of value
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );


    println!("rect1 is {rect1:?}");
    println!("rect1 is {rect1:#?}");

    let rect1 = dbg!(&rect1); // Passing reference to avoid giving
                              // ownership to dbg! macro.
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

