% id: 15
% title: Concurrent data retrieval 🤹‍♂️

Data retrieval is often input/output (I/O) bound, which means that the speed at which one can retrieve data is often bound to the speed of the system providing the data. Sending a request to a service that provides data doesn't require a lot of resources on the client side, but might require significantly more resources on the service side, as it needs to get data from a database, for example. The request itself also needs time to travel from the client to the service.

Up until now I have created a number of data connectors that pull data from other systems into a data lake, where it can be analysed. Recently I took on the challenge to create a connector that gets to the I/O bound. This was only possible with concurrency.

<pre class="mermaid">
  
</pre>
