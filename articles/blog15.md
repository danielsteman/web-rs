% id: 15
% title: A disposable data layer
% date: 2024-06-01
% tags: devops, data platform

## Sources

Since a while I've been using Databricks as a data lake, a centralized place to store and process data. Through a number of ingestion pipelines, data is inserted into the data lake. One example of a source is an operational database, that is continuously being filled with data by backend services. Another source is a SaaS (software-as-a-service) system that exposes data through an API (application programming interface).

<pre class="mermaid">
    graph TD;
        A-->B;
        A-->C;
        B-->D;
        C-->D;
</pre>
