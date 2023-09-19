// 1-x: Chapter 1 Summary

/* Write a program that prompts the user for two integers, then tell the user
the result of adding and subtracting those two numbers. */

#include <iostream> // for user I/O

int main()
{
    // take user input of two integers, on separate lines
    std::cout << "Enter an integer: ";
    // define first user input
    int firstInput{ };
    // store user input into firstInput
    std::cin >> firstInput;

    // repeat for second integer
    std::cout << "Enter another integer: ";
    int secondInput{ };
    std::cin >> secondInput;

    // Now, evaluate addition and subtraction expressions, and pass to cout
    // addition:
    std::cout << firstInput << 
        " + " << secondInput << 
        " is " << firstInput + secondInput << '\n';
    // subtraction:
    std::cout << firstInput << 
        " - " << secondInput << 
        " is " << firstInput - secondInput << '\n';

    // return 0 to end program execution
    return 0;

}