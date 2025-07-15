use std::io;
use std::io::Write;

fn main() {
	let mut input = String::new();

	print!("Insert the fibonacci index you wanna know: ");

	io::stdout()
		.flush()
		.expect("Error prompting.");

	io::stdin()
		.read_line(&mut input)
		.expect("Error reading input.");

	let input: i64 = input.trim()
						   .parse()
						   .expect("Error parsing input.");

	let output = fib(input);
	println!("Fibonacci's {input} nth number: {output}.");
}

fn fib(nth: i64) -> i128 {
	if nth == 1 { return 1; }
	if nth == 0 { return 0; }

	fib(nth - 1) + fib(nth - 2)
}







