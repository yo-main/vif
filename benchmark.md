Reaching a stage where I can benchmark Vif against itself or other languages.

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
2023-12-03 - 24157817   - 12 seconds - 107166150a040e98715a44338d146a9d7748a4f9
2023-12-03 - 24157817   - 11 seconds - 870c8990dfa7de692e400c348c645a4e5dfd4fa4
2023-12-07 - 24157817   - 11 seconds - 661033dfd7291ea1caa3fe882662f65eb2ae3f3b (1.12 ± 0.04 times faster)

this one is probably because the benchmark don't use a lot of variable. I guess perf get worse as we get more variables
2023-12-07 - 24157817   - 9 seconds  - 4fb81204da5895d3e6a3e783e0c10265e8dcbf08 (1.14 ± 0.02 times faster)
2023-12-07 - 24157817   - 7 seconds  - 623c2c21f283d67a30445205c5c9db793175474a (1.28 ± 0.04 times faster)
2023-12-07 - 24157817   - 6 seconds  - ed8d720a1c670e18a5feb758676cc5ba5567e0d6 (1.11 ± 0.02 times faster)
2023-12-12 - 24157817   - 5 seconds  - 1b111ba5dd31f0a2f3f4d46a6da1e30eef89da66 (1.08 ± 0.03 times faster)
2023-12-23 - 24157817   - 5 seconds  - 27e2da04038c54023bbd971af1e5e81ca4b5e47b (1.04 ± 0.04 times faster)
```
