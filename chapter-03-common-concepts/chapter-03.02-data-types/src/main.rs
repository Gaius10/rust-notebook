#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_doc_comments)]

fn main() {
	/* *********************
	 * Scalar types
	 */

	// Throws an error, guess must be type annotated in cases
	// where type inference is not possible.
	// let guess = "42".parse().expect("Not a number");

	let guess: u32 = "42".parse().expect("Not a number");

	/* Integer types */
	let unsigned_128bits_integer: u128 = 2139;
	let single_byte_integer: i8 = 123;
	let any_big_number = 1_000; // same as 1000
	let forced_type_inference = 19u8; // will be an u8 integer

	/* Float types */
	// Obs.: floats are always signed
	let default_float_type = 3.0; // f64
	let float_with_32bits: f32 = 3.0; // f32

	/** Numeric operations */
	let sum = 5 + 10;
	let difference = 95.5 - 4.3;
	let product = 4 * 30;

	let quotient = 56.7 / 32.2;
	let truncated = -5 / 3; // Results in -1

	let remainder = 43 % 5;

	/* Boolean type */
	let t = true;
	let f: bool = false;

	/* Character type */
	// Rust chars are 4 bytes sized
	// They represents a Unicode Scalar Value (not ASCII)
	let c = 'z';
	let z: char = 'ℤ';
	let heart_eyed_cat = '😻';

	/* *********************
	 * Compound types
	 */

	/* Tuples */
	// The size of a tuple is not mutable
	let tup: (i32, f64, u8) = (500, 6.4, 1);
	let (x, y, z) = tup;
	println!("Value of y is: {y}.");

	// Accessing tuple elements
	let five_hundred = tup.0;
	let six_point_four = tup.1;
	let one = tup.2;

	// The unit type (empty tuple)
	let unit: () = ();

	// Tests about tuple immutability

	// This throws an error (not mutable)
	// tup.0 = 430;
	// println!("{:?}", tup);

	/* Arrays */
	// They are allocated on stack, as previous types

	// They also have fixed size

	// Useful when you know that the variable will always
	// have the same number of elements.

	let arr = [1, 2, 3, 4, 5];
	let arr: [i32; 5] = [1, 2, 3, 4, 5];
	let arr = [3; 4];
	println!("{:?}", arr);

	let first = a[0];
	let second = a[1];

	// It is possible to have 'array index out of bounds' errors
	// not caught at compile time in cases like:
 	// arr[<something-from-user-input>]

	// But rust always checks for this error at runtime, avoiding
	// security issues
}

