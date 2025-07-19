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

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, another: &Rectangle) -> bool {
        self.width > another.width &&
        self.height > another.height
    }

    //
    // This is not a method, as it doesn't depends on a `self`
    // parameter, but it's an associated function.
    // Obs.: another example of associated function that is not a
    // method is String::new().
    //
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let square = Rectangle::square(30);

    if rect1.width() {
        println!(
            "The rectangle has a nonzero width. It is {}.",
            rect1.width
        );

        println!(
            "The area of the rectangle is {} square pixels",
            rect1.area()
        );
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("Square: {:#?}", square);
}

