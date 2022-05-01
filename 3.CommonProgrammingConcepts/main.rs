fn main() {
	/* Variables and Mutability */
	let x = 5; // x is immutable
	let mut y = 5; // y is mutable

	const SECONDS_IN_MINUTE = 60; // can't use `mut` as it is always immutable

	/* shadowing */
	let a = 1;
	let a = a+1;
	// a is 2;
	{
		let a = a * 2; 
		// a is 4
		let a = 'a'; // shadowing is crating a new variable so there is no restrictions on data type
	}
	// a is 2 again

	/* tuple */
	let t: (i32, f64, u8) = (500, 6.6, 1);
	let five_hundred = t.0; // accessing the first element


	/* array */
	let arr = [1,2,3];
	let arr: [i32; 5] = [1,2,3,4,5];
	let arr: [3; 5]; // [3,3,3,3,3]
	let first = arr[0];

	/* functions */
	// Syntax1
	fn function_name() -> return_type {
		//statements
		return value;
	}
	//Syntax2
	fn function_name() -> return_type {
		value //no semicolon means this value is returned
	}

	fn plus_one(x: i32) -> i32 {
    x + 1
	}

	/* control flow */
	let number = 6;

	if number % 4 == 0 {
			println!("number is divisible by 4");
	} else if number % 3 == 0 {
			println!("number is divisible by 3");
	} else if number % 2 == 0 {
			println!("number is divisible by 2");
	} else {
			println!("number is not divisible by 4, 3, or 2");
	}	

	let condition = true;
	let number = if condition { 5 } else { 6 };

	/* loop */
	loop {
		println!("Again!!");
	}

	let mut counter = 0;

	let result = loop {
			counter += 1;

			if counter == 10 {
					break counter * 2; // return this value to result
			}
	};

	let mut number = 3;

	while number != 0 {
			println!("{}!", number);

			number -= 1;
	}

	let a = [10, 20, 30, 40, 50];

	for element in a {
			println!("the value is: {}", element);
	}

	for number in (1..4).rev() {
		println!("{}!", number);
	}
}