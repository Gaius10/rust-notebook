use std::io;
use std::io::Write;

fn main() {
	let mut input = String::new();

	print!("Input the temperature in celsius: ");

	io::stdout()
		.flush()
		.expect("Error printing prompt message.");

	io::stdin()
		.read_line(&mut input)
		.expect("Error reading input temperature.");

	let input: f64 = input.trim()
						  .parse()
						  .expect("Error parsing input.");

	let output: f64 = input * (9.0/5.0) + 32.0;

	println!("Temperature in Fahrenheit: {output}ºF");
}
