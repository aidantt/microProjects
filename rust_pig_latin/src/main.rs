// Convert strings to pig latin.
//
// Following pig latin syntax, take user input as an english string
// and convert to pig latin.
//
// The first consonant of each word is moved to the end of the word,
// and "-ay" is added; "first" becomes "irstfay".
//
// Words that start with a vowel have "-hay" added to the end.
// "apple" becomes "applehay".
//
// A rough operation of the program:
// 1. use std::io to get user input as a string
// 2. Pass string to conversion function
// 3. convert each word based on pig latin syntax
// 4. print translated string to console

use std::io; // for user input

fn user_input() -> String {
    // when called, prompt user for input and return String input

    let mut input = String::new();
    print!("Please enter the string to be translated: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Fatal: user-entered string was invalid");

    // return user input
    input
}

fn main() {
    println!("Hello, world!");

    // write user input into string
    let user_string = user_input();

    // TEST: print user input
    println!("user input: {user_string}");
}
