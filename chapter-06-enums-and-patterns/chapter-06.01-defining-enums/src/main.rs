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
}

