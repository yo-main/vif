# Language Design

I am not 100% of all the details and things might change but here's a few snippets of what I have in mind.

## Function & variable

```python
def say_hello():
    var world = "world"
    print("Hello", world)

say_hello()
```

As announced, it does look like python. The only different I introduce here is the `var` keyword. 
Because the way local/global interacts in python always bugged me a little: having to use `local` and `global` bring in some complexity in my eyes.

And I also suppose it will be less complex to manage for the interpreter because we are more strict on how variables are assigned/mutated.

## Constants

Vif has the notion of a constant, and as well as mutable for function parameters.
By default, function parameters are not mutable, you need to specify which parameter should be mutable.

This is probably one of the most important difference with Python. Directly influenced by Rust as you might have guessed :)

```python
var mut one = 1
var two = 2

def add(a, b):
    return a + b

def substract(mut a, b):
    return a - b

assert add(one + two) == 3

# the compiler should fail here because `one` is a constant but the function's signature is expecting that mutable value
assert substract(one, two)
```

Here we create 2 constants and pass them to the `add` function. By default those parameters are considered as constant, t

You'll notice that parameters are defined with the `const` keyword. It's a way to indicate that those parameters cannot be altered by the function.

It's ok to pass a `var` as a `const`, but the reverse is not allowed.


## Class & interface

I'm not sure how class will take form yet.

I do like how they work in python, especially magic methods. It's very powerful to integrate in the language, but I don't want 
it to cost too much from a performance point of view. So those magic methods might come in a second time.

```python
class AlgorithmConcrete:
    word = "hello"

    def add(self, a, b):
        return a + b

    def substract(a, b):
        return a - b


interface AlgorithmInterface:
    word: str
    
    def add(self, a, b):
        ...

    def substract(a, b):
        ...

def coucou(algo: AlgorithmInterface):
    algo.add(1, 2)
```

I do want some kind of interface though. Different from a class and very similar to what a `Protocol` is in python.

Another difference is that `self` is part of the language. 
You can add it, or not, Vif is smart enough to know if it's an instance method.

You can add attribute in the interface, on top of methods.


## Typing

Typing will be a huge part of the language.
I want it to be natural and smart, letting the compiler guess what should be the type of some variables if needed.
If a type cannot be guessed and is not specified, then we would check that on runtime.

But it will obviously lower performances.

```python
# here vif should be smart enough to understand that 
# `a` should be of type int and `b` of type int or float
def add(a, b) -> int:
    return a + b
```

I wonder if that kinf of typing can exist. A mixt between a compiled language and an interpreted one.
Interpreted languages are still compiled though, so I guess/hope this might be possible.

We'll see what kind of mountains await me on that !
