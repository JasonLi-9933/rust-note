fn main() {
	enum IpAddrKind {
			V4,
			V6,
	}

	struct IpAddr {
			kind: IpAddrKind,
			address: String,
	}

	let home = IpAddr {
			kind: IpAddrKind::V4,
			address: String::from("127.0.0.1"),
	};

	let loopback = IpAddr {
			kind: IpAddrKind::V6,
			address: String::from("::1"),
	};
}

// ============================================================================
// ============================================================================

fn main2() {
	// This new definition of the IpAddr enum says that both V4 and V6 variants will have associated String values
	enum IpAddr {
		V4(String),
		V6(String),
	}
	// name of each enum variant that we define also becomes a function that constructs an instance of the enum
	let home = IpAddr::V4(String::from("127.0.0.1"));
	let loopback = IpAddr::V6(String::from("::1"));
}

// ============================================================================
// ============================================================================

/*
Code below illustrates that you can put any kind of data inside an enum variant:
strings, numeric types, or structs, for example. You can even include another enum!
*/
fn main3() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}

fn main4() {
	struct Ipv4Addr {
		// --snip--
	}

	struct Ipv6Addr {
		// --snip--
	}

	enum IpAddr {
		V4(Ipv4Addr),
		V6(Ipv6Addr),
	}
}

// ============================================================================
// ============================================================================
fn main5() {
	enum Message {
		Quit,
		Move { x: i32, y: i32 },
		Write(String),
		ChangeColor(i32, i32, i32),
	}

	// The following structs could hold the same data that the preceding enum variants hold:
	struct QuitMessage; // unit struct
	struct MoveMessage {
		x: i32,
		y: i32,
	}
	struct WriteMessage(String); // tuple struct
	struct ChangeColorMessage(i32, i32, i32); // tuple struct

	/* 
	we couldn’t as easily define a function to take any of these kinds of messages
	as we could with the Message enum defined, which is a single type
	*/

	/*
	There is one more similarity between enums and structs:
	just as we’re able to define methods on structs using impl, 
	we’re also able to define methods on enums.
	*/
	impl Message {
			fn call(&self) {
					// method body would be defined here
			}
	}

	let m = Message::Write(String::from("hello"));
	m.call();
}