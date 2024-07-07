% id: 14
% title: Enabling Terraform authentication for AWS using OpenID
% date: 2024-04-28
% tags: devops

## OpenID

Conventionally, developers would have to create secrets in the application they want to authenticate with and store these secrets in a place where they are available for an application. This raises a security risk because secrets may be exposed if they're not being handled carefully. One way of mitigating this risk is using [OpenID](https://openid.net/developers/how-connect-works/), where the application outsources authentication to an OpenID Connect (OIDC) provider. The application that requires authentication is registered with the OIDC provider and uses tokens to verify the identity of the user.

## Infrastructure as code

There are several ways to manage infrastructure with declarative code, but what I like about [Terraform](https://www.terraform.io/) is that it is cloud provider agnostic. This means that you can use the same code base to provision resources in AWS, GCP and Azure. While the three big cloud providers are the most common use case, many SaaS solutions that have an API often also have a Terraform provider.

## OpenID in the CI

Usually you don't want to apply a Terraform plan (a set of definitions of the infrastructure you want to deploy) from your local machine, but from a CI (continuous integration) pipeline that only runs after code has been reviewed and merged. To deploy anything in a cloud environment, [authentication and authorization](https://auth0.com/docs/get-started/identity-fundamentals/authentication-and-authorization) of the CI runner is required. The CI runner is the environment that runs the sequence of commands in that are described in the CI pipeline.

I want to deploy AWS infrastructure from a Gitlab CI, so the first step is to create a new OIDC provider.

```hcl
variable "gitlab_url" {
  type    = string
  default = "https://gitlab.com"
}

data "tls_certificate" "gitlab" {
  url = var.gitlab_url
}

resource "aws_iam_openid_connect_provider" "gitlab" {
  url = var.gitlab_url

  client_id_list = [
    var.gitlab_url,
  ]

  thumbprint_list = [data.tls_certificate.gitlab.certificates.0.sha1_fingerprint]
}
```

Next, we need a role that is allowed to request temporary credentials.
