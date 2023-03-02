def fib(x):
    if x < 2:
        return x
    return fib(x-1) + fib(x-2)


fib(7)
