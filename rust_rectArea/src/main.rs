// Practice with structs by calculating rectangle area

// Define a struct Rectangle with two properties
// Use derive to give struct debug properties
#[derive(Debug)]
struct Rectangle {
    width: u32, // u32 for unsigned integers
    height: u32,
}

// Define area function, takes a reference to Rectangle struct and returns
// area as an integer
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    // instantiate variable of type Rectangle
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1) // area() borrows rect1, main retains ownership
    );
}
