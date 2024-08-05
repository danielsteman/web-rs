## Data platform

Since a while I've been using [Databricks](https://www.databricks.com) to build a data lake, which is a fancy word for a centralized place where data is stored and processed, to make it useful for individuals or applications that look for insights about the business. The advantage of performing the whole process (going from raw data to insights) is that it scales well. The same cloud resources, like a compute unit that does calculations (like running queries), can be shared among several users. Also, it's easier to prevent duplication when you have a central data lake, since logic built by others is much more discoverable.

Databricks performs many tasks that would otherwise be performed by separate tools and applications. In an alternative stack, you might use [dbt](https://www.getdbt.com/) for transformations, [Airflow](https://airflow.apache.org/) for data pipeline orchestration and [Kubernetes](https://kubernetes.io/) as a compute platform. Databricks provides solutions for data transformation, orchestration, compute and more, which makes the aforementioned technologies less attractive if your goal is to maximise business value with limited engineering capacity. Since Databricks is fully managed you will lose configurability and thus flexibility and room to optimize. This makes it not the best choice for every team, but I believe it's a good solution for many teams, especially if the company still needs to become _data driven_.

## Sources

Through a number of ingestion pipelines, data is inserted into the data lake. One example of a source is a database, that is being filled with data by backend services when certain events happen. For example a user service that handles new users signin up, existing users logging in, or users changing their details. Everytime when an event like that occurs, the database state changes. Another source is a SaaS (software-as-a-service) system that exposes data through an API (application programming interface). The API serves as an extra layer between the consumer (the data pipeline) and the database that underlies this SaaS. This makes sense because the SaaS wouldn't want anyone accessing their database directly. At last, there may be a data bump on an object storage like [S3](https://aws.amazon.com/s3/) or [Azure Blob Storage](https://azure.microsoft.com/en-us/products/storage/blobs). Usually this is static data from a system that is deprecated and offline.

<pre class="mermaid">
    graph LR
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
            just think of it as a collection of tables]
        end
</pre>

## Lineage

The flow of data (lineage) is very straight forward and there is a clear separation of concerns where the data lake (Databricks) only reads from production systems (sources). With this setup, Databricks can be swapped with a different data lake solution, such as [Snowflake](https://www.snowflake.com/en/). Being able to swap out technologies within your ecosystem makes you flexible and less dependent on a tool that you might outgrow at some point.

## Infrastructure as code

Cloud resources can be managed in the portal of your cloud provider (the [ClickOps](https://aws.amazon.com/console/) approach). A much more scalable approach is to manage infrastructure is through [infrastructure as code (IaC)](https://www.redhat.com/en/topics/automation/what-is-infrastructure-as-code-iac). Within the IaC paradigm, there are several tools that you can use, both cloud provider specific, like [Cloud Formation](https://aws.amazon.com/cloudformation/) or [Azure Resource Manager](https://learn.microsoft.com/en-us/azure/azure-resource-manager/management/overview) or cloud agnostic, like [Terraform](https://www.terraform.io/) or [Pulumi](https://www.pulumi.com/). What I personally like about the latter is that you can use it for several cloud providers, so your knowledge is very transferrable to stacks built on other cloud providers. I happened to work in a team that already adopted Terraform so I continued to use it to provision a data platform. 

Aside from making the platform managable and maintainable, the goal of setting up the platform as code is to make it disposable. Why would you want that? An obvious argument would be disaster recovery. If production resources, such as a data pipeline, is constantly changing but it's state is never tracked and stored, and this resource gets deleted for some reason, it won't be possible to recover. 