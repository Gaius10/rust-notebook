#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

fn main() {
    // Type annotation needed when empty:
    let v: Vec<i32> = Vec::new();
    println!("{v:?}");

    // More convenient way:
    let v = vec![1, 2, 3];
    println!("{v:?}");

    // Mutating:
    let mut v = vec![1, 2, 3, 4, 5];
    println!("{v:?}");

    // Appending values:
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{v:?}");

    // Reading values via index:
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}.");

    // Reading values via get method:
    let v = vec![1, 2];
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}."),
        None => println!("There is no third element."),
    }

    // Same ownership rules applies:
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {first}.");
    // The above would throw an error related to an immutable
    // and a mutable reference being created in the same scope.
    //
    // Interesting explanation from the docs:
    //
    // The code might look like it should work: why should a
    // reference to the first element care about changes at the end
    // of the vector? This error is due to the way vectors work:
    // because vectors put the values next to each other in memory,
    // adding a new element onto the end of the vector might require
    // allocating new memory and copying the old elements to the new
    // space, if there isn’t enough room to put all the elements
    // next to each other where the vector is currently stored. In
    // that case, the reference to the first element would be
    // pointing to deallocated memory. The borrowing rules prevent
    // programs from ending up in that situation.
    //


    // Iterating:
    // Again, ownership rules make it safe.
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }

    // Multiple types with enum:
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    // For dealing with exaustive number of different types in a
    // vector, Rust provides trait objects, explored in chapter 18.
}


