% id: 15
% title: A disposable data layer
% date: 2024-06-01
% tags: devops, data platform

## Sources

Since a while I've been using Databricks as a data lake, a centralized place to store and process data. Through a number of ingestion pipelines, data is inserted into the data lake. One example of a source is a database, that is being filled with data by backend services when certain events happen. For example a user service that handles new users signin up, existing users logging in, or users changing their details. Everytime when an event like that occurs, the database state changes. Another source is a SaaS (software-as-a-service) system that exposes data through an API (application programming interface). The API serves as an extra layer between the consumer (the data pipeline) and the database that underlies this SaaS. This makes sense because the SaaS wouldn't want anyone accessing their database directly. At last, there may be a data bump on an object storage like [S3](https://aws.amazon.com/s3/) or [Azure Blob Storage](https://azure.microsoft.com/en-us/products/storage/blobs). Usually this is static data from a system that is deprecated and offline.

<pre class="mermaid">
    flowchart LR
        style note opacity:0
        style note fontSize: 10px

        database --> uc[unity catalog*]
        SaaS --> uc
        sd[static dump] --> uc
        uc --> analytics
        uc --> ds[data science projects]

        subgraph sources
            database
            SaaS
            sd
        end

        subgraph dl[data lake]
            uc
            analytics
            ds
        end
        
        subgraph note [" "]
            uc -.- ucnote[*tables in databricks
            are governed by 'unity catalog', 
            so whenever you read 'unity catalog', 
            just think of a collection of tables]
        end
</pre>

The flow of data (lineage) is very straight forward and there is a clear separation of concerns where the data lake (Databricks) only reads from production systems.
