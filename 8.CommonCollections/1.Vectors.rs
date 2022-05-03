fn main() {
	/* Initialization */
	let v: Vec<i32> = Vec::new();
	let v = vec![1, 2, 3];
	

// ============================================================================
// ============================================================================


	/* Updating a vector */
	let mut v = Vec::new();
	v.push(5);
	v.push(6);
	v.push(7);
	v.push(8);


// ============================================================================
// ============================================================================

	/* 
	When the vector gets dropped, all of its contents are also dropped, 
	meaning those integers it holds will be cleaned up. 
	*/
	{
			let v = vec![1, 2, 3, 4];

			// do stuff with v
	} // <- v goes out of scope and is freed here


// ============================================================================
// ============================================================================

	/* Reading from vectors*/
	let v = vec![1, 2, 3, 4, 5];

	// & and [] give us a reference of the element
	let third: &i32 = &v[2];
	println!("The third element is {}", third);

  // using the get method with the index passed as an argument, which gives us an Option<&T>
	match v.get(2) {
			Some(third) => println!("The third element is {}", third),
			None => println!("There is no third element."),
	}

	let does_not_exist = &v[100]; // PANIC!!!! it references a nonexistent element
	let does_not_exist = v.get(100); // returns None without panicking


// ============================================================================
// ============================================================================
	let mut v = vec![1, 2, 3, 4, 5];

	// switch the order of next two line will make the code compile
	let first = &v[0]; // immutable borrow
	v.push(6); // mutable borrow

	// change first below to &v[0] will make the code compile
	println!("The first element is: {}", first); // ERROR!!! immutable borrow used here

	/* 
	This error is due to the way vectors work: 
	because vectors put the values next to each other in memory, 
	adding a new element onto the end of the vector might require 
	allocating new memory and copying the old elements to the new space, 
	if there isn’t enough room to put all the elements next to each other 
	where the vector is currently stored. In that case, 
	the reference to the first element would be pointing to deallocated memory.
	*/


// ============================================================================
// ============================================================================

/* Iterating over the Values in a Vector */
	let v = vec![100, 32, 57];
	for i in &v {
		println!("{}", i);
	}

	let mut v = vec![100, 32, 57];
	for i in &mut v {
		*i += 50;
	}

// ============================================================================
// ============================================================================

/* 
Using an Enum to Store Multiple TypesIterating over the Values in a Vector 
All the enum variants will be considered the same type: that of the enum
Vectors can only store values that are the same type.
*/
	enum SpreadsheetCell {
		Int(i32),
		Float(f64),
		Text(String),
	}

	let row = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Text(String::from("blue")),
		SpreadsheetCell::Float(10.12),
	];
}
