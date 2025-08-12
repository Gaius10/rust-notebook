#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!(
            "Dropping CustomSmartPointer with data `{}`!",
            self.data
        );
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");

    // This throws error:
    // c.drop();
    // std::mem::drop function must be used.
    // This way Rust prevents itself from double free problems.
    std::mem::drop(c);
    println!("CustomSmartPointer dropped before the end of scope.");
}
