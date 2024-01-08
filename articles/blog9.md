% id: 9
% title: Parallelizing workloads with Celery and RabbitMQ on Kubernetes 🍃🐇☸️
% date: 2023-07-19
% tags: cloud, distributed

Recently I have been working on a project involving a lot of IO to process a vast amount of documents in a relatively short time. After writing the application that contained the business logic, I had to find a way to parallelize the workload to make it scalable. Since my team already hosts most software on Kubernetes, running it in several "worker containers" made sense. At first I followed [some examples with native Kubernetes object](https://kubernetes.io/docs/tasks/job/parallel-processing-expansion/), which worked well but missed some important features. For example, I would have to build retry logic to in case a `Job` would fail for some reason. Also, I would have to deploy a key-value database that would contain the queue and build CRUD-like operations to let the `Jobs` interact with the queue. Even though this seemed like a fun projects, I started looking for alternative solutions, and that is when I a colleague pointed me towards [Celery](https://docs.celeryq.dev/en/stable/getting-started/introduction.html).

Celery met my requirements and counters the issues I had with the solution that involved solely K8s native objects. Quoted from the docs:

## Highly available

Workers and clients will automatically retry in the event of connection loss or failure, and some brokers support HA in way of Primary/Primary or Primary/Replica replication.

## Fast

A single Celery process can process millions of tasks a minute, with sub-millisecond round-trip latency (using RabbitMQ, librabbitmq, and optimized settings).

## Flexible

Almost every part of Celery can be extended or used on its own, Custom pool implementations, serializers, compression schemes, logging, schedulers, consumers, producers, broker transports, and much more.

With Celery, there are still some decisions that have to be made based on the use case. One of the first things is choosing a message broker and a result backend. The message broker sends messages from the Celery application to the workers. For this, I relied on RabbitMQ, which is also the default broker. The [RabbitMQ cluster operator](https://www.rabbitmq.com/kubernetes/operator/operator-overview.html) makes is easy to deploy, manage and operate a RabbitMQ cluster, so that is what I used after going through the Celery documentation with a local RabbitMQ service. Testing a local Celery application with a distributed broker is still possible by port-forwarding the service of the RabbitMQ cluster operator. I always like this approach because it allows me to get my application from running locally to running on a cluster in phases. Having the message broker set up in a distributed fashion makes that part of the application very scalable and fault tolerant, as data is replicated on several nodes so data loss can be prevented.

To run workloads on Celery workers, Python code needs to be wrapped in a `task`. The `task` is linked to a `Celery` instance that gets the entrypoint of the RabbitMQ cluster as an argument. In the example below, the `broker` URL depends on where the Celery workers and the message broker are running. When the complete application is deployed, this should be the internal Kubernetes DNS record of the RabbitMQ service.

```py
from celery import Celery

app = Celery("tasks", broker="pyamqp://guest@localhost//")

@app.task
def add(x, y):
    return x + y
```

You can already run this example Celery application (saved as `tasks.py`) locally with a simple command `celery -A tasks worker`. This means that it is also not very complicated to wrap this application in a Docker container, as it just needs the workload dependencies (think of the workload as the business logic that needs to be performed), the Celery python package and the `RUN` command to fire up the worker.

```dockerfile
FROM python:3.10
WORKDIR /app
COPY ./requirements.txt /app/requirements.txt
RUN pip install --no-cache-dir --upgrade -r /app/requirements.txt
COPY src src
WORKDIR src/celery
CMD celery -A tasks worker --loglevel=INFO --concurrency=1
```

The worker image can be deployed as a replicaset on Kubernetes, where the number of replicas can scale when the workload increases. This means that both the broker nodes and worker nodes are horizontal scalable, as displayed in the high-level diagram showed below.

![Diagram](assets/images/k8s_celery_scaling.svg)

I left the results backend out of scope for the first iteration, but according to the Celery docs, [Redis](https://redis.io/) (which is also horizontal scalable) is a popular choice to complement RabbitMQ. If something more persistent is required, a Postgres database is also an option. I also left monitoring out of scope, but [Flower 🌸](https://flower.readthedocs.io/en/latest/features.html) seems like an amazing tool to monitor Celery events in real-time. Monitoring is still possible through the [RabbitMQ management API](https://www.rabbitmq.com/management.html) which is exposed on port `15672` by default. In fact, Flower consumes data from the management API to show information about the workers.

This basic setup was sufficient to get a feeling of its scaling potential and serves as a good starting point for further iterations.
