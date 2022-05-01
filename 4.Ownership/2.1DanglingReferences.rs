fn main() {
	let reference_to_nothing = dangle();
}

fn dangle() -> &String {
	let s = String::from("hello");

	&s
} // s is removed, reference of s points to nothing!

fn no_dangle() -> String {
	let s = String::from("hello");

	s
} // ownership of s is moved out, nothing is deallocated