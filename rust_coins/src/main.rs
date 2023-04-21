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

// If we want to match case with Option<T>, it can do that too
//
// build a function that takes an input Option<i32>, checks if there is
// a value inside, then adds 1 to that value.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

// match does not have to be inside a function either.
// based on the value of some variable, execute different functions:

fn add_fancy_hat() {println!("Fancy Hat!")}
fn remove_fancy_hat() {println!("No hat for you!")}
fn move_player(num_spaces: u8) {
    println!("You move {} spaces!", num_spaces);
}

fn main() {
    let money = value_in_cents(Coin::Penny);
    println!("That's worth {} cents.", money);

    let nick = value_in_cents(Coin::Nickel);
    println!("That's worth {} cents.", nick);

    let lucky = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("That's worth {} cents.", lucky);

    // Syntax Option<i32>, where Option is Some & i32 is 5:
    let five = Some(5);
    println!("This should be {:?}", plus_one(five)); // should return Some(6)

    // match does not have to be inside a function either.
    // based on the value of some variable, execute different functions:

    fn add_fancy_hat() {println!("Fancy Hat!")}
    fn remove_fancy_hat() {println!("No hat for you!")}
    fn move_player(num_spaces: u8) {
        println!("You move {} spaces!", num_spaces);
    }

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(), // if rolls a 3, give a fancy hat
        7 => remove_fancy_hat(), // if rolls a 7, take away the hat
        other => move_player(other), // for all other cases, move player
    }

    // the other case acts as a catch-all that handles all non-explicit cases
}
