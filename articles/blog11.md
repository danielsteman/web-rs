% id: 11
% title: Deploying Databricks asset bundles using Gitlab CI 🧱
% date: 2023-12-15
% tags: data, devops

Databricks asset bundles (DAB) is a thin wrapper around Terraform that allows you to bundle pipelines ([delta live tables](https://www.databricks.com/product/delta-live-tables)), workflows, machine learning (ML) endpoints, ML experiments and ML model. Essentially, it's Databricks infrastructure-as-code (IaC) solution. These bundles contain YAML configuration files that make deployments highly configurable. In my experience, there is no reason to define your own Terraform code outside of DAB. The project structure looks something like:

```bash
├── README.md
├── databricks.yml
├── pyproject.toml
├── resources
│   ├── workflow.yml
│   └── variables.yml
└── src
    ├── model_inference.py
    ├── model_train.py
    └── utils.py
```

The bundle configuration is defined in `databricks.yml`. You can cut the configuration in several files and include files in `databricks.yml` like I have done with resources (`workflow.yml` and `variables.yml`):

```yaml
bundle:
  name: my_bundle

include:
  - resources/*.yml
```

In `variables.yml` you'll find variables that closely resemble Terraform variables:

```yaml
variables:
  commit_hash:
    default: "abc123"
```

These variables can contain an optional `default` value and can be overwritten through environment variables. The variable `commit_hash` can be set with the environment variable `BUNDLE_VAR_commit_hash`.

Instead of the `terraform` CLI, you can use the `databricks` CLI to validate and deploy bundles to a specific target:

```bash
databricks bundle validate
databricks bundle deploy -t dev
```

With the `deploy` command, the source files in the bundle will be stored in the Databricks filesystem (DBFS) under `files/` and the Terraform state files will be stored under `state/`. This means that you won't have to save state files in a self-provisioned storage solution like you'd do in conventional Terraform projects.

This layer of abstraction makes it so that the Gitlab CI pipeline is very simple. Only a couple of actions are required to deploy a single bundle:

```yaml
variables:
  BUNDLE_VAR_some_var: $some_cicd_variable
  BUNDLE_PATH: .

.install_databricks_cli:
  before_script:
    - curl -fsSL https://raw.githubusercontent.com/databricks/setup-cli/main/install.sh | sh

.install_python_dependencies:
  before_script:
    - echo "Installing Python dependencies..."
    - pip install poetry
    - poetry install

stages:
  - auth
  - validate
  - build
  - deploy

auth:
  stage: auth
  extends: .install_databricks_cli
  script:
    - |
      cat <<EOT >> $CI_PROJECT_DIR/.databrickscfg
      [DEFAULT]
      host             = https://$DATABRICKS_WORKSPACE_DEV.databricks.com/
      token            = $DATABRICKS_TOKEN_DEV
      jobs-api-version = 2.0

      [PROD]
      host             = https://$DATABRICKS_WORKSPACE_PROD.cloud.databricks.com/
      token            = $DATABRICKS_TOKEN_PROD
      jobs-api-version = 2.0
      EOT
  artifacts:
    paths:
      - $CI_PROJECT_DIR/.databrickscfg

validate:bundle:
  extends: .install_databricks_cli
  stage: validate
  script:
    - cd $BUNDLE_PATH
    - databricks bundle validate

deploy:dev:
  extends: .install_databricks_cli
  stage: deploy
  script:
    - cd $BUNDLE_PATH
    - databricks bundle deploy -t dev
  needs:
    - auth
    - validate:bundle
  rules:
    - if: "$CI_COMMIT_REF_NAME != $CI_DEFAULT_BRANCH && $CI_PIPELINE_SOURCE == 'merge_request_event'"
      when: on_success

deploy:prod:
  extends: .install_databricks_cli
  stage: deploy
  script:
    - cd $BUNDLE_PATH
    - databricks bundle deploy -t prod
  rules:
    - if: "$CI_COMMIT_REF_NAME == $CI_DEFAULT_BRANCH"
      when: on_success
```

To prevent duplication, initializing a Python environment with the required dependencies and downloading and authenticating the Databricks CLI have been abstracted in jobs that can be included in other jobs through the `extends:` key. The pipeline contains two deployment steps; one for development and one for production. Only one runs depending on the corresponding `rules`.

Things get a little bit more challenging when you have multiple DABs in one repo. I faced a repo with seven DABs that I wanted to deploy in a CI pipeline, but I also wanted to prevent code duplication. To solve this issue I wrote a `PipelineWriter`, a Python module that can be imported and called in downstream CI pipelines. The module contains a couple of functions that parameterize CI pipeline snippets by using multi-line F-strings. For example, to dynamically render a DAB validation job, I'd use these functions:

```py
class PipelineWriter:
    @staticmethod
    def deployment_parent_job_template(target: Environment) -> str:
        parent_job_template = f"""
workflow:
  rules:
    - when: always

stages:
  - auth
  - deploy-bundles-{target}
.install_databricks_cli:
  before_script:
    - curl -fsSL https://raw.githubusercontent.com/databricks/setup-cli/main/install.sh | sh

auth:
  stage: auth
  extends: .install_databricks_cli
  script:
    - |
      cat <<EOT >> $CI_PROJECT_DIR/.databrickscfg
      [DEFAULT]
      host             = https://dbc-123456.cloud.databricks.com/
      token            = $DATABRICKS_TOKEN_DEV
      jobs-api-version = 2.0

      [PROD]
      host             = https://dbc-123456.cloud.databricks.com/
      token            = $DATABRICKS_TOKEN_PROD
      jobs-api-version = 2.0
      EOT
  artifacts:
    paths:
      - $CI_PROJECT_DIR/.databrickscfg
"""
        return parent_job_template

    @staticmethod
    def deployment_child_pipeline_job_template(name: str, target: Environment) -> str:
        pretty_name = stringcase.spinalcase(name)
        child_pipeline_job_template = f"""
deploy-{pretty_name}-bundle-{target}:
  extends: .install_databricks_cli
  stage: deploy-bundles-{target}
  variables:
     BUNDLE: {name}
  script:
    - echo "Validate $BUNDLE"
    - cd bundles/$BUNDLE
    - databricks bundle deploy -t {target}
    - echo "Validated $BUNDLE"
  dependencies:
    - auth
"""
        return child_pipeline_job_template
```

The returned string is then written to a new `.yml` file like so:

```py
def get_bundles() -> List[str]:
    bundles_path = "bundles"
    bundles = [path for path in os.listdir(bundles_path) if os.path.isdir(f"{bundles_path}/{path}")]
    return bundles

def generate_validation_jobs() -> None:
    with open("validation-jobs.yml", "w+") as f:
        f.write(PipelineWriter.validation_parent_job_template())
        for bundle in get_bundles():
            f.write(PipelineWriter.validation_child_pipeline_job_template(bundle))
```

Where the parent job is written at the start of the document and the child jobs are appended at the bottom. Each folder in the `bundles` folder is considered a bundle. This is an important detail because if this assumption is wrong, the generated pipeline will be wrong. Now that the tools to generate pipeline are here, the parent and child jobs are created and ran:

```yaml
generate-deploy-dev-jobs:
  stage: generate-deploy-dev-jobs
  extends: .install_python_dependencies
  image: python:3.10
  script:
    - |
      cat <<EOT >> $CI_PROJECT_DIR/generate_deploy_dev_jobs.py
      from pipelinewriter.generate import generate_deployment_jobs
      generate_deployment_jobs("dev")
      EOT
    - python generate_deploy_dev_jobs.py
  artifacts:
    paths:
      - deploy-dev-jobs.yml
  allow_failure: false
  when: on_success

deploy-dev:bundles:
  stage: deploy-dev
  trigger:
    include:
      - artifact: deploy-dev-jobs.yml
        job: generate-deploy-dev-jobs
    strategy: depend
  needs:
    - job: generate-deploy-dev-jobs
      artifacts: true
  when: on_success
```

It seems like a hacky workaround, but it works like a charm. The child pipeline jobs are rendered on the right of the Gitlab CI pipeline graph and shows a failed child pipeline just like a regular failed pipeline task.
