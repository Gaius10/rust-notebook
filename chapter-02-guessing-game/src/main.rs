use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
	let secret_number = rand::rng().random_range(1..=100);

	println!("Guess the number!");
	println!("The secret number is {secret_number}");
	println!("Please input your guess.");

	loop {
		let mut guess = String::new();

		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			},
		}

		println!("The secret number isn't {guess}, please try again.");
	}

	println!("Congratulations! You guessed the number!");
}

