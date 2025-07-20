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
    Quarter(UsState),
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // --snip--
        }
    }
}

fn main() {
    // In situations like this, when we want to do something in only
    // one case and ignore the rest, it can be annoying to add
    // the _ => () boilerplate every time:
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is {max}."),
        _ => (),
    }

    // So Rust gives us the if let syntax:
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}.");
    }

    // Back to previous chapter's example:
    let coin = Coin::Quarter(UsState::Alaska);
    let description = describe_state_quarter(coin);

    if let Some(text) = description {
        println!("{text}");
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    // Chaotic version of code: (works)
    /*if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }*/

    // Slightly better option:
    /*let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }*/

    // Much better (fail fast pattern):
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}



