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

fn main() {
    let home = IpAddr::V4(Ipv4Addr {});
    let loopback = IpAddr::V6(Ipv6Addr {});
}

