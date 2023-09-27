// 4-3: Object sizes and the sizeof operator

/* As covered in 4-1, memory on a computer is organized into byte-sized
units, with each byte of memory containing 8 bits and corresponding to a
unique memory address. Although it is simpler to think of memory as a collection
of unique "mailboxes" or containers, it is often the case that data will take
more than one byte of memory.

A single object might use 1, 2, 4, 8, or even more consecutive memory addresses.
The amount of memory that an object uses is based on its data type. Because it is
typical to access memory through variable names rather than direct memory addresses,
The C++ compiler often hides the details of how much memory a given object uses,
generally to aid readability and simplicity.

Despite this, it is often useful to know how much memory an object takes up, for
several reasons. The sizeof operator is a core piece of C++ syntax that displays
the "size of" the type of the given argument. */

// The following code outputs sizeof() for each basic data type:

#include <iomanip> // for std::setw (which sets the width of the subsequent output)
#include <iostream>

int main()
{
    std::cout << std::left; // left justify output
    std::cout << std::setw(16) << "bool:" << sizeof(bool) << " bytes\n";
    std::cout << std::setw(16) << "char:" << sizeof(char) << " bytes\n";
    std::cout << std::setw(16) << "short:" << sizeof(short) << " bytes\n";
    std::cout << std::setw(16) << "int:" << sizeof(int) << " bytes\n";
    std::cout << std::setw(16) << "long:" << sizeof(long) << " bytes\n";
    std::cout << std::setw(16) << "long long:" << sizeof(long long) << " bytes\n";
    std::cout << std::setw(16) << "float:" << sizeof(float) << " bytes\n";
    std::cout << std::setw(16) << "double:" << sizeof(double) << " bytes\n";
    std::cout << std::setw(16) << "long double:" << sizeof(long double) << " bytes\n";

    return 0;
}