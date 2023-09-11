// learning enums by categorizing different ip addresses

// define an enumeration for IP
enum IpAddrKind {
    // define enum variants with associated data
    V4(String),
    V6(String),
}

// with an enum, a function can take a single custom type that can
// instantiate as multiple different variants of the same type
//
// For example, a useful function might route an IP differently if it
// was V4 or V6; an enum can handle both use cases within one data structure:
fn route(ip_kind: IpAddrKind) {/* code goes here */}

// when used in combination with structs, enums can transform a complex set of
// possible variables into a data structure that is easily parsed and, more importantly,
// easily integrated and built upon.
//
// An Example Structure:
//
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
//
// let localhost = IpAddr {
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// };
//
// let loopback = IpAddr {
//     kind: IpAddrKind::V6,
//     address: String::from("::1"),
// };

// An enum can also contain variants of any data type: integers, tuples, strings,
// or even structs and other enums:
enum Message {
    Quit, // no data
    Move { x: i32, y: i32 }, // struct with two fields
    Write(String), // String value
    ChangeColor(i32, i32, i32), // tuple of three signed integers
}

// In addition, enums can have implementation functions:
impl Message {
    fn call(&self) {
        // method body goes here
    }
}

fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    let message = Message::Write(String::from("hello world"));
    message.call();
}
