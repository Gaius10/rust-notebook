#![allow(unused_variables)]
#![allow(dead_code)]

enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;

    route(ipv4);
    route(ipv6);
}

fn route(ip_kind: IpAddrKind) {
}

