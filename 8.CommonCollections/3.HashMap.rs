fn main() {
	/* Initialization */

	use std::collections::HashMap;
	//1
	let mut scores = HashMap::new();
	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);
	//2
	// constructing a hash map is by using iterators and the collect method 
	// on a vector of tuples, where each tuple consists of a key and its value.
	let teams = vec![String::from("Blue"), String::from("Yellow")];
	let initial_scores = vec![10, 50];
	let mut scores: HashMap<_, _> =
	teams.into_iter().zip(initial_scores.into_iter()).collect();

// ============================================================================
// ============================================================================
/* Hash Maps and Ownership */

/**
 * For types that implement the Copy trait, like i32, the values are copied into the hash map. 
 * For owned values like String, the values will be moved and the hash map will be the owner of 
 * those values
 */
	let field_name = String::from("Favorite color");
	let field_value = String::from("Blue");

	let mut map = HashMap::new();
	map.insert(field_name, field_value);
	// field_name and field_value are invalid at this point

/**
 * If we insert references to values into the hash map, the values won’t be moved into the hash map. 
 * The values that the references point to must be valid for at least as long as the hash map is valid.
 */

// ============================================================================
// ============================================================================
/* Accessing Values in a Hash Map */
	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);

	let team_name = String::from("Blue");
	// result is wrapped in Some because get returns an Option<&V>
	let score = scores.get(&team_name); // sore is Some(&10)

	for (key, value) in &scores {
		println!("{}: {}", key, value);
	}

// ============================================================================
// ============================================================================
/* Updating a Hash Map */

// Overwriting a Value
	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Blue"), 25);

	println!("{:?}", scores); // 25

// Only Inserting a Value If the Key Has No Value
/**
 * entry():
 * The return value of the entry method is an enum called Entry 
 * that represents a value that might or might not exist.
 * 
 * or_insert():
 * or_insert method on Entry is defined to return a mutable reference to the value 
 * for the corresponding Entry key if that key exists, and if not, 
 * inserts the parameter as the new value for this key and returns a mutable reference to the new value.
 */
	scores.entry(String::from("Yellow")).or_insert(50);
	scores.entry(String::from("Blue")).or_insert(50);

// Updating a Value Based on the Old Value
	let text = "hello world wonderful world";

	let mut map = HashMap::new();

	for word in text.split_whitespace() {
			let count = map.entry(word).or_insert(0); // count is a mutable reference
			*count += 1;
	} // mutable reference count goes out of scope

	println!("{:?}", map);
}
