// use enums and the "match" control structure to handle
// complex control flow

// define an enum for coins:
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// find the value in individual cents for any coin:
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// depending on the variant of the Coin type parameter, the function
// returns different integer values through the match case structure.

fn main() {
    println!("Hello, world!");
}
