// 2-8: Programs with multiple code files

/* as programs get larger, it becomes useful to split up parts of a program into
smaller source files. Here, a simple program that adds two user generated numbers
is shown as an example of multi-file projects.

If the #include preprocessor directive is used to reference outside files,
then the compiler will directly insert the contents of that file into the compiled
program. This is a useful way of doing things, but another way of referencing
outside files is to use a forward function declaration, basically telling the
compiler that a function that is called is defined somewhere in the program. */

#include <iostream> // preprocessor directive

int add(int x, int y); // forward declaration of add()

// main() function is defined
int main()
{
    std::cout << "Enter an integer: ";
    int x{};
    std::cin >> x;

    std::cout << "Enter another integer: ";
    int y{};
    std::cin >> y;

    // pass these two numbers to the add function in the output statement
    std::cout << x << " + " << y << " is " << add(x, y) << '\n';
}