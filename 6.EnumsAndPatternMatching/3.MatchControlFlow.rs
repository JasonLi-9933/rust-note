enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
	match coin {
			Coin::Penny => {
					println!("Lucky penny!");
					1
			}
			Coin::Nickel => 5,
			Coin::Dime => 10,
			Coin::Quarter => 25,
	}
}


// ============================================================================
// ============================================================================

/* Patterns that Bind to Values */

enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}


// ============================================================================
// ============================================================================

/* Matching with Option<T> */

fn main2() {
	fn plus_one(x: Option<i32>) -> Option<i32> {
			match x {
					None => None,
					Some(i) => Some(i + 1),
			}
	}

	let five = Some(5);
	let six = plus_one(five);
	let none = plus_one(None);
}

// ============================================================================
// ============================================================================

/* Matches are Exhaustive */
fn main3() {
	fn plus_one(x: Option<i32>) -> Option<i32> {
			match x {
					Some(i) => Some(i + 1),
					// ERROR!!! pattern `None` not covered
			}
	}

	let five = Some(5);
	let six = plus_one(five);
	let none = plus_one(None);
}

// ============================================================================
// ============================================================================

/* Catch-all Patterns and the _ Placeholder */

fn main4() {
	let dice_roll = 9;
	match dice_roll {
			3 => add_fancy_hat(),
			7 => remove_fancy_hat(),
			other => move_player(other),
	}

	fn add_fancy_hat() {}
	fn remove_fancy_hat() {}
	fn move_player(num_spaces: u8) {}
}

fn main5() {
	let dice_roll = 9;
	match dice_roll {
			3 => add_fancy_hat(),
			7 => remove_fancy_hat(),
			_ => reroll(), // catch-all pattern: _, which is a special pattern that matches any value and does not bind to that value
	}

	fn add_fancy_hat() {}
	fn remove_fancy_hat() {}
	fn reroll() {}
}

fn main() {
	let dice_roll = 9;
	match dice_roll {
			3 => add_fancy_hat(),
			7 => remove_fancy_hat(),
			_ => (), // we don’t want to run any code in this case
	}

	fn add_fancy_hat() {}
	fn remove_fancy_hat() {}
}



