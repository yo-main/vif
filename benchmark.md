Reaching a stage where I can benchmark Zeus against itself or other languages.

Let's always use the release binary to run those tests.

Let's use the `snippet/fibo.zs` script for the time being.

```
def fib(n):
    if n <= 1:
        return n
    return fib(n-2) + fib(n-1)

var i = 0
while True:
    var start = get_time()
    var value = fib(i)
    var duration = (get_time() - start) / 1000000

    print("Got", value, "after", duration, "seconds")

    i = i + 1
```

```
2023-10-29 - 1346269    - 61 seconds - AST version
2023-12-03 - 24157817   - 12 seconds - stack-based version
2023-12-03 - 24157817   - 3  seconds - python
2023-12-03 - 4807526976 - 13 seconds - rust
```