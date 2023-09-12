// 1-1: Statements and program structure

/* 
A program is a sequence of computer instructions. A statement is a part of
a program that contains an instruction for the computer to do. When a statement
is made in a program, it is a type of instruction that dictates the performance
of some action in the program.
*/

/* 
A function is a collection of statements, usually seeking to achieve a certain
goal. The statements in a function are executed sequentially, and the function
sometimes returns a value before ending.

The main() function is present in every C++ program. It tells the computer where
to start executing code. Although it is not the only form of termination, a program
will always end when it reaches the end of the main function's statements.
*/

// Let's dissect the hello world program and see what statements there are:

#include <iostream>
/* 
This is a special type of statement called a preprocessor directive. This type of
statement tells the computer that the program uses functions that are defined in
some other file. Here, that file is called 'iostream', and it is a library file
in the core C library that allows programs to interface with certain aspects
of a command line.
*/

/* 
Then, line 35 defines the main function, of type int. An int type function
means that the function will return an integer at the end of execution. In the
case of main(), it will return a 0 if the execution of the program was successful,
and will otherwise return some other number. 
*/
int main()
{ // curly braces define the statements inside the function

    /* std::cout is a special type of statement from the iostream library that
    will print strings to the console. */
    std::cout << "hello world!";
    return 0; // return 0 signifies a successful execution of main()
}