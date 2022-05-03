fn main() {
/* Initialization */
	// 1
	let mut s = String::new();

	// 2
	let data = "initial contents";
	let s = data.to_string();

	// 3
	// the method also works on a literal directly:
	let s = "initial contents".to_string();

// ============================================================================
// ============================================================================
/* Updating */
	let mut s1 = String::from("foo");
	let s2 = "bar";
	s1.push_str(s2); // push_str takes string slices(&str) as argument cuz we don't want take ownership of the argument
	println!("s2 is {}", s2);

	let mut s = String::from("lo");
	s.push('l'); // push takes a char as argument

// ============================================================================
// ============================================================================
/* Concatenation with the + Operator or the format! Macro */
	let s1 = String::from("Hello, ");
	let s2 = String::from("world!");
	let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used, for details check the book

	let s1 = String::from("tic");
	let s2 = String::from("tac");
	let s3 = String::from("toe");

	let s = s1 + "-" + &s2 + "-" + &s3;
	let s = format!("{}-{}-{}", s1, s2, s3);

// ============================================================================
// ============================================================================
/* Indexing */
	let s1 = String::from("hello");
	let h = s1[0]; // ERROR! Will not compile

	// A String is a wrapper over a Vec<u8>
	let hello = String::from("Hola"); // length 4
	let hello = String::from("Здравствуйте"); // length 24!!! as each character in that string takes 2 bytes
	let answer = &hello[0]; // ERROR, answer is not valid and does not represent 3
	let s = &hello[0..4]; // Rust would panic at runtime if we were to try to slice only part of a character’s bytes with something like &hello[0..1]

// ============================================================================
// ============================================================================
/* Methods for Iterating Over Strings */
	for c in "नमस्ते".chars() {
		println!("{}", c);
	}

	for b in "नमस्ते".bytes() {
		println!("{}", b); // returns raw data!!
	}
}

