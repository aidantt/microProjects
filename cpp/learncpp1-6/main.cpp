// 1-6: uninitialized variables & undefined behavior

#include <iostream>

/* in C/C++, variable initialization is not automatic. When a variable is defined
without initialization, it is not auto-initialized to 0 or some null type value,
as may be seen in higher-level languages like python. This was actually an
intentional aspect of C, as the original language was developed during a time
in which initializing every variable could actually affect the performance of
a program. Nowadays, it is best practice to always initialize variables upon
definition, as uninitialized variables are prone to showing undefined behavior,
as can be seen in the following examples. */

int main()
{
    // define an integer variable x
    int x;

    // print the value of x to the screen
    std::cout << x << '\n';
    // we don't know what will be output, as x is uninitialized

    // alongside undefined variable behavior, there are other types of undefined
    // behavior that should be avoided.

    // Implementation-defined behavior is when the behavior of the program is
    // determined by the interpretation of the compiler.
    std::cout << sizeof(int) << '\n';

    // The above statement will produce different output based on which compiler
    // is interpreting it. Most will output 4, but some will output 2.

    // Best practice: avoid implementation-defined behavior.

    return 0;
}