# Zeus

Zeus is supposed to be a simple, straight language for every day usage.

## A python replacement

It aims to replace python, plain and simple.

Why, you ask ?
Well, valid question. I like python. A lot. It's elegant, simple, easy to learn.
It's the best way to onboard more people into the programing world. That's how I came in.

But I'm also asking myself a lot of questions regarding python and its energy consumption. Seeing it
as one of the most energivore language saddens me. It's a great language, I love it, but I consider
that the world should offer us a language similar but which much better performance.

That's what I aim to build. Yeah, that may be a huge claim, but that's my goal, and my way to motivate
me to start working on that project.

It's also a way for me to learn more about what's a programing language.

I consider programing an art, and given my objectives for Zeus, it should be my most beautifully crafted piece of code.

## Main features

### Typing (not)

I'm not sure I want to make it mandatory. I think it's convenient to forget about typing for small scripts and moving on quickly.
But it should still be part of the language. Big projects will rely on it. 

This will have an influence on performance for sure. Something typed will be more efficient. 

Can typed and not typed cohabit together ? That I'm not sure and will have to dig through this.
Probably that if something is typed but receives a wrong type while running, an error should be raised. If it's not typed, it our language
to do its best with what's given to it.

### Dynamic

I want it dynamic but I'm not 100% sure yet.

It goes hand in hand with the not typed thing I wanna say, but I don't see why it should be a problem using a compiler.
Things need to get parsed and compiled anyway.

Seems to me more friendly for beginners/debugging/scripting to have a dynamic language rather than compiled one.

I find packaging a bit more complex on a dynamic language rather than compiled though, as you just need a binary for the compiled language.
But that's something I can solve by good tooling.

### Tooling

I should provide a lot of tooling for the language if I want it to be adopted by the community:
- package manager
- formatter
- lsp
- test utils
- profiling !!!!
- ease of packaging for publisher

### Async or not async

I think I'm getting more and more convinced that we should have a solid thread utilty, and not focus on async.
I mean, async is nice and depending on the use case it might be more effective than thread. But I'm getting the 
understanding that those use cases are a minotiry and right now async is just the new sexy.

Maybe having a easy to use thread library will fix that ?

### Indentation

It might be controvertial, but it's an assumed choice. Indentations is what make python readable.
It's not just a matter of linting and formatting. I find it easier for us human to read and understand
code without blocks.

As much as I love rust, python readability is unbeatable.

