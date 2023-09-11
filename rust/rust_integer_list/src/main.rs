// Given a list of integers, use a vector and return the median and mode
// of the list. When the list is sorted numerically, the median will be
// the middle element. The mode is the most common value in the list.
//
// The order of operation, roughly:
// 1. Take a list of integers as comma separated values from input
// 2. Convert user input to a vector array
// 3. Use Vec methods to find the median and mode of the list

use std::io; // for user input
use std::collections::HashMap; // for mode calculation

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

    // // TEST: print output string
    // println!("Your input: {}", &string_list);
    //
    // // TEST: print split string
    // println!("split string: {:?}", vec_string);
    //
    // // TEST: print integer vector
    // println!("integer vector is: {:?}", vec_int);

    // Return the integer vector list
    vec_int
}

fn find_average(list: &Vec<i32>) -> f32 {
    // given a list, find the average
    
    // find the sum of the list, cast into float
    let sum: f32 = list.iter().sum::<i32>() as f32;

    // // TEST: print sum
    // println!("find_average sum: {sum}");

    // divide sum by the length of the list and return average
    let average = sum / list.len() as f32;

    // return average
    average
}

fn find_median(list: &mut Vec<i32>) -> f32 {
    // Given a list of integers, sort and find the median

    // // TEST: print passed list
    // println!("passed list is: {:?}", list);

    // sort the list numerically
    list.sort();

    // The median has two cases: either the list has an odd number
    // of elements, and the middle index can be returned, or the list
    // has an even number of elements, and the mean of the two middle
    // elements must be returned.
    //
    // use an if else statement to handle binary control flow
    //
    // if list length is even by modulo, follow even case
    if (list.len() % 2) == 0 {
        // define a left and right index in the middle
        let left = list.len()/2 - 1;
        let right = list.len()/2;
        // return the mean of the middle two elements
        (list[left] + list[right]) as f32 / 2.0
    } else {
        // list has odd number of elements, return middle element
        list[list.len()/2] as f32
    }
}

fn find_mode(list: &mut Vec<i32>) -> i32 {
    // given a vector list, find the mode and return its value
    // the mode is the most common number in the list

    // in order to number elements by occurrence, use a HashMap
    let mut occurrences = HashMap::new();

    // use a referenced element in the list to enter the list elements
    // into the hashmap by occurrence
    for &mut value in list {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    // Now, find the most common element and return this value
    occurrences
        .into_iter() // convert hashmap to iterable list of tuples
        .max_by_key(|&(_, count)| count) // find most common value
        .map(|(val, _)| val) // unwrap most common element from hash tuple
        .expect("Fatal: cannot compute mode of zero numbers")
}

fn main() {
    // first, use a function that takes no parameters and returns
    // the vector array
    let mut list: Vec<i32> = user_input();

    // find the sum and average
    let sum: f32 = list.iter().sum::<i32>() as f32;
    let average = find_average(&list);

    // pass the vec list to functions that calculate the median and mode
    let median = find_median(&mut list);
    let mode = find_mode(&mut list);

    // print all desired output
    println!("
    Sorted List: {:?}
    Sum: {}
    Average: {}
    Median: {}
    Mode: {}
    ", &list, sum, average, median, mode);
}
