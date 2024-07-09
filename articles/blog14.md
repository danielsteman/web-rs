% id: 14
% title: Enabling Terraform authentication for AWS using OpenID
% date: 2024-04-28
% tags: devops

## OpenID

Conventionally, developers would have to create secrets in the application they want to authenticate with and store these secrets in a place where they are available for an application. This raises a security risk because secrets may be exposed if they're not being handled carefully. One way of mitigating this risk is using [OpenID](https://openid.net/developers/how-connect-works/), where the application outsources authentication to an OpenID Connect (OIDC) provider. The application that requires authentication is registered with the OIDC provider and uses tokens to verify the identity of the user.

## IaC (Infrastructure as code)

There are several ways to manage infrastructure with declarative code, but what I like about [Terraform](https://www.terraform.io/) is that it is cloud provider agnostic. This means that you can use the same code base to provision resources in AWS, GCP and Azure. While the three big cloud providers are the most common use case, many SaaS solutions that have an API often also have a Terraform provider.

## OpenID in the CI

Usually you don't want to apply a Terraform plan (a set of definitions of the infrastructure you want to deploy) from your local machine, but from a CI (continuous integration) pipeline that only runs after code has been reviewed and merged. The CI runner is the environment that runs the sequence of commands in that are described in the CI pipeline. To deploy anything in a cloud environment, [authentication and authorization](https://auth0.com/docs/get-started/identity-fundamentals/authentication-and-authorization) of the CI runner is required. However, since this requires some more cloud resources, you often have to bootstrap your Terraform project but deploying some things with Terraform from your local machine. This is not an issue since you can sync your local state with the remote state that is used by the CI runner.

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

Next, we need a role that is allowed to request temporary credentials. This role also gets an assume role policy to establish 'trust' with the OIDC provider. With the condition we make sure that the CI runner can only get temporary credentials if the CI run is on the main branch of a project specified in the `project_path` variable. This will prevent some one from deploying infrastructure from a feature branch. You can of course change this condition if your branching strategy is different.

```hcl
variable project_path {
  type    = string
  default = some_project_id/some_repo_name:ref_type:branch:ref:main
}

resource "aws_iam_role" "gitlab" {
  name = "gitlab"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Principal = {
          Federated = aws_iam_openid_connect_provider.gitlab.arn
        }
        Action = "sts:AssumeRoleWithWebIdentity"
        Condition = {
          StringEquals = {
            "gitlab.com:sub" = "project_path:${var.project_path}"
          }
        }
      }
    ]
  })
}
```
