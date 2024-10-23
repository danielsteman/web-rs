% id: 15
% title: Concurrent data retrieval

Data retrieval is often input/output [(I/O) bound](https://en.wikipedia.org/wiki/I/O_bound), which means that the speed at which one can retrieve data is often bound to the speed of the system providing the data. Sending a request to a service that provides data doesn't require a lot of resources on the client side, but might require significantly more resources on the service side, as it needs to get data from a database, for example. The request itself also needs time to travel from the client to the service.

Up until now I have created a number of data connectors that pull data from other systems into a data lake, where it can be analysed. Recently I created one that gets to the I/O bound of the subsystem it is pulling data from. This was only possible with concurrency.

I'm building these connectors in Python, so I started reading the docs on [`asyncio`](https://docs.python.org/3/library/asyncio.html), which is the implementation of coroutines in the Python ecosystem. It uses the `async` and `await` syntax to submit tasks to the [event loop](https://docs.python.org/3/library/asyncio-eventloop.html), which runs on a single thread. That's right, async Python code runs concurrently and not in parallel, which can be confusing at first but it's important to [understand the difference](https://stackoverflow.com/questions/1050222/what-is-the-difference-between-concurrency-and-parallelism). Before showing an example of concurrent data retrieval, let's go back to the origin.

## Iterators

A prerequisite for understanding async functions, is an understanding of iterators. An iterator is an object that provides access to an element of a collection and can change its internal state to provide access to the next element (traversal). In Python, an iterator implements the special methods `__next__()` and `__iter__()`. Now, you might think of a list, but that is actually an <i>iterable</i> because it only has the `__iter__()` method, which produces an <i>iterator</i> when it is called. You can get an iterator from a list and call `next()` until the iterator raises `StopIteration`.

```py
lst = [1,2,3]
iterator = iter(lst)
print(next(iterator))  # Output: 1
print(next(iterator))  # Output: 2
print(next(iterator))  # Output: 3
print(next(iterator))
...
StopIteration
```

## Generators

A generator is a kind of iterator, but instead of having all the values available up front, each value is [lazily evaluated](https://en.wikipedia.org/wiki/Lazy_evaluation). In Python terms, a value is yielded at each iteration. Also, a value from a generator can only be yielded once, since the generator only knows its current state each time it's yielding a value. This is why it can be memory-efficient to use a generator instead of a list.

```py
def n_generator(n):
    for i in range(n):
        yield i

generator = n_generator(3)

for i in generator:
    print(i)
...
0
1
2

for i in generator:
    print(i)
...
```

The second time we try to print out items from the generator, they're gone. It is also possible to `send` values to a generator.

```py
from typing import Generator

def square() -> Generator[float, float | None, None]:
    while True:
        x = yield
        yield x ** 2

squarer = square()
squarer.send(2)
...
TypeError: can't send non-None value to a just-started generator
```

Interestingly, when we try to send a value to the generator, a `TypeError` is being raised. As pointed out in [PEP 342](https://peps.python.org/pep-0342/), generators begin execution at the top of the function body, meaning that there is no `yield` that can receive a value that is not `None`. This is why we first need to call `next` with the generator as argument. [David Beazley](https://dabeaz.com/coroutines/Coroutines.pdf) called this "priming", which covers it well in my opinion, since this operation is necessary before using the generator.

```py
squarer = square()
next(squarer)
squarer.send(2)
...
4

next(squarer)  # continue to the next x = yield
squarer.send(3)
...
9

squarer.send(None)  # we can also 'prime' the generator like this
squarer.send(4)
...
16
```

Now that we understand how to send data to a generator, let's examine what would happen when the generator is wrapped in another function. This is where `yield from` comes into play. This Python feature is well described in [this Stack Overflow issue](https://stackoverflow.com/questions/9708902/in-practice-what-are-the-main-uses-for-the-yield-from-syntax-in-python-3-3). The following wrapper function is copied from the issue, complemented with type annotations which were introduced for generators in [PEP 484](https://peps.python.org/pep-0484/).

```py
from typing import Generator

def wrapper(gen: Generator[float, float | None, None]) -> None:
    next(gen)
    while True:
        try:
            x = yield
            gen.send(x)
        except StopIteration:
            pass

squarer = square()
w = wrapper(squarer)
w.send(None)
w.send(2)
...
4
```

The wrapper primes the generator (`gen`) by implicitly calling `__next__` with `next(gen)`. It needs to handle `StopIteration` because this is raised from within `gen` when the generator is finished ([PEP 255](https://peps.python.org/pep-0255/)👴). All this logic can be refactored into something much more elegant:

```py
def wrapper(gen: Generator[float, float | None, None]) -> None:
    yield from gen
```

This will yield the same outcome as the previous function, but with more concise code.

## Coroutines

Coroutines are program components that can be paused and resumed. Since Python 3.4 it's possible to create a coroutine using the `asyncio` standard library and its `@asyncio.coroutine` decorator in combination with the `yield from` syntax above, as code execution in the `wrapper` is momentarily paused in favor of the code execution in the sub generator. Since `yield from` is now used used for two different purposes: to transfer control from one generator to a sub generator bidirectionally and to create sub routines. That is why the more intuitive and not-conflicting `await` syntax was introduced in Python 3.5.

```py
import asyncio

async def square(x: float) -> float:
    return x ** 2

async def wrapper(x: float) -> float:
    return await square(x)

asyncio.run(wrapper(2))
...
4
```

You might have noticed that the wrapper function is now called using the `asyncio.run` function. This is needed to execute code on the event loop.

## Event loop

An event loop is an orchestrator of coroutines and can pause and resume code efficiently, given that the code is I/O-bound. `asyncio` is one of Python's builtin implementations to achieve concurrency through coroutines and asynchronous programming. The diagram shows how the event loop "waits efficiently" on I/O operations.

<pre class="mermaid">
  sequenceDiagram
    participant EL as "Event Loop"
    participant C1 as "Coroutine 1"
    participant C2 as "Coroutine 2"
    participant IO as "IO Operation"

    EL->>+C1: Start Coroutine 1
    C1-->>EL: Await IO Operation
    EL->>+C2: Start Coroutine 2
    C2-->>EL: Await IO Operation
    EL->>IO: Monitor IO Events
    IO-->>EL: IO Operation Complete
    EL->>+C1: Resume Coroutine 1
    C1-->>EL: Complete
    EL->>+C2: Resume Coroutine 2
    C2-->>EL: Complete
</pre>

You might be asking yourself how the event loop knows when an IO operation has been completed. For that, `asyncio` leverages [select](https://docs.python.org/3/library/select.html) which is an interface to the Unix [`select()` system call](https://man7.org/linux/man-pages/man2/select.2.html). This system call allows a program to monitor processes and waits until one or more processes are completed.

## Concurrency in a data retrieval context

A nice example of a coroutine is this function that makes an API call to get `items`:

```python
async def get_items(client: httpx.AsyncClient, from: int = 0) -> dict[Any, Any]:
    r = await client.get(f"https://www.data_provider.com/api/items?from={from}")
    return r.json()
```

Let's assume that we there are millions of items and that they are paginated in pages of 50 items. When we run this function, we will mostly wait for the server to return data, so this function is I/O bound. With a synchronous approach, we would
make a request, wait for data, make the next request, wait for data, et cetera. It would be much more efficient to do something else while waiting for data to be returned, such as sending off the next request.
