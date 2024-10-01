% id: 15
% title: Concurrent data retrieval 🤹‍♂️

Data retrieval is often input/output (I/O) bound, which means that the speed at which one can retrieve data is often bound to the speed of the system providing the data. Sending a request to a service that provides data doesn't require a lot of resources on the client side, but might require significantly more resources on the service side, as it needs to get data from a database, for example. The request itself also needs time to travel from the client to the service.

Up until now I have created a number of data connectors that pull data from other systems into a data lake, where it can be analysed. Recently I created one that gets to the I/O bound of the subsystem it is pulling data from. This was only possible with concurrency.

I'm building these connectors in Python, so I started reading the docs on [`asyncio`](https://docs.python.org/3/library/asyncio.html), which is a library used to as a foundation for other asynchronous frameworks. It uses the `async` and `await` syntax to submit tasks to the [event loop](https://docs.python.org/3/library/asyncio-eventloop.html), which runs on a single thread. That's right, async code runs concurrently and not in parallel, which can be confusing at first but it's important to [understand the difference](https://stackoverflow.com/questions/1050222/what-is-the-difference-between-concurrency-and-parallelism).

## Event loop

In the context of computer science, the event loop is a design pattern that waits for events and dispatches events. The event loop makes a request to some event provider and calls the appropriate handler. This request can be seen as a process that is waiting until an event has arrived. When the request is waiting, the request is [blocked](<https://en.wikipedia.org/wiki/Blocking_(computing)>). An event in this context is a trigger which can be triggered by user interactivity like a mouse click or key press, or by another process that triggers programatically.

<pre class="mermaid">
  
</pre>
