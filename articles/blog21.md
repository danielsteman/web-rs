# A new class of interface consumers 🍝

## Interfaces

In computing, an interface is the shared boundary of two or more components in a system where information is shared. These components can be hardware, like a motherboard, disk and memory that work together to perform the tasks that you give to your computer. They can also be software components, like a database, REST (Representational State Transfer) API (application programming interface) and a web client. In that example, the web client sends a request to the API to fetch certain data from the database. The API receives the requests, fetches the requested data from the database and responds to the client with the requested data. In this example, the API is the boundary that is shared by the client and the API where information is exchanged. The example I sketched here is the [client server model](https://en.wikipedia.org/wiki/Client%E2%80%93server_model) that has been around since the 70s and is still widely used. Even modern architectures like [microservices](https://en.wikipedia.org/wiki/Microservices) are built on top of distributed client server principles. Another example is a [graphical user interface (GUI)](https://en.wikipedia.org/wiki/Graphical_user_interface) that is used by humans to navigate in an application, such as your browser. The browser is the boundary that is shared by you (human, presumably) and websites that you visit. In the past couple of years, I've been building interfaces for applications, mostly frontends, but recently I've been building a new type of interface according to the model context protocol (MCP) for a new type of consumer, AI agents. After of couple of months of building and taking a couple of steps back, I wanted to write about the differences between these interfaces and how MCP builders need to drop the principles that apply to REST APIs (interfaces for other applications) and shift their mental model to build for a new class of consumers [AI agents].

## Before AI

If you every build a web application that had interaction with some kind of data store, you probably wrote API services that would dispatch a request at a server to (hopefully) receive data in a response that you were able to render. In the process of building your web application, you can't really guess where you have to dispatch that request and what the request should look like; you need documentation. Since the introduction of [OpenAPI](https://en.wikipedia.org/wiki/OpenAPI_Specification), many APIs ship with standardized documentation
making it convenient for application builders that rely on a particular API. The web application builder would read the spec, either using human cognitive functions or programmatically, and they would know exactly what their API calling code should look like. If the API (server code) wouldn't change, the OpenAPI spec wouldn't change and their would be no reason for the client code to change.

<pre class="mermaid">
  flowchart TD
    subgraph REST
        A1[server] --> A2[OpenAPI spec]
        A1 --> A3[client code]
        A2 -- "consumed at build time" --> A3
        A1 -- deterministic --> A3
    end

    subgraph MCP
        B1[server] --> B2[Tool metadata]
        B1 --> B3[LLM]
        B2 -- "consumed at inference time (every tool call)" --> B3
        B1 -- probabilistic --> B3
    end
</pre>
