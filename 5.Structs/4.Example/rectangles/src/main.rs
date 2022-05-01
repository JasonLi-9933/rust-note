#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    /* 
    Putting the specifier `:?` inside the curly brackets tells `println!` we want to use an output format called Debug.
    The Debug trait enables us to print our struct in a way that is useful for developers
    so we can see its value while we’re debugging our code.
    */

    // println!("rect1 is {:?}", rect1);
    // OR
    dbg!(rect1);
}

fn debug_example() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
