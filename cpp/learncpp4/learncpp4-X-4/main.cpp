// 4-X-4: Simple Gravity Simulation

/* Write a short program to simulate a ball being dropped off of a tower.
The user should be asked for the height of the tower in meters. Assume a
constant gravity of 9.8 m/s^2, and that the ball has no initial velocity.

Have the program output the height of the ball after 0, 1, 2, 3, 4, and 5
seconds. The ball should not go underneath the ground (height = 0).

Use a function to calculate the height of the ball after x seconds. The
function can calculate the height of the ball after x seconds using the 
kinematic formula:

distance fallen = gravity * (x_seconds^2 / 2)

This program would be much better with the usage of loops, but for now,
just calculate up to 5 seconds, even if the ball does not reach the ground. */

#include <iostream> // for user input

double getHeight()
{
    std::cout << "Enter the height of the tower in meters: ";
    double height{};
    std::cin >> height;

    return height;
}

double heightAt(int seconds)
{
    
}

int main()
{
    // first: get the height of the tower from the user.
    double height{ getHeight() };

    // For seconds 0 - 5, calculate the height of a falling ball
    
    return 0;
}