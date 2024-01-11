### what is the stack

it is a special region of the process memory that stores variables created by each function.

The memory for each function is called stack frame and this is where our local variables live for every function.

For every function call a new stack frame is allocated on top of the current one.

The size of every variable on the stack has to be known at compile time.

When a function is exists it's stack frame is released.