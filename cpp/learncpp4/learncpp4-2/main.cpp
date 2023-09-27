// 4-2: The void type

/* Void is the first example in core C++ of an incomplete type. An incomplete
type is a type that has been declared but not yet defined. The compiler
recognizes these types, but does not know enough about the variable to declare
memory allocation for the type.

The incomplete nature of the void type is intentional, as it represents the lack
of a type and cannot be defined.

The vast majority of the time, the void type is used to define a function that
will not return any value to the function call. This allows programs to contain
functions that exist solely to execute statements, usually on a conditional
basis. */

#include <iostream>

void writeValue(int x) // function of type void with one passed parameter
{
    std::cout << "The value of x is: " << x << '\n';

    // writeValue() has no return value, as it is of type void, which cannot
    // be defined.
}

int main()
{
    int x{2}; // instantiate x with value 2
    
    writeValue(x); // call a function writeValue and pass x as an argument
    
    return 0;
}