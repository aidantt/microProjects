// 2-11: header files

/* As programs grow in scope, even forward declaring every function in the
main.cpp file can become tedious. In this case, a header file is a special
type of program file that can be used to collect all forward function
declarations into a single file. 

We can reference previous work for a multi-file addition expression, now
also using a header file for increased compactedness.*/

#include <iostream>
#include "add.h"

int main()
{
    int x{ 5 };
    int y{ 5 };

    std::cout << x << " + " << y << " is " << add(x, y) << '\n';

    return 0;
}