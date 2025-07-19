#![allow(unused_variables)]
#![allow(dead_code)]

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

//
// This is similar to defining multiple different structs,
// but here they're grouped, so we could write a function that
// accepts anyone of them as argument.
//
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // do something...
    }
}

fn main() {
    let home = IpAddr::V4(Ipv4Addr {});
    let loopback = IpAddr::V6(Ipv6Addr {});
    let m = Message::Write(String::from("Hello"));
    m.call();

    // Rust have no implementation of the null value, but it's
    // possible to deal with absent values through Option<T> enum.
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    // This approach is better than having null values because
    // this way Rust compiler can prevent us of making invalid
    // operations with T and null, since T and Option<T> are
    // diffent types.
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; // Would'n compile
}






