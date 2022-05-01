/*
Methods are similar to functions: we declare them with the fn keyword and a name,
they can have parameters and a return value, and they contain some code that’s run
when the method is called from somewhere else. Unlike functions, methods are defined within
the context of a struct (or an enum or a trait object),
and their first parameter is always self, which represents the instance of the struct the method is being called on.
*/

#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

// Everything within this impl block will be associated with the Rectangle type.
impl Rectangle {
// Methods can take ownership of self, borrow self immutably as we’ve done here,
// or borrow self mutably, just as they can any other parameter.
	fn area(&self) -> u32 { // &self === self: &Self
			self.width * self.height
	}

	fn can_hold(&self, other: &Rectangle) -> bool {
			self.width > other.width && self.height > other.height
	}

	// All functions defined within an impl block are called associated functions
	// because they’re associated with the type named after the impl
	fn square(size: u32) -> Rectangle {
			Rectangle {
					width: size,
					height: size,
			}
	}
}

fn main() {
	let rect1 = Rectangle {
			width: 30,
			height: 50,
	};

	let sq = Rectangle::square(12); // because square is doesn't need an instance of the type to work with

	println!(
			"The area of the rectangle is {} square pixels.",
			rect1.area()
	);
}
