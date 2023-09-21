// 4-1: Intro. to fundamental data types

/* A computer is a machine that can store values in data and 
execute programs that manipulate this stored data. The first
three chapters serve to establish the basics of how a program
can manipulate data. Now, in chapter 4, we pivot to exploring
how a computer stores data.

The smallest unit of memory that could possibly be stored as information is a
binary digit, called a bit. The definition of a binary digit is a mathematical 
object that can represent a 0 or 1. A computer uses a predetermined method to
read data encoded in binary, such that a string of bits can represent data.

Using a circuitry structure called random access memory (RAM), a computer can
store the state of an array of bits by taking advantage of different properties
of current flow. Memory stored in RAM is organized into sequential units called
memory addresses. In modern computer architectures, each bit does not get a
dedicated address in memory. Instead, each memory address stores 8 bits of data,
or a byte of data. In C++, data is typically worked with in byte-sized chunks.

Although all data is just a sequence of bits, programs abstract data into separate
data types to tell the compiler how to treat the data, such that it will be
assigned in memory in a way that the architecture knows what to do with it.

When a value is assigned in a program, the compiler takes care of encoding that
value into the appropriate sequence of bits for that data type. */

int main()
{
    // examples of different data types

    // float corresponds to floating point, or a number with a fractional part.
    float pie{ 3.14159 }; // normal float type
    double duo{ 4.97197491739571935 }; // longer float point precision

    // bool represents binary states
    bool present = true;
    bool absent = false;

    // char represents a single character of text
    char eyy{ 'a' };

    // int, hopefully familiar, represents whole number integers.
    int one = 1;

    return 0;
}