// Practice with structs by calculating rectangle area

// Define a struct Rectangle with two properties
// Use derive to give struct debug properties
#[derive(Debug)]
struct Rectangle {
    width: u32, // u32 for unsigned integers
    height: u32,
}

// instead of an area function that takes an instance of Rectangle
// as a parameter, define a method linked to the Rectangle struct type
// that returns the area of a Rectangle instance given only itself.
impl Rectangle { // impl for "implementation"
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // add a can_hold() method that does a binary check if parent Rectangle
    // can hold another, i.e. rect1.area() > rect2.area() -> true
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width) && (self.height > other.height)
    }

    // all functions defined inside the implementation block are associated functions,
    // but not all associated functions have to be methods. If a impl function does not
    // reference &self, then the function should not be considered a method, but can still
    // provide functionality to the parent object, in this case the Rectangle struct.
    //
    // Here, define a square function that creates a new instance of Rectangle with the same
    // width and height dimensions. If some associated function instantiates a new instance,
    // it is usually called a "constructor".
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// Define area function, takes a reference to Rectangle struct and returns
// area as an integer
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

fn main() {
    // instantiate variable of type Rectangle
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // construct a Rectangle object with the square function of dimensions 5x5
    // use '::' to construct a Rectangle value with the square function
    let square1 = Rectangle::square(5);

    println!(
        "The area of the first rectangle is {} square pixels.",
        rect1.area() // method call syntax on an instantiated struct object
        // area(&rect1) // area() borrows rect1, main retains ownership
    );

    // if you want to find if one rectangle can hold another, define another
    // method implemented for Rectangle, can_hold(&Rectangle):
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // use :? to print a struct in debug mode
    println!("Look ma, a square! {:?}", &square1);
    println!("Seems like the area is {}", square1.area());
}
