#![allow(unused_variables)]
#![allow(dead_code)]

use std::io;
use std::fs;
use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() {
    let greeting_file_result = File::open("hello.txt");
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}")
            }
        },
    };

    // Cleaner  version of same thing:
    let _greeting_file = File::open("hello1.txt").unwrap_or_else(
        |error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello1.txt").unwrap_or_else(|error| {
                    panic!("Panic creating the file: {error:?}");
                })
            } else {
                panic!("Problem opening the file: {error:?}");
            }
        }
    );
    ////////////////////////////////////////////////
    // CHALLENGE: Come back here after chapter 13 //
    ////////////////////////////////////////////////

    // Result<T, E>::unwrap method is a shortcut for when we want
    // to panic if something goes wrong:
    let _greeting_file = File::open("hello2.txt").unwrap();

    // Result<T, E>::expect is similar, but makes possible for us
    // to customize panic! message:   
    let _greeting_file = File::open("hello3.txt").expect(
        "hello3.txt should be included in this project."
    );


    // Error propagation:
    read_username_from_file().expect("Error reading username.");
    short_read_username_from_file().expect("Error reading file.");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello4.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn short_read_username_from_file() -> Result<String, io::Error> {
    //
    // The ? operator is a shortcut for propagating errors, so this
    // function is almost equivalent to the one above.
    //
    // For more details about the difference between match and ?,
    // check the documentation.
    // (basically, ? uses the From trait to convert any error to
    // the current function's error type before returning it)
    // 
    let mut username_file = File::open("hello5.txt")?;
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn even_more_short_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello6.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn oneline_read() -> Result<String, io::Error> {
    // As reading to string is a fairly common operation, Rust
    // provides this convenience function.
    fs::read_to_string("hello7.txt")
}

//
// Obs.: the ? operator can only be used inside functions whose
// return type is compatible with it. So, for instance, it's not
// possible to use it directly inside the main function.
//
// P.S.: actually it is. But we have to change return type of main
// function to something like Result<(), Err<T>>
//
// The compatible return types are Result, Option and types that
// implements FromResidual.
//
// Obs.: if the return type of the function is Option, ? may be only
// used on Options, same for Results or other types.
//
// Detail: Rust's main function is compatible with C's at the
// concerns of return values (0 -> success, anything -> fails)














