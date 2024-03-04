% id: 6
% title: Integrating a ML model in an API 🔀
% date: 2023-01-23
% tags: ml

In my [previous post](https://www.danielsteman.com/blog/5) I wrote about deploying a ML model using the Seldon Core library. At the end of the post we ended up with a `SeldonDeployment` kubernetes object which roughly resembles a standard `Deployment` object, but with some extras. When deployed, you'll end up with a micro service that runs in a pod and a service that exposes the pod.

For a project I was working on, we needed more than just the service, we also needed to store predictions and apply some business logic. Hence, I built a wrapping API and giving our Python-focused stack, my weapon of choice was [FastAPI](https://fastapi.tiangolo.com/). I inspired the design on [an example](https://github.com/tiangolo/full-stack-fastapi-postgresql) that was built by [the maker](https://github.com/tiangolo) of FastAPI, so I would advice you to check that out as well. A very high level design is shown below.

![API design high level overview](/assets/images/api_design.png)

Before, a client calls the ML micro service directly with a post request and a body, where the body contains the data that will be used to predict. An example `cURL` request looks like the snippet below.

```bash
curl -X POST \
  -H 'Content-Type: application/json' \
  -d '{"ndarray":{"data":[[1,3,3]]}}' \
  http://application.cluster-domain:9000/api/v1.0/predictions
```

The body, `[1,3,3]`, is the input `X`, an argument of the `predict` method of a trained [Scikit Learn](https://scikit-learn.org/stable/) model (or model trained with another library, which might have a different standard method).
In this scenario, data is not persisted, as the prediction is returned immediately.

With an API in between, we can do anything we'd like with the incoming requests (from the client) and incoming responses (from the ML micro service), such as writing data to a database for later use. Prediction data is relevant for performance measurements, as a historical set of predictions can be compared to actual historical data points. This could be used to trigger retraining of the ML model, as described more in depth in this [MLOps article of Google](https://cloud.google.com/architecture/mlops-continuous-delivery-and-automation-pipelines-in-machine-learning). Data sent by the client is relevant because it accumulates to the next training set. Also, new data may have a different distribution compared to the old data that was used to initially train the model. Without storing data it would be impossible to know and it can affect model performance badly. This [blog post](https://superwise.ai/blog/data-drift-detection-basics/) about data drift explains the concept in some more detail.

Enough about [MLOps things](https://ml-ops.org/). The tree below shows how I layered the application according to the tasks each component is fulfilling.

```bash
.
├── alembic
│   ├── env.py
│   └── versions
│       └── create_tables.py
├── app
│   ├── __init__.py
│   ├── crud
│   │   ├── __init__.py
│   │   ├── base.py
│   │   └── classification.py
│   ├── db
│   │   ├── __init__.py
│   │   ├── base_class.py
│   │   └── session.py
│   ├── main.py
│   ├── ml
│   │   ├── __init__.py
│   │   ├── base.py
│   │   └── classifier.py
│   ├── models
│   │   ├── __init__.py
│   │   └── classification.py
│   ├── routers
│   │   ├── __init__.py
│   │   └── classification.py
│   └── schemas
│       ├── __init__.py
│       └── classification.py
├── deploy
└── tests
```

## Tables

Let's start with `alembic`. [This](https://alembic.sqlalchemy.org/en/latest/) is the Python tool I'm using to migrate tables to the Postgres DB. It's easy to use, as it leverages the well known object-relational mapping (ORM) library `sqlalchemy` for declaring tables. I prefer setting up tables this way because the configuration will be kept in version control and is easy to reproduce in a CI/CD pipeline.

## Models

Each database table is also declared by a class with a `Base` parent class. To construct the `Base`, the [declarative API](https://docs.sqlalchemy.org/en/13/orm/extensions/declarative/api.html#sqlalchemy.ext.declarative.declared_attr) of `sqlalchemy` is used to map model class names to table names, for example:

```py
from typing import Any
from sqlalchemy.orm import as_declarative, declared_attr


@as_declarative()
class Base(object):
    __name__: str

    @declared_attr
    def __tablename__(cls) -> str:
        return cls.__name__.lower()
```

## Schemas

These are `pydantic` classes that are used to validate requests and responses, by enforcing type hints. This way we can be sure that the expected features are sent to the ML model and that the ML model gives back an expected response.

## Routers

Endpoints can be declared directly in `app/main.py` but I always find it more convenient to group endpoints in `Router`s and use the `include_router` method to attach them to the `App` object. In the below example, the `router` is not encapusulated in a separate file for demostration but should be in a real project.

```py
from fastapi import FastAPI, APIRouter

router = APIRouter(prefix="/classification")

app = FastAPI()
app.include_router(router)
```

From the router, we can inject the ML model such that we can fetch predictions when we receive a request with features. The `app.ml.classifier.Classifier` model class is responsible for sending requests to the ML micro service, hence it should be [injected as a dependency](https://fastapi.tiangolo.com/tutorial/dependencies/).

```py
from fastapi import Depends

@router.post("/classify/")
def create_prediction(
    clf=Depends(Classifier),
    X: List[Any],
) -> Any:
    return clf.predict(X)
```

The `Classifier` makes requests with the `requests` module, and because we are calling an interal Kubernetes service, the domain of the ML micro service is formatted like `my-svc.my-namespace.svc.cluster-domain.example`. You can find more info about the Kubernetes DNS [here](https://kubernetes.io/docs/concepts/services-networking/dns-pod-service/).

In the above exmaple we return the prediction immediately, but that's not neccesary. Instead, we can implement logic in between receiving the request (and its payload as argument `X`) and returning something. For example, we can insert the prediction into the database.

```py
from fastapi import Depends
from app import crud
from app.db.session import get_db
from app import models

@router.post("/classify/")
def create_prediction(
    clf=Depends(Classifier),
    database_connection=Depends(get_db),
    X: List[Any],
) -> Any:
    prediction = clf.predict(X)
    database_object = Classification(
        data=X,
        prediction=prediction,
    )
    crud.classification.create(database_connection, database_object)
    return clf.predict(X)
```

And just like that, through a [CRUD](https://www.freecodecamp.org/news/crud-operations-crud-definition-in-programming/) method, it's possible to write input and output of the ML microservice to a database for later (or instant) use. This example is very lean and doesn't show the definition of the CRUD method. If you'd like to see an example of that, please check out [this project builder](https://github.com/tiangolo/full-stack-fastapi-postgresql).

## Deployment

Instead of using a `SeldonDeployment`, as explained in the [previous blog](https://www.danielsteman.com/blog/5), we use a regular `Deployment` [Kubernetes object](https://kubernetes.io/docs/concepts/workloads/controllers/deployment/) to deploy this API. Obviously, the application needs to be packaged in a Docker container first. [The docs](https://fastapi.tiangolo.com/deployment/docker/) contain an elaborate explanation of how to do this and I also referred to these steps in my own project. I exposed my database credentials through environment variables in the `Deployment`, which isn't the most secure way but sufficient for a [MVP](https://www.techopedia.com/definition/27809/minimum-viable-product-mvp). Additionally, I deployed a `Service` [kubernetes object](https://kubernetes.io/docs/concepts/services-networking/service/) to expose the FastAPI container on port 80, the port commonly used for HTTP traffic. Depending on your load balancing solution, some additional steps are required to expose the [Swagger docs](https://swagger.io/docs/) which are [automatically generated](https://fastapi.tiangolo.com/features/) by FastAPI. More details about deployment strategies and [infra-as-code tools](https://www.terraform.io/) are out of scope for this article, so I'll save them for later!
