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

    // make a vector that separates the string by comma
    let vec_string: Vec<&str> = string_list.trim()
        // split by comma
        .split(',')
        // assemble into a list of Strings
        .collect();

    // using the vector array of number characters, type cast into
    // the returned vector as i32
    let mut vec_int: Vec<i32> = Vec::new();
    for i in &vec_string {
        // parse character into integer and unwrap Result
        let integer: i32 = i.parse::<i32>().unwrap();
        // push into return vector
        vec_int.push(integer);
    }

    // TEST: print output string
    println!("Your input: {}", &string_list);

    // TEST: print split string
    println!("split string: {:?}", vec_string);

    // TEST: print integer vector
    println!("integer vector is: {:?}", vec_int);

    // Return the integer vector list
    vec_int
}

fn find_median(mut list: &Vec<i32>) -> i32 {
    // Given a list of integers, sort and find the median

    // TEST: print passed list
    println!("passed list is: {:?}", list);

    // sort the list numerically
    list.sort();

    // TEST: return dummy integer
    1
}

fn main() {
    println!("Hello, world!"); // validates runtime

    // first, use a function that takes no parameters and returns
    // the vector array
    let list: Vec<i32> = user_input();

    // pass the vec list to functions that calculate the median and mode
    let median = find_median(&list);
}
