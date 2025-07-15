fn main() {
    println!("Hello, world!");

	another_function(5);
	print_labeled_measurement(4, 'h');

	// All of the below throws errors:
	// let x = y = 0;
	// let mut x = 1;
	// let mut y = 3;
	// y = x = 5;
	// These doesn't work in rust, because of the difference
	// between statements and expressions.

	// To be short, statements doesn't resolve to any value,
	// so the result of 'y = 0' couldn't be stored in x.

	// See the docs for more details

	// Interesting example of scope block resolving to an
	// expression:
	let y = {
		let x = 3;
		x + 1 // yes, no semicolon here
	};

	println!("The value of y is: {y}.");

	// Returning values:
	let x = five();
	println!("The returned value of x is: {x}.");

	let x = plus_one(x);
	println!("The value of x plus one is: {x}.");
}

fn another_function(x: i32) {
	println!("Another function.");
	println!("The value of x is {x}.");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
	println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
	5
}

fn plus_one(x: i32) -> i32 {
	x + 1

	// The below would generate an error, since a statement
	// always return () while this plus_one function is expected
	// to return a i32 vlue.
	// x + 1;
}

