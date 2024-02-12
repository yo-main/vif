Currently variables are managed on the stack.

When a variable is needed, the compiler set the index in the function scope of where this variable should be, and the VM fetches it and clone it.

Values are then fetched and popped as they are used.

There is also the notion of inherited variables, where in that case the compiler is able to set the variable from the function where the closure is declared, and the VM is able to adjust when parsong the stack to fetch the correct variable.

I intend to add another kind of variable management, regarding to how variables are passed in functions.

Currently we kind of take the advantage of the stack by getting the function and its parameters, then the VM readjust itself on the stack by the arity. This is nice, but variables are still cloned then.

On top of that, I want to pass variables by their reference, so we don't need to clone them and we can mutate their original value.

Not sure how to do that but I guess it will happen in the compiler `call` management, as well as the VM maybe ?

My idea is to have a new kind of function to manage function parameters, we resolve the variable and set a reference to it for the VM, kind of like inherited variable.
