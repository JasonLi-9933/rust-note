use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess: ");

    // In Rust, variables are immutable by default. 
    // let apples = 5; // immutable
    // let mut bananas = 5; // mutable
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess) // & indicates the argument is a reference: `guess` is a mutable reference
            .expect("Failed to read line!");

        // overshadow the previous `guess` and convert it to a number
        let guess: u32 = match guess.trim().parse() // parse strings to numbers
                        {
			                // parse returns `Result` type is an enum has variants `OK` and `Err`
                            Ok(num) => num,
                            Err(_) => continue, // `_` is a catchall value, here we ignore all the errors and continue
                        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}