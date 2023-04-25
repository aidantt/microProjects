// Given a list of integers, use a vector and return the median and mode
// of the list. When the list is sorted numerically, the median will be
// the middle element. The mode is the most common value in the list.
//
// The order of operation, roughly:
// 1. Take a list of integers as comma separated values from input
// 2. Convert user input to a vector array
// 3. Use Vec methods to find the median and mode of the list

use std::io;

// take user input as csv and convert to Vec
fn user_input() -> Vec<i32> {
    // instantaite user input string
    let mut string_list = String::new();

    // use std::io to read user input
    // ask user for input:
    println!("Enter the list in csv format: ");
    // take input from the command line
    io::stdin()
        // if succesful, write to string_list
        .read_line(&mut string_list)
        // if fail, panic and output failure message
        // otherwise, unwrap the Result and pass forwards
        .expect("Panic! failed to read user input");

    // TEST: print output string
    println!("Your input: {}", &string_list);

    // TEST: return empty Vec
    Vec::new()
}

fn main() {
    println!("Hello, world!"); // validates runtime

    // first, use a function that takes no parameters and returns
    // the vector array
    let list = user_input();
}
