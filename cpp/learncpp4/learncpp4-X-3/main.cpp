// 4-X, Question 3

/* 

Problem Statement:

Write the following program: The user is asked to enter 2 floating point numbers 
(use doubles). The user is then asked to enter one of the following mathematical 
symbols: +, -, *, or /. The program computes the answer on the two numbers the 
user entered and prints the results. If the user enters an invalid symbol, 
the program should print nothing. 

Solution Outline:
    1. execution begins at main()
    2. User is prompted for two inputs in the form of doubles, function
    3. User is prompted for mathematical symbol, function
        a. conditional return if the input symbol is invalid
    4. program computes the resulting expression, function
    5. program prints evaluated expression, with formatting

*/

#include <iostream>

double userInputDouble()
{
    // prompt with std::cout and store result
    std::cout << "Enter a double value: ";
    double inputDouble{};
    std::cin >> inputDouble;

    return inputDouble;
}

char userInputSymbol()
{
    std::cout << "Enter one of the following: +, -, *, or /: ";
    char symbol{};
    std::cin >> symbol;

    return symbol;
}

void printResult(double result, double firstInput, double secondInput, char symbol)
{
    std::cout << 
        firstInput << ' ' << symbol << ' ' << 
        secondInput << " is " << result << "\n\n";
}

int main()
{
    // prompt user for input of two double values
    double firstInput{ userInputDouble() };
    double secondInput{ userInputDouble() };

    // prompt user for mathematical symbol
    char symbol{ userInputSymbol() };

    // use conditionals to perform the correct operation
    if (symbol == '+')
        printResult((firstInput + secondInput), firstInput, secondInput, symbol);
    
    else if (symbol == '-')
        printResult((firstInput - secondInput), firstInput, secondInput, symbol);

    else if (symbol == '*')
        printResult((firstInput * secondInput), firstInput, secondInput, symbol);
    
    else if (symbol == '/')
        printResult((firstInput / secondInput), firstInput, secondInput, symbol);

    else if (true) // if symbol is invalid
        std::cout << "Error: Invalid Symbol\n";
    
    return 0;
}