// use enums and the "match" control structure to handle
// complex control flow

// add US states enum for additional variants of data
#[derive(Debug)] // to print enum name
enum UsState {
    Alabama,
    Alaska,
    // etc...
}

// define an enum for coins:
enum Coin {
    Penny,
    Nickel,
    Dime,
    // only quarters vary in design by state
    Quarter(UsState),
}

// find the value in individual cents for any coin:
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!"); // execute statement inside match flow
            1 // return coin value to match
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // take Quarter variant data value and print
            println!("State quarter from {:?}", state);
            25 // return coin value to match
        }
    }
}

// depending on the variant of the Coin type parameter, the function
// returns different integer values through the match case structure.

fn main() {
    let money = value_in_cents(Coin::Penny);
    println!("That's worth {} cents.", money);

    let nick = value_in_cents(Coin::Nickel);
    println!("That's worth {} cents.", nick);

    let lucky = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("That's worth {} cents.", lucky)
}
