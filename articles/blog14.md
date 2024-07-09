% id: 14
% title: Setting up CI/CD authentication for AWS using OpenID
% date: 2024-07-09
% tags: devops

## OpenID

Conventionally, developers would have to create secrets in the application they want to authenticate with and store these secrets in a place where they are available for an application. This raises a security risk because secrets may be exposed if they're not handled carefully. One way of mitigating this risk is using [OpenID](https://openid.net/developers/how-connect-works/), where the application outsources authentication to an OpenID Connect (OIDC) provider. The application that requires authentication is registered with the OIDC provider and uses tokens to verify the identity of the user.

<pre class="mermaid">
  sequenceDiagram
    participant GitLab CI Runner
    participant AWS OIDC as AWS OIDC Provider
    participant AWS Resource as AWS Account

    GitLab CI Runner->>AWS Resource: Attempt to access AWS resource
    AWS Resource->>GitLab CI Runner: Redirect to AWS OIDC Provider for authentication
    GitLab CI Runner->>AWS OIDC: Authentication request with OIDC token
    AWS OIDC-->>GitLab CI Runner: Authentication response
    GitLab CI Runner->>AWS Resource: Return with authentication token
    AWS Resource->>AWS OIDC: Validate authentication token
    AWS OIDC-->>AWS Resource: Token validation response
    AWS Resource-->>GitLab CI Runner: Grant access to AWS resource
</pre>

## IaC (Infrastructure as code)

There are several ways to manage infrastructure with declarative code, but what I like about [Terraform](https://www.terraform.io/) is that it is cloud provider agnostic. This means that you can use the same code base to provision resources in AWS, GCP and Azure. While the three big cloud providers are the most common use case, many SaaS solutions that have an API often also have a Terraform provider. Terraform uses [state](https://developer.hashicorp.com/terraform/language/state) to keep track of your current stack and to determine changes. State is kept in `terraform.tfstate`, which _can_ be kept locally, but _should_ be kept in remote storage. Maintaining the state remotely allows you to work on the same IaC project with others, and it's generally safer.

You can `terraform refresh` to update the state with the actual state of your cloud environment. You can `terraform import` cloud resources that are not yet tracked in the state. You can `terraform plan` to compare your Terraform code with the state, which results in an overview of which resources will be created, updated, or destroyed. You can 'terraform apply` this plan to let the previously reported changes take effect.

## OpenID in the CI

Usually you don't want to apply a Terraform plan (a set of definitions of the infrastructure you want to deploy) from your local machine, but from a CI (continuous integration) pipeline that only runs after code has been reviewed and merged. The CI runner is the environment that runs the sequence of commands that are described in the CI pipeline. To deploy anything in a cloud environment, [authentication and authorization](https://auth0.com/docs/get-started/identity-fundamentals/authentication-and-authorization) of the CI runner is required. However, since this requires additional cloud resources, you often have to bootstrap your Terraform project by deploying some things with Terraform from your local machine. This is not an issue since you can `terraform sync` your local state with the remote state that is used by the CI runner. GitLab even offers a [managed remote state backend](https://docs.gitlab.com/ee/user/infrastructure/iac/terraform_state.html).

I want to deploy AWS infrastructure from a GitLab CI, so the first step is to create a new OIDC provider.

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

Next, we need a role that is allowed to request temporary credentials. This role also gets an assume role policy to establish 'trust' with the OIDC provider. With the condition we make sure that the CI runner can only get temporary credentials if the CI run is on the main branch of the project specified in the `project_path` variable. This will prevent someone from deploying infrastructure from a feature branch. You can change this condition if your branching strategy is different.

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

This role has an ARN (Amazon Resource Name), which is a unique identifier of any resource. This ID is needed by the CI runner to identify as a trusted entity. You can pass this ID through a CI pipeline variable, such as `ROLE_ARN`.

```yml
auth:
  image: python:3.12-slim
  stage: auth
  id_tokens:
    GITLAB_OIDC_TOKEN:
      aud: https://gitlab.com
  script:
    - pip install awscli
    - >
      STS=($(aws sts assume-role-with-web-identity
      --role-arn $ROLE_ARN
      --role-session-name "gitlab-${CI_PROJECT_ID}-${CI_PIPELINE_ID}"
      --web-identity-token ${GITLAB_OIDC_TOKEN}
      --query 'Credentials.[AccessKeyId,SecretAccessKey,SessionToken]'
      --output text))

    - export AWS_ACCESS_KEY_ID="${STS[0]}"
    - export AWS_SECRET_ACCESS_KEY="${STS[1]}"
    - export AWS_SESSION_TOKEN="${STS[2]}"

    - echo "AWS_ACCESS_KEY_ID=${AWS_ACCESS_KEY_ID}" >> config.env
    - echo "AWS_SECRET_ACCESS_KEY=${AWS_SECRET_ACCESS_KEY}" >> config.env
    - echo "AWS_SESSION_TOKEN=${AWS_SESSION_TOKEN}" >> config.env
    - aws sts get-caller-identity
  artifacts:
    reports:
      dotenv: config.env
```

At the beginning, an ID token is added to the CI job. This token is used to authenticate with the OIDC provider. I'm using a Python image on which I install the `awscli` that is needed to interact with an AWS cloud account. The credentials are assigned to environment variables and these variables are bundled in an artifact that is picked up by the subsequent jobs. It's important to note that this is not an ideal way to transmit credentials between jobs because [artifacts can be downloaded](https://docs.gitlab.com/ee/ci/yaml/artifacts_reports.html). You can, however, restrict users from downloading artifacts by setting `actifacts:access`. Since the credentials fetched in the CI above are temporary and because my project is private, I didn't bother to do this.

The AWS provider looks for the environment variables that were set in the `auth` stage [by default](https://registry.terraform.io/providers/hashicorp/aws/latest/docs). So in the subsequent CI task we can `terraform init` to initialize the project and install the AWS provider.

```yml
dev_init:
  image: registry.gitlab.com/gitlab-org/terraform-images/stable:latest
  stage: init
  needs: [auth]
  variables:
    TF_ADDRESS: ${CI_API_V4_URL}/projects/${CI_PROJECT_ID}/terraform/state/dev
  script:
    - gitlab-terraform validate
    - gitlab-terraform init
```
