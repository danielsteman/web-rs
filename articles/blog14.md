% id: 14
% title: Enabling Terraform authentication for AWS using OpenID
% date: 2024-04-28
% tags: devops

## OpenID

Conventionally, developers would have to create secrets in the application they want to authenticate with and store these secrets in a place where they are available for an application. This raises a security risk because secrets may be exposed if they're not being handled carefully. One way of mitigating this risk is using OpenID, where the application outsources authentication to an identity provider (IDP). The application that requires authentication is registered with the IDP and uses tokens to verify the identity of the user. Go [here](https://openid.net/developers/how-connect-works/) to find out more.

## Infrastructure as code

There are several ways to manage infrastructure with declarative code, but what I like about Terraform is that it is cloud provider agnostic. This means that you can use the same code base to provision resources in AWS, GCP and Azure. While the three big cloud providers are the most common use case, many SaaS solutions that have an API often also have a Terraform provider.
