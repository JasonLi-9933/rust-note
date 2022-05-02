/*
As such, Rust does not have nulls, but it does have an enum that can 
encode the concept of a value being present or absent. 
This enum is Option<T>, and it is defined by the standard library as follows:
*/

enum Option<T> {
	None,
	Some(T),
}

// The Option<T> enum is so useful that it’s even included in the prelude; you don’t need to bring it into scope explicitly

/*
you can use Some and None directly without the Option:: prefix. 
The Option<T> enum is still just a regular enum, and Some(T) and None are still variants of type Option<T>.
*/

fn main() {
	let some_number = Some(5); // Option<i32>
	let some_string = Some("a string"); // Option<&str>

	// the compiler can’t infer the type that the corresponding Some variant will hold by looking only at a None value.
	// Rust requires us to annotate the overall Option type
	let absent_number: Option<i32> = None;

	let x: i8 = 5;
	let y: Option<i8> = Some(5);

	let sum = x + y; // ERROR!!!! x and y have different types
	let sum = x + y.unwrap();
}
