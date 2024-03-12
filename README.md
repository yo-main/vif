# VIF

Blam, say hi to Vif, the language that will take over python.

## Overview

Vif is a programing language under development. I have no experience whatsoever in buiding languages, but it looked interesting and I wanted to try building a new one.

https://craftinginterpreters.com/ gave me a huge boost, and then I went rogue with my own, strange, ideas...

Vif aims to look like python, because I look how python looks like. But a few things are very different from it:

- `var` is used to declare variables
- `mut` is used to mark a variable as mutable
- variables are __always__ passed by reference

Vif is an interpreted language, but lot of checks are still hapenning through compilation.

### Examples

Let's start with a simple "Hello world" program !
```python
print("hello, world!")
```

And see how variables comes into play

```python
var word = "Hello"
var sentence = hello + "," + "world !"
assert sentence == "Hello, world!"
```

To modify a variable, it needs to be declared with `mut`

```python
var mut word = "hello"
word = "olleh"
assert word == "olleh"
```

The compiler will raise an error if you try to modify a non-mutable variable

```python
var word = "hello"
word = "olleh" # crash
```

This mutability also applies to function parameters

```python
def add_excitement(mut word):
  word = word + "!"

var mut string = "Hello"
add_excitement(string)
assert string == "Hello!"
```

The compiler will not accept a non mutable variable to be passed to `add_excitement`.

This implies many other things, and you can find more details about it in the documentation.


## Usages

### How to build

```bash
cargo build --release
```

### How to open a vif shell

```bash
./target/release/vif-cli
```

### Open documentation

```bash
mdbook serve --open ./docs
```

### Run test suites

```bash
cargo test --workspace
```

### Benchmarking

In term of performance, I was deeply hoping to be much faster than python. But well I'm still an amateur and I'm still slower than python when computing the fibonacci sequence through recursive calls (like 3 or 4 times slower).

I have not given up through. It's a never ending journey than optimizing Vif !
The script below helps me to benchmark different version of Vif.

```bash
# if not done already
cp ./target/release/vif-cli ./target/release/faster

# change stuff and rebuild a new bin that you can use for the benchmark
cargo build --release

# run benchmark with hyperfine
hyperfine -w 10 -r 100 './target/release/vif-cli ./snippets/benchmark.vif' './target/release/faster ./snippets/benchmark.vif'
```

## Road map

Well for now I'm frankly happy to have come that far.

Lot of bugs are still presents, and lot of features are missing, but here's what I am proud to have achieved:

- functions
- closures (limited to F+1, don't try to encapsulate a closure inside another closure)
- mutability
- passing by reference

I struggled a lot to make that happens and I have something which is good enough and tested enough so that I'm can confidently say that it's working as expected for common cases.


What I would like to add in the future ?

- Class or Interface, to be decided which one
- Modules
- Typing
- LLVM as backend ? (whoooa dude stop dreaming)
