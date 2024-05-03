## what is the stack

it is a special region of the process memory that stores variables created by each function.

The memory for each function is called stack frame and this is where our local variables live for every function.

For every function call a new stack frame is allocated on top of the current one.

The size of every variable on the stack has to be known at compile time.

When a function is exists it's stack frame is released.

The stack has limited size annd the size is determine by the machine architecture, operating system,compiler and other factors.

## What us the heap

It's a region of the process memory that is not automatically managed for us.

- It has no size restrictions
- It is accessible by any function, anywhere in the program
- We have to always deallocate the memory that we have allocated on the heap
what the restof the team will be doing while you are setting up the project

What will the rest of the team be doing while you are setting up the project?