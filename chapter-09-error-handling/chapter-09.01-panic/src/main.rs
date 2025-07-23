
fn main() {
    //
    // Important detail: panic! macro walks back through the stack
    // cleaning all resources our app is using, then finally exits.
    // If we need to decrease as much as possible our binary file's
    // size, it's possible to change this behaviour in Cargo.toml
    // by adding:
    //
    // [profile.release]
    // panic = 'abort'
    //
    // This will cause panic! to immediately abort the execution,
    // without cleaning anything.
    //
    // panic!("Hello, world!"); // Just an example.

    // You can use RUST_BACKTRACE env var to locate errors:
    let v = vec![1, 2, 3];
    v[99];
}

