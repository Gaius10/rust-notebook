
fn main() {
    let mut v = vec![1, 2, 3, 3, 3, 4, 5, 6, 7, 7, 8, 8, 9, 9, 1];
    v.sort();

    let Some(median) = statistics::median(&v) else {
        println!("Error getting median.");
        return;
    };

    let Some(mode) = statistics::mode(&v) else {
        println!("Error getting mode.");
        return;
    };

    println!("Dataset: {v:?}");
    println!("Dataset size: {}", v.len());
    println!("Median: {median}");
    println!("Mode: {mode}");
}
