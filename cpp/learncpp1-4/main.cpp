// 1-4: variable assignment & initialization

#include <iostream>

int main()
{
    // as covered in 1-3, variables are objects that can store data in memory.
    // a variable can be defined in a statement, like this:
    int x; // define a variable named x, of type int

    // the type of a variable determines what kind of data it stores.

    // After defining a variable, it can be assigned a value in memory:
    int width; // define a variable named width, of type int
    width = 5; // assign the value '5' to width

    // now, when width is referenced, the space in memory it is assigned
    // will store the value '5'.

    // the equals sign (=) is the assignment operator. When used correctly
    // in a statement, it will assign a value to an object.

    // when a variable is assigned after being defined, it is called
    // copy assignment.

    // now, the value stored in width can be manipulated:
    std::cout << width; // prints '5' to the console

    // the value stored in width can change with another copy assignment:
    width = 7; // change value stored in width to '7'

    std::cout << width; // prints the new value of width '7' to the console

    // C++ is capable of reducing the number of statements needed to assign
    // a value to a variable. When a variable is assigned a value in the same
    // statement that it is defined, it is defined as initialization.

    // there are 6 basic ways to initialize variables in C++:

    int a;          // no initializer (default initialization)
    int b = 5;      // copy initialization
    int c( 6 );     // direct initialization

    // List initialization methods
    int d { 7 };    // direct list initialization
    int e = { 8 };  // copy list initialization
    int f {};       // value initialization

    // in general, it is best practice to initialize using list initialization.
    // it's also best practice to always initialize variables upon creation.

    return 0;
}