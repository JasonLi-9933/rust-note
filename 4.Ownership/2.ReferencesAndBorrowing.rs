fn main() {
	let s1 = String::from("hello");

	let len = calculate_length(&s1); // borrowing

	println!("The length of '{}' is {}.", s1, len);
}

/* we take &String rather than String. These ampersands represent references,
and they allow you to refer to some value without taking ownership of it*/
fn calculate_length(s: &String) -> usize { // s is a reference to a String
	s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, nothing happens.


// ============================================================================
// ============================================================================

/* We are NOT allowed to modified the borrowed value */
fn main2() {
	let s = String::from("hello");

	change(&s);
	change2(&mut s);
}

fn change(some_string: &String) {
	// wont compile
  //some_string.push_str(", world");
}
// &mut create a mutable reference
fn change2(some_string: &mut String) {
  some_string.push_str(", world");
}

// ============================================================================
// ============================================================================

/*Mutable references have one big restriction: you can have only one mutable 
	reference to a particular piece of data at a time.
  This code that attempts to create two mutable references to s will fail: */
fn main3() {
	let mut s = String::from("hello");

	let r1 = &mut s;
	let r2 = &mut s;

	println!("{}, {}", r1, r2);
}

fn main4() {
	let mut s = String::from("hello");

	{
			let r1 = &mut s;
	} // r1 goes out of scope here, so we can make a new reference with no problems.

	let r2 = &mut s;
}

// ============================================================================
// ============================================================================

fn main5() {
	let mut s = String::from("hello");

	let r1 = &s; // no problem
	let r2 = &s; // no problem
	let r3 = &mut s; // BIG PROBLEM

	println!("{}, {}, and {}", r1, r2, r3);
}

// ============================================================================
// ============================================================================

/* 
a reference’s scope starts from where it is introduced and continues through the last time that reference is used.
For instance, this code will compile because the last usage of the immutable references, the println!
*/
fn main6() {
	let mut s = String::from("hello");

	let r1 = &s; // no problem
	let r2 = &s; // no problem
	println!("{} and {}", r1, r2);
	// variables r1 and r2 will not be used after this point

	let r3 = &mut s; // no problem
	println!("{}", r3);
}
