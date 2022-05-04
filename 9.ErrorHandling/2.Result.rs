use std::fs::File;

/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

fn main() {
	let f = File::open("hello.txt"); // returns a Result

	let f = match f {
		Ok(file) => file,
		Err(error) => panic!("Problem opening the file: {:?}", error),
	};
// ============================================================================
// ============================================================================
/* Matching on Different Errors */
	let f = match f {
		Ok(file) => file,
		Err(error) => match error.kind() {
			ErrorKind::NotFound => match File::create("hello.txt") {
				Ok(fc) => fc,
				Err(e) => panic!("Problem creating the file: {:?}", e),
			},
			other_error => {
				panic!("Problem opening the file: {:?}", other_error)
			}
		},
	};

// ============================================================================
// ============================================================================
/* Shortcuts for Panic on Error: unwrap and expect */	

 // unwrap():
 // If the Result value is the Ok variant, unwrap will return the value inside the Ok. 
 // If the Result is the Err variant, unwrap will call the panic! macro for us.
 let f = File::open("hello.txt").unwrap();

 // expect():
 // We use expect in the same way as unwrap: to return the file handle or call the panic! macro.
 // Using expect instead of unwrap and providing good error messages 
 // can convey your intent and make tracking down the source of a panic easier.
 let f = File::open("hello.txt").expect("Failed to open hello.txt");

// ============================================================================
// ============================================================================
/* Propagating Errors */
	use std::io::{self, Read};

	fn read_username_from_file() -> Result<String, io::Error> {
		let f = File::open("hello.txt");

		let mut f = match f {
			Ok(file) => file,
			Err(e) => return Err(e), // return the error
		};

		let mut s = String::new();

		match f.read_to_string(&mut s) {
			Ok(_) => Ok(s),
			Err(e) => Err(e),
		} // return the String or the error
	}
	// |
  // V
	fn read_username_from_file2() -> Result<String, io::Error> {
		/**
		 * the ? at the end of the File::open call will return the value inside an Ok 
		 * to the variable f. If an error occurs, the ? operator will return early out of 
		 * the whole function and give any Err value to the calling code
		 */
		let mut f = File::open("hello.txt")?;
		let mut s = String::new();
		f.read_to_string(&mut s)?;
		Ok(s)
	}
	// |
  // V
	fn read_username_from_file3() -> Result<String, io::Error> {
		let mut s = String::new();
		File::open("hello.txt")?.read_to_string(&mut s)?;
		Ok(s)
	}
	/*
	There is a difference between what the match expression:
	what the ? operator does: error values that have the ? operator called on them 
	go through the from function, defined in the From trait in the standard library, 
	which is used to convert errors from one type into another.

	When the ? operator calls the from function, the error type received is 
	converted into the error type defined in the return type of the current function. 

	This is useful when a function returns one error type to represent all the ways a 
	function might fail, even if parts might fail for many different reasons. As long as 
	there’s an impl From<OtherError> for ReturnedError to define the conversion in the 
	trait’s from function, the ? operator takes care of calling the from function automatically
	*/
	use std::fs;
	fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
	}

// ============================================================================
// ============================================================================
/* Where ? Operator can be used */

/*
The ? operator can only be used in functions whose return type is compatible with the value the ? is used on. 
This is because the ? operator is defined to perform an early return of a value out of the function, 
in the same manner as the match expression.

we’re only allowed to use the ? operator in a function that returns 
Result, Option, or another type that implements FromResidual
*/
	fn last_char_of_first_line(text: &str) -> Option<char> {
		text.lines().next()?.chars().last()
	}

	/*main can also return a Result<(), E>.*/
	use std::error::Error;
	fn main() -> Result<(), Box<dyn Error>> {
		let f = File::open("hello.txt")?;
		Ok(())
	}
}