#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Different coin for each state
}

fn main() {
    let value = value_in_cents(Coin::Penny);

    // With the Option<T>...
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //
    // Obs.: matches MUST deal with all possible cases, so
    // commenting the None => None, line would crash the program
    // at compile time.
    //
    // For that, there are catch all patterns and the _ placeholder
    //
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),

        // Another possibilities ignoring the value
        // _ => reroll()
        // _ => () // Nothing happens, resolve to unit value
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter for {state:?}.");
            25
        }
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

