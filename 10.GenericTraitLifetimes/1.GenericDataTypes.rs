/* In Function Definition */

/*
If we don’t want to restrict the largest function to the types that implement the Copy trait, 
we could specify that T has the trait bound Clone instead of Copy. 
Then we could clone each value in the slice when we want the largest function to have ownership. 
Using the clone function means we’re potentially making more heap allocations in the case of types 
that own heap data like String, and heap allocations can be slow if we’re working with large amounts of data.
*/
fn largest<T>(list: &[T]) -> T { // correct way: fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
	let mut largest = list[0];

	for &item in list {
		if item > largest { // BIG PROBLEM!!!!
			largest = item;
		}
	}

	largest
}

fn main() {
	/* THIS WON'T COMPILE!!!!  as T above may not be "comparable" */
	let number_list = vec![34, 50, 25, 100, 65];
	let result = largest(&number_list);
	println!("The largest number is {}", result);

	let char_list = vec!['y', 'm', 'a', 'q'];
	let result = largest(&char_list);
	println!("The largest char is {}", result);
}

// ============================================================================
// ============================================================================
/* In Struct Definition */
struct Point<T> {
	x: T,
	y: T,
}

struct Point2<T, U> {
	x: T,
	y: U,
}

fn main() {
	let integer = Point { x: 5, y: 10 };
	let float = Point { x: 1.0, y: 4.0 };
	let wont_work = Point { x: 5, y: 4.0 }; // WON'T WORK!! x and y must have the same type

	let both_integer = Point2 { x: 5, y: 10 }; // No Problem
	let both_float = Point2 { x: 1.0, y: 4.0 }; // No Problem
	let integer_and_float = Point2 { x: 5, y: 4.0 }; // No Problem
}

// ============================================================================
// ============================================================================
/* In Enum Definition */
enum Option<T> {
	Some(T),
	None,
}

enum Result<T, E> {
	Ok(T),
	Err(E),
}

// ============================================================================
// ============================================================================
/* In Method Definition */
struct Point<T> {
	x: T,
	y: T,
}
// Note here we have to declare T just after impl so we can use it 
// to specify that we’re implementing methods on the type Point<T>
impl<T> Point<T> {
	fn x(&self) -> &T {
		&self.x
	}
}
// This code means the type Point<f32> will have a method named distance_from_origin 
// and other instances of Point<T> where T is not of type f32 will not have this method defined.
impl Point<f32> {
	fn distance_from_origin(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2)).sqrt()
	}
}

fn main() {
	let p = Point { x: 5, y: 10 };
	println!("p.x = {}", p.x());
}

// ============================================================================
// ============================================================================
struct Point<X1, Y1> {
	x: X1,
	y: Y1,
}
// Here, the generic parameters X1 and Y1 are declared after impl because they go with the struct definition
impl<X1, Y1> Point<X1, Y1> {
	// The generic parameters X2 and Y2 are declared after fn mixup, because they’re only relevant to the method.
	fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
		Point {
			x: self.x,
			y: other.y,
		}
	}
}

fn main() {
	let p1 = Point { x: 5, y: 10.4 };
	let p2 = Point { x: "Hello", y: 'c' };

	let p3 = p1.mixup(p2);

	println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}