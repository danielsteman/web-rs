% id: 15
% title: Concurrent data retrieval

In data intensive applications many processes are [I/O bound](https://en.wikipedia.org/wiki/I/O_bound), meaning that their speed is limited not by CPU processing power but by the time spent waiting for I/O operations to complete, such as network requests or file reads. For example, when a client application requests data from a server, the request is sent, and while the server processes and sends back a response, the client is essentially waiting, or "idle." This waiting time can add up, especially when multiple requests are needed to retrieve large datasets.

Since most applications I'm currently working on are written in Python, the examples throughout this post use [`asyncio`](https://docs.python.org/3/library/asyncio.html), the implementation of coroutines in the Python ecosystem. It uses the `async` and `await` syntax to submit tasks to the [event loop](https://docs.python.org/3/library/asyncio-eventloop.html), which runs on a single thread. That's right, async Python code runs concurrently and not in parallel, which can be confusing at first but it's important to [understand the difference](https://stackoverflow.com/questions/1050222/what-is-the-difference-between-concurrency-and-parallelism). Before showing an example of concurrent data retrieval, let's go back to the origin.

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

def square() -> Generator:
    while True:
        x = yield
        if x is not None:
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

def wrapper(gen: Generator) -> Generator:
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
def wrapper(gen: Generator) -> Generator:
    yield from gen
```

Like in the previous `wrapper` declaration, the `yield from` gives control to the sub-generator. The second `wrapper` declaration will yield the same outcome as the previous function, but with more concise code.

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

You might be asking yourself how the event loop knows when an IO operation has been completed. For that, `asyncio` leverages the [select](https://docs.python.org/3/library/select.html) module which is an interface to the Unix [system calls](https://man7.org/linux/man-pages/man2/select.2.html) that are used to examine that status of a [file descriptor](https://en.wikipedia.org/wiki/File_descriptor) of I/O channels. What does this mean? A file descriptor is like a receipt that a program receives when it opens a file or connection. The receipt can be used to read, write or close the associated file or connection. Unix system calls such as `select`, `poll` and `epoll` are indirectly used by `asyncio` to determine if a task is blocked.

## Concurrency in a data retrieval context

Let's assume that we have an application that wants to requests many pages of data from another application. Often times we know how many pages of data we need to request in advance, because this information is returned with the first request. To reduce idle time of our single thread, we can create a list of tasks (requests), where we create one task for each page of data, and push this to the event loop.

```python
from typing import Any
import httpx
import asyncio

async def get_items(client: httpx.AsyncClient, page: int = 0, page_size: int = 10) -> dict[str, Any]:
    r = await client.get(f"https://www.data_provider.com/api/items", params={"page": page, "pageSize": page_size})
    r.raise_for_status()
    print(f"Fetched items {page * page_size} - {(page + 1) * page_size}")  # show range of items
    return r.json()

async def fetch_all_items(client: httpx.AsyncClient, total_pages: int, page_size: int = 10) -> list[dict[str, Any]]:
    # create list of tasks
    tasks = [
        get_items(client, page=page, page_size=page_size)
        for page in range(total_pages)
    ]

    # push tasks to event loop
    try:
        results = await asyncio.gather(*tasks)
    except httpx.HTTPStatusError as e:
        print("Error fetching items: {e}")  # that's right, asyncio will propagate exceptions

    # flatten list of lists of items
    all_items = [item for result in results for item in result.get("items", [])]
    return all_items
```

In this example, a task would be a `get_items` call. For demonstration purposes, I printed the range of items that was retrieved. Since the tasks are ran concurrently, the event loop will determine when to perform which task, making the order of the tasks nondeterministic.

```python
async def main():
    async with httpx.AsyncClient() as client:
        items = await fetch_all_items(client, 100)

if __name__ == "__main__":
    asyncio.run(main())
...
Fetched items 0 - 10
Fetched items 20 - 30
Fetched items 10 - 20
Fetched items 30 - 40
```

The next time `fetch_all_items` is called, the printed output might look different:

```python
...
Fetched items 20 - 30
Fetched items 0 - 10
Fetched items 30 - 40
Fetched items 10 - 20
```

When dealing with concurrent data fetching applications, you will probably run into several problems like, like overloading the server or race conditions. Some of these problems can be solved with [synchronization](https://docs.python.org/3/library/asyncio-sync.html), which I will write about in more detail in a next post.
