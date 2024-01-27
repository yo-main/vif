# ZEUS

Blam, welcome to Zeus repo, the language that will take over python.

Is zeus a final name ? I am really not sure, but for now I'll stick with it.
Maybe a more adapted name will show itself as I'm progressing in making that language a real thing.

## The plan

So, what's the plan ? What's the goal ? What's my endgame ?

My plan is to take over the world, take over python. Plain simple.

Questions ? No ? Good. Let's go.

Just kidding, I have a few questions myself so let's try to answer them here.

### Why taking over python ?

Because I love python. I find its syntax pleasant, sorry for people against indents.
It's the language through which I came into programming and I'm really grateful that it's python and not javascript (kidding (not)).

But more seriously, despite all my love for Python, a few things about it bugs me:

__It's damn slow__

I mean, compared to most of the others languages, it's one of the slowest and my green concious isn't really happy about it.
It's not like python is some kind of underused language or whatnot. No. It's present everywhere, but it's slow and takes a lot of memory from processes.

It's not because today's hardware allows it that it's acceptable, at least to my eyes.

So one of my major goal would be to design something looking similar to python, but trying to change a few things under the hood to make it more resource friendly.

Because yeah, I'm smarter than all the people working on python. That and also because I don't know shit about compilers and interpreters. So I thought, why not try to learn how to do it, while building the future most used language in the world.

Two birds, one stone. I like it, so here I go.

Of course one of those goals is completely out of reach. I mean, how can a guy like me with no computer science background could even build something remotely close to python, performance wise. 


### Milestones ?

Good question. What are my next steps ? Should I try to design how my future language will look like ? Should I watch/read all available resources on compilers ?

Nah, not my style. I do know though I want my language to look like python, so that's enough for a start. Let's just jump into coding, I'll figure out the rest later.

So first goal, be able to read that piece of code here:

```python
def entrypoint():
    var world = "world"
    print("Hello", world)

entrypoint()
```

It does look like python. The only different I introduce here is the `var` keyword to declare a variable.
Because why not ? And because the way local/global interacts in python always bugged me a little.

A few things I know I would like to see in my language:

- constants and variables (so I would need keywords to differentiate them)
- native typing. I always felt the typing in python isn't very native - I want my interpreter to crash when it encounter some errors through typing. I'm not sure yet how this will be implemented and to which degree, but that's an idea I like.

Anyway, there's a lot going on already on that snippet. We implement a function, define some variables, declare and call functions, indentation...
That would be a huge step to make that happens !

Things that can be added at that point:
- typing
- classes
- errors
- modules
- closure ?
- Null statement ?


I don't know much about how this language will be designed. I intend to do mistake and learn while doing.
I should not be afraid to take time to think, graph, document everything if I get blocked to find a solution.





