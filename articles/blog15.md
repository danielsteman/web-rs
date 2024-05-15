% id: 15
% title: A disposable data layer
% date: 2024-06-01
% tags: devops, data platform

## Sources

Since a while I've been using Databricks as a data lake, a centralized place to store and process data. Through a number of ingestion pipelines, data is inserted into the data lake. One example of a source is a database, that is being filled with data by backend services when certain events happen. For example a user service that handles new users signin up, existing users logging in, or users changing their details. Everytime when an event like that occurs, the database state changes. Another source is a SaaS (software-as-a-service) system that exposes data through an API (application programming interface). The API serves as an extra layer between the consumer (the data pipeline) and the database that underlies this SaaS. This makes sense because the SaaS wouldn't want anyone accessing their database directly.

APIs can come in many forms: an API with endpoints that can be called and return some data object.

<pre class="mermaid">
    graph TD;
        A-->B;
        A-->C;
        B-->D;
        C-->D;
</pre>
