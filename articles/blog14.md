% id: 13
% title: MLflow and Docker
% date: 2024-04-28
% tags: ml

## MLflow

In a performant data science team, there is a desire the train many models, many times, with several combinations of parameters and with different features (input variables). To determine which configuration performs best, each iteration needs to be tracked carefully. MLflow is an end-to-end machine learning solution that helps keeping track of trained models. These models are kept in a model registry, which has some similarities with a container registry (if you're familiar with Docker images). Each model consists of an artifact, a serialized file, that is linked to a data set (train set) and a model configuration.
