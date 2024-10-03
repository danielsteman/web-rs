% id: 15
% title: Concurrent data retrieval

Data retrieval is often input/output (I/O) bound, which means that the speed at which one can retrieve data is often bound to the speed of the system providing the data. Sending a request to a service that provides data doesn't require a lot of resources on the client side, but might require significantly more resources on the service side, as it needs to get data from a database, for example. The request itself also needs time to travel from the client to the service.

Up until now I have created a number of data connectors that pull data from other systems into a data lake, where it can be analysed. Recently I created one that gets to the I/O bound of the subsystem it is pulling data from. This was only possible with concurrency.

I'm building these connectors in Python, so I started reading the docs on [`asyncio`](https://docs.python.org/3/library/asyncio.html), which is the implementation of coroutines in the Python ecosystem. It uses the `async` and `await` syntax to submit tasks to the [event loop](https://docs.python.org/3/library/asyncio-eventloop.html), which runs on a single thread. That's right, async code runs concurrently and not in parallel, which can be confusing at first but it's important to [understand the difference](https://stackoverflow.com/questions/1050222/what-is-the-difference-between-concurrency-and-parallelism). There are also a couple of other concepts that help to understand coroutines in Python.

## Generators

A list of values that can be iterated upon, is called an iterable. You can traverse through its values and you can count the number of values. A generator is a kind of iterator, but is instead of having all the values available up front, each value is [lazily evaluated](https://en.wikipedia.org/wiki/Lazy_evaluation). This means that at each iteration, the value is yielded instead of returned.

## Coroutines

Coroutines are tasks, units of asynchronous work that the event loop manages. A nice example of a coroutine is this function that makes an API call to get `items`:

```python
async def get_items(client: httpx.AsyncClient, from: int = 0) -> dict[Any, Any]:
    r = await client.get(f"https://www.data_provider.com/api/items?from={from}")
    return r.json()
```

Let's assume that we there are millions of items and that they are paginated in pages of 50 items. When we run this function, we will mostly wait for the server to return data, so this function is I/O bound. With a synchronous approach, we would
make a request, wait for data, make the next request, wait for data, et cetera. I would be much more efficient to do something while we are waiting for data to be returned, perhaps already make the next request, in an asynchronous fashion.

## Event loop

The diagram shows how the event loop is the orchestrator of coroutines and it "waits efficiently" on I/O operations.

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

This diagram shows that a running event loop is
