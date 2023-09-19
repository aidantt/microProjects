// 2-2: function return values

/* all functions have a "returnType" identifier. This signifies to the program
what the function is intended to do. Functions of type "int" will return
an integer, while functions of type "String" will return a string.

However, not all functions must return an explicit value to main. If a function
is not intended to return any value, and instead acts only as shorthand for a
series of statements, it can be classified as a "void" function. */

/* Back in ch.1, we made a program that took an integer from the user and
doubled it. While this can be done with only a main function, as all programs
technically can be, separating tasks into user-defined functions can increase
readibility and logic flow. */

// Using functions, take an integer from the user, double it, and output it.

#include <iostream>

int getUserInput() // definition of user input function
{
    // prompt user for input and store in a zero-initialized variable
    std::cout << "Enter an integer: ";
    int input{ };
    std::cin >> input;

    return input; // input is passed to whatever called getUserInput()
}

int doubleInput(int input)
{
    // here, the task is simple enough that it can be done in single expression
    // this expression can be fit inside the return statement:

    // while fairly trivial, this helps readability and is a good example of how
    // functions can aid in the abstraction of programs.
    return (2 * input);
}

// definition of main(), start of execution
int main()
{
    // first, we need a function to get user input.
    // the best way to do this is to initialize an integer with a function
    // that will ask the user for input and return this input back to main.

    // define integer to hold user input
    int input{ getUserInput() }; // getUserInput() is called

    // with input stored, call a doubling function and output the results:
    std::cout << input << " doubled is: " << doubleInput(input) << '\n';
}