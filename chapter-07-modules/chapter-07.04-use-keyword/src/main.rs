#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use rand::Rng;

// use std::{cmp::Ordering, io};
// Same as:
// use std::cmp::Ordering;
// use std::io;


// use std::io::{self, Write};
// Same as:
// use std::io;
// use std::io::Write;

// Bringing all submodules:
// this is not a very good option, prefer to bring modules
// individually, it's more readable.
use std::collections::*;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}

