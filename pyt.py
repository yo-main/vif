import time


def fib(n):
    if n <= 1:
        return n
    return fib(n-2) + fib(n-1)


i = 0
while True:
    start = time.time()
    value = fib(i)
    duration = round(time.time() - start)

    print("Got", value, "after", duration, "seconds")

    i = i + 1



