use std::io::stdin;

fn main() {
	fn add(a: i32, b: i32) -> i32 {
		a + b
	}
	fn sub(a: i32, b: i32) -> i32 {
		a - b
	}
	fn mul(a: i32, b: i32) -> i32 {
		a * b
	}
	fn div(a: i32, b: i32) -> i32 {
		a / b
	}

	let mut value: i32;

	println!("Welcome to the calculator!");
	println!("Please enter a number:");
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();
	value = input.trim().parse().unwrap();

	loop {
		println!("Please enter an operator:");
		let mut input = String::new();
		stdin().read_line(&mut input).unwrap();
		let operator = input.trim();

		println!("Please enter a number:");
		let mut input = String::new();
		stdin().read_line(&mut input).unwrap();
		let number = input.trim().parse().unwrap();

		match operator {
			"+" => value = add(value, number),
			"-" => value = sub(value, number),
			"*" => value = mul(value, number),
			"/" => value = div(value, number),
			_ => {
				println!("Invalid operator!");
				continue;
			}
		}

		println!("Result: {}", value);

		let mut input = String::new();
		println!("Press enter to continue or type 'q' to quit.");
		stdin().read_line(&mut input).unwrap();
		if input.trim() == "q" {
			break;
		}
	}
}
