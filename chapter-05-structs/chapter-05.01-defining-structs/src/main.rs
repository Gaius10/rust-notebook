#![allow(unused_variables)]
#![allow(dead_code)]
// Structs are customizable variable types built by association of
// other types/structs previously existant.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// They are also tuple structs, that are basically special named
// tuples:
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Also, unit-like structs are used when we have no data attributes 
// to associate it with.
struct AlwaysEqual;

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // Rust doesn't allow us to make individual attributes mutable.
    // Either the entire instance must be mutable or not.
    let mut user2 = User {
        active: true,
        username: String::from("someusername1234"),
        email: String::from("someone2@example.com"),
        sign_in_count: 1,
    };

    user2.email = String::from("another@example.com");

    let user3 = build_user("email123@example.com", "nickname123");
    let user3_modified = User {
        active: false,
        ..user3 // data is moved, no way to use `user3` after this
    };

    let user2_modified = User {
        username: String::from("username123123"),
        email: String::from("email@example.com"),
        ..user2 // User 2 is still valid after this, as it's
                // username and email wasn't moved.
                // (active and sign_in_count implement Copy trait,
                // so they're not moved as well)
    };

    //
    // Using tuple structs without named fields
    //
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // Despite the similar structure, Color and Point are different,
    // uncompatible types.

    // Destructuring struct tuples:
    let Point(x, y, z) = origin;
    // It's required to name the struct when destructuring.

    // Unit-like structs:
    let subject = AlwaysEqual;
    // more discussed at chapter 10.

    // Also, it's possible to implement structs with references
    // as attributes, but it requires specifying lifetime, concept
    // that will be introduced later in chapter 10.
}

fn build_user(email: &str, username: &str) -> User {
    let email = String::from(email);
    let username = String::from(username);
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}







