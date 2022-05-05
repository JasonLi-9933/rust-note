/*
A trait tells the Rust compiler about functionality a particular type has and 
can share with other types. We can use traits to define shared behavior in an abstract way. 
We can use trait bounds to specify that a generic type can be any type that has certain behavior

Traits are similar to a feature often called interfaces in other languages, 
although with some differences.
*/

pub trait Summary {
	fn summarize(&self) -> String;
}

pub struct NewsArticle {
	pub headline: String,
	pub location: String,
	pub author: String,
	pub content: String,
}

impl Summary for NewsArticle {
	fn summarize(&self) -> String {
		format!("{}, by {} ({})", self.headline, self.author, self.location)
	}
}

pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: bool,
	pub retweet: bool,
}

impl Summary for Tweet {
	fn summarize(&self) -> String {
		format!("{}: {}", self.username, self.content)
	}
}

/*
Now that the library has implemented the Summary trait on NewsArticle and Tweet, 
users of the crate can call the trait methods on instances of NewsArticle and Tweet 
in the same way we call regular methods. 

The only difference is that the trait has to be brought into scope 
as well as the types to get the additional trait methods.

For example: 
*/
use aggregator::{Summary, Tweet};

fn main() {
	let tweet = Tweet {
		username: String::from("horse_ebooks"),
		content: String::from(
			"of course, as you probably already know, people",
		),
		reply: false,
		retweet: false,
	};

	println!("1 new tweet: {}", tweet.summarize());
}
/*
 coherent (orphan rule):

 We can also implement Summary on Vec<T> in our crate, 
 because the trait Summary is local to our crate.

 We can’t implement external traits on external types. 
 For example, we can’t implement the Display trait on Vec<T> within our aggregator crate, 
 because Display and Vec<T> are defined in the standard library and aren’t local to our 
 aggregator crate.
*/


// ============================================================================
// ============================================================================
/* Default Implementations */

// Sometimes it’s useful to have default behavior for some or all 
// of the methods in a trait instead of requiring implementations for all methods on every type
pub trait Summary {
	fn summarize(&self) -> String {
		String::from("(Read more...)")
	}
}

/*
syntax for overriding a default implementation is the same as the syntax 
for implementing a trait method that doesn’t have a default implementation.
*/

// Default implementations can call other methods in the same trait, 
// even if those other methods don’t have a default implementation.
pub trait Summary {
	fn summarize_author(&self) -> String;

	fn summarize(&self) -> String {
		format!("(Read more from {}...)", self.summarize_author())
	}
}

impl Summary for Tweet {
	fn summarize_author(&self) -> String {
		format!("@{}", self.username)
	}
}

// ============================================================================
// ============================================================================
/* Traits as Parameters */
pub fn notify(item: &impl Summary) {
	println!("Breaking news! {}", item.summarize());
}
// Trait Bound Syntax (Equivalent to above): 
pub fn notify<T: Summary>(item: &T) {
	println!("Breaking news! {}", item.summarize());
}

// Specifying Multiple Trait Bounds with the + Syntax
pub fn notify(item: &(impl Summary + Display)) {}
pub fn notify<T: Summary + Display>(item: &T) {}

// Clearer Trait Bounds with where Clauses
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

fn some_function<T, U>(t: &T, u: &U) -> i32
	where T: Display + Clone,
				U: Clone + Debug
{}

// ============================================================================
// ============================================================================
/* Returning Types that Implement Traits */
fn returns_summarizable() -> impl Summary {}

// you can only use impl Trait if you’re returning a single type. 
// For example, this code that returns either a NewsArticle or a Tweet 
// with the return type specified as impl Summary wouldn’t work:
fn returns_summarizable(switch: bool) -> impl Summary {
	if switch {
		NewsArticle {
			headline: String::from(
				"Penguins win the Stanley Cup Championship!",
			),
			location: String::from("Pittsburgh, PA, USA"),
			author: String::from("Iceburgh"),
			content: String::from(
				"The Pittsburgh Penguins once again are the best \
					hockey team in the NHL.",
			),
		}
	} else {
		Tweet {
			username: String::from("horse_ebooks"),
			content: String::from(
				"of course, as you probably already know, people",
			),
			reply: false,
			retweet: false,
		}
	}
}

// ============================================================================
// ============================================================================
/* Using Trait Bounds to Conditionally Implement Methods */
use std::fmt::Display;

struct Pair<T> {
	x: T,
	y: T,
}

impl<T> Pair<T> {
	fn new(x: T, y: T) -> Self {
		Self { x, y }
	}
}

impl<T: Display + PartialOrd> Pair<T> {
	fn cmp_display(&self) {
		if self.x >= self.y {
			println!("The largest member is x = {}", self.x);
		} else {
			println!("The largest member is y = {}", self.y);
		}
	}
}

// We can also conditionally implement a trait for any type that implements another trait.
// Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations
impl<T: Display> ToString for T {
	// --snip--
}

// Because the standard library has this blanket implementation, 
// we can call the to_string method defined by the ToString trait on any type that implements the Display trait.
let s = 3.to_string();
