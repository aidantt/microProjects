// 2-1: Intro to functions

/* In C++, a function is no more than a bundle of statements
that performs a manipulation of data. So far, we've already covered a certain
function that is present in almost every program in C++: main().

main() is defined as the "root" function of C++. It is where execution of
statements always begins and usually ends. User defined functions can be 
defined before the main function, and "called" inside the main function in order
to start execution of the function's statements. */

/* almost all functions take the syntax of the following:

returnType functionName() // function definition
{
    // function body
} // end of function scope

*/

#include <iostream>

// An example of a function definition
void doPrint()
{
    std::cout << "In doPrint()\n";
}

// definition of main(), always appends other function definitions
int main()
{
    std::cout << "Starting main()\n"; // executed before doPrint()
    doPrint(); // calls into doPrint() scope

    std::cout << "Ending main()\n"; // executed after doPrint()

    return 0;
}