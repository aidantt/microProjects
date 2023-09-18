// 1-11: Developing first programs

/* This lesson mainly touches on all of the syntax and terminology covered
by the preceding lessons. As opposed to just simple examples, we begin to
create programs that have a use case, and could be considered the most simple
programs with real purpose. */

// Let's start with something easy: take an input from console, double it, and
// output the evaluated expression.

#include <iostream>

int main()
{
    // prompt user input
    std::cout << "Enter an integer: ";

    // define variable to store user input
    int userInput{ }; // zero-initialize
    std::cin >> userInput; // store console input in userInput

    // evaluate double userInput and send to cout with formatting
    std::cout << "Double that number is: " << 2 * userInput << '\n';

    // Here's a challenge: triple the input and output!
    std::cout << "Triple that number is: " << 3 * userInput << '\n';
    // ok not very hard. But it's a foundation.
    // From humble beginnings, programs from here on will grow in complexity.
    
    return 0;
}