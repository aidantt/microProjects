// 1-5: Intro to iostream; cout, cin, & endl

#include <iostream>

/* 
iostream, fully known as the input/output library, is a core part of the C++
standard library. As a part of the core library, iostream handles OS-level
interfacing between a program and the console, and simplifies these
interactions into compact functions and operations.
*/

int main()
{
    /* 
    std::cout is a variable that determines data that is shown to the console.
    When the insertion operator (<<) is used with cout, it stores whatever is
    given on the right side of the operator into the cout variable.
    */
   std::cout << "Hello World!"; // print Hello World! to console

   // cout can also store other types of data, like integers:
   std::cout << 4; // print 4 to console

   // as expected, cout can take data stored by variables:
   int x{ 5 }; // define integer x with value 5
   std::cout << x; // print value of x to console

   // The insertion operator can be used with multiple sets of data:
   std::cout << "Hello" << " world!"; // prints 'Hello world!' to the console

   // This can be used to give output that varies at runtime, based on variables
   // values:
   std::cout << "x is equal to: " << x;

   // In order to invoke a carriage return (equivalent to pressing Enter),
   // iostream has a variable named endl:
   std::cout << "Hi!" << std::endl;
   std::cout << "My name is herbert." << std::endl;

   /* the downside of endl is that it not only moves the cursor, but also
   flushes the buffer. Put simply, cout functions by taking data into
   a buffer, and periodically posting to the console before flushing the
   buffer and awaiting new input. Since cout usually flushes the buffer
   by itself optimally, endl is usually inefficient. When only the cr
   is needed, \n is a part of the ascii set that moves the cursor to the
   next line. */

   std::cout << "x is equal to: " << x << '\n';
   std::cout << "And that's all, folks!\n";

   /* in contrast to cout, cin is a variable that can take data from the
   console and store it in an object. This finally allows programs to
   execute based on user input, and manipulate data that is given at runtime,
   as opposed to compile time. */

   std::cout << "Enter a number: "; // ask user for a number

   int y{ }; // define integer y and zero initialize it
   std::cin >> y; // get input from console and store it in y

   std::cout << "You entered: " << y << '\n';

   std::cout << "Enter two numbers seperated by a space: ";
   
   std::cin >> x >> y;

   std::cout << "You entered " << x << " and " << y << '\n';

   return 0;
}