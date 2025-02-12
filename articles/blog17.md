% id: 17
% title: Tail Recursion 🔁
% date: 2025-02-11
% tags: algorithms

If a function makes a tail-recursive call, there should be no more computations in that function. The recursive call should be the last operation that the function performs.

```js
function factorial(n):
    if n <= 1:
        return 1
    else:
        return n * factorial(n - 1)
```

This example is not tail-recursive, because the recursive call is not the last operation that the function `factorial` performs, because the result of the recursive call still needs to be multiplied with `n`. As a result, each recursive function call will be pushed to the call stack, after which the stack will be resolved in reversed order.

```js
factorial_tr(5);
factorial_tr(4);
factorial_tr(3);
factorial_tr(2);
factorial_tr(1);
```

So `factorial_tr(5)` is pushed to the stack first, and `factorial_tr(1)` last. Then, `factorial_tr(1)` is resolved first and `factorial_tr(5)` last. This means that the maximum size of the call stack is equal to the number of nested recursive calls.

We can make the function tail recursive by adding an accumulator that holds the intermediate result of the recursive call.

```js
function factorial_tr(n, accumulator):
    if n <= 1:
        return accumulator
    else:
        return factorial_tr(n - 1, n * accumulator)
```

The compiler that optimizes tail calls can keep reusing the same stack frame (the portion of the stack allocated to the function call) because it becomes an iterative process.

```js
factorial_tr(5, 1);
factorial_tr(4, 5);
factorial_tr(3, 20);
factorial_tr(2, 60);
factorial_tr(1, 120);
```

So first, `factorial_tr(5, 1)` is pushed to the call stack. Then `factorial_tr(5, 1)` is popped and resolved, emptying the call stack. Then `factorial_tr(4, 5)` is pushed to the call stack and popped and resolved, etcetera. In this case, the maximum size of the call stack is equal to one function call.

Unfortunately, not all compilers perform tail call optimization. For example, Python will always yeet all recursive calls on the call stack. [Rust and C do eliminate tail calls](https://stackoverflow.com/questions/59257543/when-is-tail-recursion-guaranteed-in-rust), but not guarenteed. As mentioned in the issue, you should not rely on this optimization when it is important to eliminate tail calls <i>for sure</i>. Instead, you should take an iterative approach.

```python
def factorial_iter(n):
    result = 1
    for i in range(1, n + 1):
        result *= i
    return result
```
