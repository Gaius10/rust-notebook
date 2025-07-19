#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    //
    // `self` is of type &Self, alias for the current type.
    //
    // Also, we could use `self`, or `&mut self`.
    //
    // Obs.: using just `self` is rare, but useful when we need to
    // invalidate self after some transformation.
    //
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );
}

