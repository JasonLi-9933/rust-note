fn main() {
	let s = String::from("hello");

	// equivalent
	let slice = &s[0..2];
	let slice = &s[..2];

	let len = s.len();
	// equivalent
	let slice = &s[3..len];
	let slice = &s[3..];

	// equivalent
	let slice = &s[0..len];
	let slice = &s[..];
}

fn main2() {
	let a = [1, 2, 3, 4, 5];

	let slice = &a[1..3]; // type &[i32]

	assert_eq!(slice, &[2, 3]);
}
