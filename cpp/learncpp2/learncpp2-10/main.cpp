// 2-10: Intro. to the preprocessor

/* the C++ preprocessor is a part of the C compiler that prepares a program
to become an executable. In general, the preprocessor contains its own set of
syntax and statements that can be used in a program.

The #include statement is used by the preprocessor, and is thus called a
preprocessor directive. It is a statement that tells the preprocessor to do
"something". In the case of #include, it tells the preprocessor to look for the
mentioned file, and insert it where the #include statement was. */

#include <iostream> // this line replaced with the contents of iostream

/* Another type of directive is the #define directive. This directive can be
used in two main ways: as a function-type macro, or as an object-type macro.
The usage of #define for function-type macros is generally a bad idea, as
everything that #define can do can be done better by typical C++ functions.

It is also generally not advised to use #define for substitution macros,
such as the following: */

#define SOMEONE "no-one"

/* While this seems like a useful way to define literals and other constants,
as was sometimes done in legacy C programming, C++ generally has better methods
to define constants inside programs.

The last way of using the #define directive is non-substitution object-type
macros. By giving a definition to a preprocessor "variable", it can be used
in conditional statements to conditionally remove entire blocks of code before
even compilation. */

#define YES

int main()
{
    std::cout << "Hello World!\n";

    std::cout << SOMEONE << '\n';

#ifdef YES // preprocessor conditional directive
    std::cout << "YES\n";
#endif

#ifndef NO
    std::cout << "NO\n";
#endif

    return 0;
}