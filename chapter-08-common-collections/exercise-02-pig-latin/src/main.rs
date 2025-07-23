
fn main() {
    let s1 = String::from("Word");
    let s2 = String::from("Two words");
    let s3 = String::from("A little bit more complex sentence.");

    println!("Base s1: {s1}");
    println!("Base s2: {s2}");
    println!("Base s3: {s3}");

    println!("Pig s1: {}", pig::say(&s1));
    println!("Pig s2: {}", pig::say(&s2));
    println!("Pig s3: {}", pig::say(&s3));
}

