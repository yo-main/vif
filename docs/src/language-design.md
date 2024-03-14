# Language Design

## Types

Vif handles several variable types for now:

- string
- integer
- float
- boolean
- None

More will come in the future (list, tuples, struct)

Like in python, you can do operations on them where it makes sense, but it's a little bit more strict

```python
assert 1+1 == 2
assert 1.0+1 == 2.0
assert "abc" + "def" == "abcdef"
assert True + True == 2

# but the below don't work
 -> assert "abc" + 1 == "abc1"
```

## Variables

A variable must be declared with the `var` keyword.

```python
var my_variable = "Hello"
```

Right now I think pretty everything can be a variable, except keywords:

- return
- while
- def
- ... to be documented

## Functions

```python
def say_something(var_1, var_2):
    print(var_1, " , ", var_2)

say_something("hello", "world")
```

Very similar to python in how it looks and how it is like.

The core functionality of functions is working today.
But a few things, nice to have, are still missing:

- no bug closure (today a closure cannot have closure itself)
- calling function with named parameters
- having default values for parameter

## Mutability

A core aspect of Vif is the notion of mutability. 
A variable can be reassigned a new value __only__ if it is mutable
By default, variables are not mutable, you need to declare them as such using the `mut` keyword, like rust.

```python
var mut my_variable = "Hello"
my_variable = "World"
```

You can only assign to a mutable variable something that is mutable, meaning:

- a core type: int, string, float...
- a mutable variable
    - a variable which is mutable itself
- a mutable function
    - a function that can only returns something mutable

This check is done at compile time and will prevent the program to load if any incoherence is found.

```python
def incr(mut counter, incr_by):
    counter = counter + incr_by
    return counter

var mut counter = 0
var variable_not_mutable = incr(counter, 10)
assert counter == 10

# this fails because "variable_not_mutable" is not... mutable
incr(variable_not_mutable, 10)
```

* actually it doesn't fail currently, it's a bug that I just identified and will fix


## Variables are passed by reference

Variables are passed by reference. It means the below is possible

```python
var mut a = 1
var b = a

a = 2
assert b = 2
```

That can be weird, but I feel that coupled with the notion of mutability, it will bring many benefits to the Vif, and, I hope, more performance because we are cloning/copying less things than passing by values.


## Class & interface

Nothing done yet. And I'm not sure how class will take form yet.

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

Typing __will__ be a huge part of the language.
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
