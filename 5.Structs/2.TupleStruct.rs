struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
	// black and origin have different types
	let black = Color(0, 0, 0);
	let origin = Point(0, 0, 0);

	// tuple structs behave like tuples
	let black0 = black.0;
}
