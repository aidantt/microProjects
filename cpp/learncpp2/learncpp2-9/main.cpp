// 2-9: Naming collisions & intro. to namespaces

/* In the case that a function is defined in two separate files and never called,
and the definitions of the function do not agree with each other, the linker
will complain upon attempting to resolve the different files that the function
has conflicting definitions, even if it is never called.

This type of conflict in attempting to compile a program is a naming collision,
two functions that have the same name with different bodies. In increasingly
large programs, this problem becomes exponentially apparent. In order to resolve
naming collisions, C++ uses a concept called a namespace.

A namespace is a region of scope that allows us to declare names for the
purpose of disambiguation. A name declared in a namespace won't be
mistaken for an identical name declared in another scope. */

/* In C++, everything that is not given a namespace is placed inside of the
global namespace scope, which is where functions like main() reside.

Although not usually recommended, the "using" preprocessor can eliminate the
need for using std or other namespace identifiers. */

#include <iostream>

using namespace std;

int main()
{
    cout << "Hello World!";
    return 0;
}