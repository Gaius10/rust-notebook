fn main() {
	if_expressions();
	loops();
}

fn if_expressions() {
	let number = 3;

	if number < 5 {
		println!("Number is lesser than five.");
	} else {
		println!("NUmber is greater than five.");
	}

	/*if number {
		println!("This will throw an error.");
		println!("if expressions must evaluate to a bool.");
	}*/

	if number != 0 {
		println!("Number is different of 0");
	}

	if number % 4 == 0 {
		println!("Number is divisible by 4.");
	} else if number % 3 == 0 {
		println!("Number is divisible by 3.");
	} else if number % 2 == 0 {
		println!("Number is divisible by 2.");
	} else {
		println!("Number is not divisible by 4, 3 or 2.");
	}

	// if is an expression, so you can do this:
	let condition = true;
	let number = if condition { 5 } else { 6 };

	println!("The value of number is: {number}.");

	// This would throw an error because both sides of th if
	// expression must be the same type
	// let number = if condition { 5 } else { "six" };
}

fn loops() {
	let mut counter = 0;
	let result = loop {
		counter += 1;
		if counter == 10 {
			break counter * 2;
		}
	};

	println!("The result is {result}.");

	// Here is a uglier way to do loops:
	let mut count = 0;
	'counting_up: loop {
		println!("count = {count}");
		let mut remaining = 10;

		loop {
			println!("remaining = {remaining}");
			if remaining == 9 {
				break;
			}

			if count == 2 {
				break 'counting_up;
			}

			remaining -= 1;
		}

		count += 1;
	}

	println!("End count = {count}");

	// While loops:
	let mut number = 3;
	while number != 0 {
		println!("{number}!");
		number -= 1;
	}
	println!("LIFTOFF!!!");

	let a = [10, 20, 30, 40, 50];
	let mut index = 0;

	while index < 5 {
		println!("The value is: {}", a[index]);
		index += 1;
	}

	// The above example could lead to panic caused by
	// undefined indexes. Below, a better implementation using
	// for expressions:

	for element in a {
		println!("The value is: {element} (for loop)");
	}

	// You can do almost everything with for, it's the most
	// adopted loop. Example with ranges:
	for number in (1..4).rev() {
		println!("{number}! (for loop)");
	}
	println!("LIFTOFF!!! (for loop ended)");
}




















