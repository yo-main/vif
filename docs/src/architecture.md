# Compiler

Vif is a dynamic language.

It has a compiler that tries to do as much as possible as a way to reduce things to be done on runtime.
Like any compilers goal, I guess.

And then a VM that manage the run time.

Since I have no knowledge about how to build a programing language, I followed the [crafting interpreter](https://craftinginterpreters.com/) course, and lots of the architecture is coming from there.

The parser is following the pratt parser algorithm. Even though I'm not sure anymore at this stage because many things changes and I can't say for sure what algorithm I have been using.

Just know that it generates a AST.

The compiler is stack-based. It's not a one-pass compiler, as I have the AST and there's some optimizations steps done before being passed to the VM. Well not yet optimized, but that's coming ! I'll describe what are the different optimization steps here.

And then the VM follows. It does what it can. I would like to get rid of it at some point and use LLVM but I'm far from that.

One advantage of using rust for the VM is that I don't have to manage a garbage collector (I think). Using LLVM, this won't be possible anymore.


