% id: 5
% title: ML model serving with Seldon Core 🤖🧠
% date: 2022-12-06
% tags: ml

Usually machine learning projects start off with the configuration and training of models, which can be accomplished relatively quickly thanks to libraries such as [scikit-learn](https://scikit-learn.org/stable/). Once trained, one can feed new data points to the model to calculate an output such as a number, which can provide new insights. At this moment, the trained model is only available on the system of the creator of the model. In case of, let's say, a model that calculates dynamic prices which are widely used across an organisation, this is not optimal nor desirable.

To solve this issue, the trained model needs to be exposed, which can be done by deploying it as an application on a server with an interface so it can communicate (API). Instead of reinventing the wheel and building a custom API, one could use [Seldon Core](https://github.com/SeldonIO/seldon-core) to do this. This open-source, _blazingly fast_ 🔥, framework, can convert machine learning models into production-ready REST or GRPC microservices. Aside from horizontal scalability, it offers a bunch of features such as "Advanced Metrics, Request Logging, Explainers, Outlier Detectors, A/B Tests, Canaries and more". I won't get into these features, but it's good to know that they exist.

You might ask, why Seldon Core and not [BentoML](https://github.com/bentoml/BentoML) or [Kubeflow](https://github.com/kubeflow/kubeflow) or another tool that has recently risen among the wave of [MLOps](https://ml-ops.org/) tools? The honest answer is: I don't know. You have to start somewhere, and Seldon Core seemed to match our (future) requirements. There are also managed solutions such as Azure ML, AWS Sagemaker and Google Cloud ML, but those provide less flexibility in keeping track of your models and exposing them.

![Seldon Core high level overview](../images/seldon-core-high-level.jpg)

The top image is from [the Seldon docs](https://docs.seldon.io/projects/seldon-core/en/latest/workflow/github-readme.html) and shows a simple example and a more elaborate example that uses more features. For my experiment, I went with a single model and focused on model serving. Getting started with Seldon Core is straight forward and if you already have a Kubernetes cluster running, you can use [this Helm chart](https://docs.seldon.io/projects/seldon-core/en/latest/charts/seldon-core-operator.html).

Next up, a ML model should be packaged in a Docker image and pushed to a registry, such that Kubernetes can pull the image and serve the model as a microservice. The first step is to serialize a trained model with [joblib](https://joblib.readthedocs.io/en/latest/). This example code serializes a simple cluster model and `dump`s it in the neighbouring `serialized_models` directory:

```py
from joblib import dump
from sklearn.cluster import KMeans

X = np.array([[1, 2], [1, 4], [1, 0], [10, 2], [10, 4], [10, 0]])
kmeans = KMeans(n_clusters=2, random_state=0).fit(X)
dump(kmeans, "serialized_models/cluster_model.joblib")
```

The prediction service is declared in Python code, as a class that should have a `predict` method that returns the prediction or classification and a constructor that loads the serialized model. To safeguard consistency among model deployments, I created this ML library-agnostic base class:

```py
from abc import ABC, abstractmethod
import numpy as np


class PredictionServiceBase(ABC):

    @abstractmethod
    def __init__(self):
        pass

    @abstractmethod
    def predict(self, X: np.ndarray, *args) -> np.ndarray:
        pass
```

An implementation could look like this:

```py
from joblib import load
import numpy as np

class ClusterModel(PredictionServiceBase):
    def __init__(self):
        with open("serialized_models/cluster_model.joblib", "rb") as f:
            self._model = load(f)

    def predict(self, X: np.ndarray[List[int]], *args) -> np.ndarray[int]:
        self._model.predict(X)
```

At this point, it's possible to run the micro service locally:

```bash
seldon-core-microservice src.prediction_service.{class_name} --service-type MODEL
```

Where `class_name` is the path to your prediction service class in dot notation (e.g. `src.model.PredictionService`). If you had nothing running on port 9000, this will be the port where the micro service is exposed. You can send requests in a separate terminal session using [curl](https://curl.se/). Depending on the shape of items in `X` (in the signature of the `predict` method) a curl request could look like this:

```bash
curl -X POST -H 'Content-Type: application/json' -d '{"data":{"ndarray":[[1,2]]}}' http://localhost:9000/api/v1.0/predictions
```

A more convenient way to send requests might be through the [swagger doc](https://swagger.io/docs/), which is also an out-of-the-box feature of Seldon Core. Make sure your service is running and go to `http://localhost:9000/api/v1.0/doc/`. Let's proceed to deployment 🚀.

Seldon Core has numerous [out-of-the-box model servers](https://docs.seldon.io/projects/seldon-core/en/latest/servers/sklearn.html) that should fit your use case. However, if you're pulling images from a private registry, follow [these instructions](https://docs.seldon.io/projects/seldon-core/en/latest/servers/sklearn.html) on how to write a Dockerfile.

By default, models are expected to be stored in public Google Storage, where a [storage initialiser](https://docs.seldon.io/projects/seldon-core/en/latest/servers/index.html) (ran as init container) can pull them from. It's also possible to use other storage providers and use other solutions for pull models. For example, it's also possible to pull the model during CI with [DVC](https://dvc.org/). An advantage of using DVC is that the serialized model and associated training dataset are always bundled in a single DVC commit.

If you have built a Docker image that contains your serialized model or if you are using an out-of-the-box model server, the micro service can be deployed using Seldon's custom resource definition ([CRD](https://kubernetes.io/docs/concepts/extend-kubernetes/api-extension/custom-resources/)) for Kubernetes, which looks something like this:

```yaml
apiVersion: machinelearning.seldon.io/v1alpha2
kind: SeldonDeployment
metadata:
  name: private-model
spec:
  name: private-model-example
  predictors:
    - componentSpecs:
        - spec:
            containers:
              - image: private-docker-repo/my-image
                name: private-model
            imagePullSecrets:
              - name: myreposecret
      graph:
        children: []
        endpoint:
          type: REST
        name: private-model
        type: MODEL
      name: private-model
      replicas: "1"
```

Or this, respectively:

```yaml
apiVersion: machinelearning.seldon.io/v1alpha2
kind: SeldonDeployment
metadata:
  name: sklearn
spec:
  name: iris
  predictors:
    - graph:
        children: []
        implementation: SKLEARN_SERVER
        modelUri: gs://seldon-models/v1.15.0-dev/sklearn/iris
        name: classifier
      name: default
      replicas: 1
```

These examples are copied directly from [the docs](https://docs.seldon.io/projects/seldon-core/en/latest/graph/private_registries.html). The `SeldonDeployment` resource can be deployed on Kubernetes with a simple `kubectl apply -f {path to yaml file or EOF-block}`. It's also possible to deploy resources like these in a CD pipeline, which I will cover in a future post.
