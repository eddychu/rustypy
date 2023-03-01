def fib(x, y):
    if x < 2:
        if y < 3:
            return x + y
    return fib(x-1) + fib(x-2)
