#![allow(dead_code)]
#![allow(unused_variables)]
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
	let x = 5;
	println!("The start value of x is {x}.");

	// This would throw an error
	// x = 6;

	// This works, and it's named "variable shadowing"
	// See the docs for more info		
	let x = x + 1;
	println!("The new value of x is {x}.");

	// Shadowing affects the variable only until the end
	// of scope in which it is defined
	{
		let x = x * 2;
		println!("The value of x in the inner scope is {x}.");
	}

	println!("The end value of x is {x}.");

	// Also, when shadowing, we can mutate the type of a variable
	let spaces = "     ";
	let spaces = spaces.len();

	// This would throw an error
	// let mut spaces = "    ";
	// spaces = spaces.len();
}
