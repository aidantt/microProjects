// 1-3: Intro to objects & variables

/* 
A variable is a thing in a program that is capable of storing a value.
A value is any piece of data that can be represented in binary to a computer.

All computers have memory that can be used to store data, called RAM. Most of
the time, RAM can be thought of as a series of numbered boxes, where each box
can store data in the form of a number.

Although it is seen in older languages, C++ discourages direct access of data
in memory. Instead, we represent data in the form of a high-level object. Most
simply, an object is a region of memory that can store a value.

Instead of referencing a certain place in memory to access data, an object can
be referenced within a program indirectly. This improves readability, while also
improving the versatility of the program. With an object, a value can be assigned
in the program, and the explicit location in memory can be determined by the computer
during runtime.
*/

/* 
An object can be named or unnamed. A named object is called a variable, and the name
of the object is the identifier.
*/

/* 
The following main() function has different examples of how objects can be
created in C++. 
*/
int main() 
{
    int x; // define a variable x, of type int
    // x is now instantiated, and will be assigned a place in memory at runtime.

    double width; // define a variable width, of type double

    // you can also instantiate multiple variables in a single statement:
    int a, b; // not best practice, too vague
}