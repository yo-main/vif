# Hello

Welcome to the documentation of the Vif Language.

Vif is being built with the aim of... taking over python !

Big claim, very low chance of success. Especially with someone like me. But hey, I'm building Vif with that goal in mind and this is what keeps me motivated to work on it. 

Please do not come here to crush my hope, I don't need you to know this will never happen anyway. So I'm trying to have fun while build my dream.

## The motivation behind Vif

I love python. It's a wonderful language. It's beautiful, it's user friendly, it's elegant. It's the language through which I came into programming and I'm really grateful that it's python and not javascript (kidding (or not?)).

But damn, it's slow. It consumes much more energy than most other languages for doing the same task. And since it's one of the most used languages all over the world, it's kind of drilling a hole in my soul. Vif is my attemp of filling in that hole.

That's the initial thought. But building Vif is also a way for me to play with rust and learn how to build a programing language, how it's compiled and interpreted.

So far it has been a very nurturing journey and I am far from having reached the end of it.

## The plan

Because I like python, I want Vif to look like it. Buuuut I'll try to bring in some changes of my own, things to make it even more friendly and, especially, performant, I think. 

The problem of Python is that it's old. It cannot change much its functionalities because it's used by thousands and you want to avoid breaking change. Right Python ? Right ? Anyway, Vif does not have that problem, literally nobody is using it.

But there's a small issue in my plan to conquer the world with Vif. I have zero knowledge about building programming languages. I'm learning while building it. For sure this does not help in building Vif in the best possible way, and there will be probable a lot of mistakes. 

Here are a bunch of things which are still to be implemented:


### Tooling

- [ ] packaging
- [ ] tree sitter
- [ ] linter
- [ ] formatter
- [ ] lsp server


### Language features

- [x] variable
- [x] function
- [x] closure
- [x] constant & mutables
- [ ] class/interface
- [ ] typing
- [ ] error management
- [ ] module
- [ ] standard lib
- [ ] LLVM 


#### Closures

I'm really considering whether I want closure in vif or not. It does complexify things a lot, and I don't see the usage of closures 
that critical in python. They could be replaced by simple functions, we just need to pass parameters in.

One advantage of vif is that passed variables can be mutated, contrary to python, so it does not prevent any use case and it enforces better
design practices.

Example of replacement

```python
def counter():
  count = 0

	def wrapper():
		count = count + 1
		return count

	return wrapper

var incr = counter()
print(incr())
print(incr())

## and in vif
def incr(mut i):
  i += 1
  return i
  
def counter():
  return partial(incr, 0)

var incr = counter()
assert incr() == 1
assert incr() == 2
```

One key aspect here is that variables are passed by reference. So the `0` I pass in the counter is actually stored on the stack and its his address that is passed to the function and thus incremented.

Anyway, this probably doesn't cover all of the use cases of closures. And at some point I want to add decorator in Vif because I love them. I'll need closure for that so... Even for now I have a few bugs in the way closures are managed, cf below, I'll have to fix them.



