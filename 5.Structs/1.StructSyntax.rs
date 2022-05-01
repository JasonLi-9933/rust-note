struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

/*
we used the owned String type rather than the &str string slice type.
This is a deliberate choice because we want each instance of this struct
to own all of its data and for that data to be valid for as long as the entire struct is valid.
*/

fn main() {
	let user1 = User {
			email: String::from("someone@example.com"),
			username: String::from("someusername123"),
			active: true,
			sign_in_count: 1,
	};
	// Rust doesn’t allow us to mark only certain fields as mutable.
	let mut user2 = User {
		email: String::from("someone@example.com"),
		username: String::from("someusername123"),
		active: true,
		sign_in_count: 1,
	};

	user2.email = String::from("anotheremail@example.com");
}


// ============================================================================
// ============================================================================

fn build_user(email: String, username: String) -> User {
	User {
			email: email,
			username: username,
			active: true,
			sign_in_count: 1,
	}
}

fn build_user2(email: String, username: String) -> User {
	User {
			email, // shorthand for email: email
			username, // shorthand for username: username
			active: true,
			sign_in_count: 1,
	}
}


// ============================================================================
// ============================================================================

/*
 In this example, we can no longer use user1 after creating user2
 because the String in the username field of user1 was moved into user2.
 If we had given user2 new String values for both email and username, 
 and thus only used the active and sign_in_count values from user1,
 then user1 would still be valid after creating user2.

 The types of active and sign_in_count are types that implement the Copy trait
 */

fn main() {
	let user1 = User {
			email: String::from("someone@example.com"),
			username: String::from("someusername123"),
			active: true,
			sign_in_count: 1,
	};
	// Creating Instances From Other Instances With Struct Update Syntax
	let mut user2 = User {
		email: String::from("someone@example.com"),
		..user1
	};
 // user1 got dropped here as user2 uses username of user1
}




