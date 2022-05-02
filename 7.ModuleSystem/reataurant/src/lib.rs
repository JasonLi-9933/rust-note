/* 
src/main.rs and src/lib.rs are called crate roots.
The reason for their name is that the contents of either of
these two files form a module named crate at the root of 
the crate’s module structure, known as the module tree

crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment

*/

mod front_of_house {
    pub mod hosting { // inner module is private by default
        pub fn add_to_waitlist() {} // this function is private by default

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

/* 
The front_of_house module isn’t public, but because the eat_at_restaurant function 
is defined in the same module as front_of_house (that is, eat_at_restaurant and front_of_house are siblings)
*/

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}


// ============================================================================
// ============================================================================

/* Starting Relative Paths with super */

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

// ============================================================================
// ============================================================================

/* Making structs public */
mod back_of_house2 {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant2() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house2::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next two lines won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
    // println!("I'd like {} toast please", meal.seasonal_fruit);
}


// ============================================================================
// ============================================================================

/* 
Making structs public 
In contrast, if we make an enum public, all of its variants are then public. 
We only need the pub before the enum keyword
*/
mod back_of_house3 {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant3() {
    let order1 = back_of_house3::Appetizer::Soup;
    let order2 = back_of_house3::Appetizer::Salad;
}
