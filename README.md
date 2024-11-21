# VIF

Vif is a programing language under development.

Vif looks like python, because I like how readable python is. But I also like rust a lot and took some inpiration from it in the design. In the end, you get something similar to python, but with a very different mechanic behind the scene.

- variables must be declared with the `var` keyword
- variables can be declared as mutable with the `mut` keyword.
- variables are __always__ passed by reference

Vif is a dynamic language, but the compiler does its best to do as much things as it can.

Vif's compiler and VM are made in rust, because I like rust. But I plan to switch to LLVM at some point, but for now I'm exploring on my own.

## Requirements

To build the project locally, you will need to have installed:

- llvm 16

If you are using nixOs, everything is in the `shell.nix`.

## Roadmap

I have been able to implement a solid base (at least according to my capabilities).

- [x] variables
- [x] mutability concept
- [x] functions
- [x] closures
- [x] error management
- [ ] typing
- [ ] classes
- [ ] modules
- [ ] decorator
- [ ] cloning object (since everything is passed by reference)
- [ ] standard library
- [ ] tooling

Currently, as I have been able to add functions and closures concepts, I am spending most of my time on refactoring stuff and building a solid base before conctinuing to add more functionalities. Hence, most of my time is spent on improving performances and having a nice error management.

## Examples

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
word = "bye"
assert word == "bye"
```

The compiler will raise an error if you try to modify a non-mutable variable

```python
var word = "hello"
word = "bye" # crash
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

In term of performance, I was deeply hoping to be much faster than python. I'm just a tiddy bit faster than python, and it took me some efforts to get there ! I'm very happy about it, but I hope this can be improved a lot in the future.

It's a never ending journey than optimizing Vif !
The commands below helps me to benchmark different version of Vif.

```bash
# if not done already
cp ./target/release/vif-cli ./target/release/faster

# change stuff and rebuild a new bin that you can use for the benchmark
cargo build --release

# run benchmark with hyperfine
hyperfine -w 10 -r 100 './target/release/vif-cli ./snippets/benchmark.vif' './target/release/faster ./snippets/benchmark.vif'
```

