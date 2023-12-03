import time


def fib(n):
    if n <= 1:
        return n
    return fib(n-2) + fib(n-1)


i = 0
while True:
    start = time.time()
    value = fib(i)
    duration = (time.time() - start) / 1000000

    print(value)

    i = i + 1



