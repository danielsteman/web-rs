# Cue lang

[CUE](https://cuelang.org/) is an open-sourced data validation language that helps you with validation, configuration and even generation of code. The CUE language allows you to be flexible in how you want to do each of those.

JSON ⊆ CUE, but CUE ⊈ JSON. [All valid JSON is CUE](https://cuelang.org/docs/tour/basics/json-superset/). However, most of my configuration is done in YAML, and YAML ⊈ CUE. While we're at it, it is also argued [by some](https://john-millikin.com/json-is-not-a-yaml-subset) that YAML ⊈ JSON. But many of the bugs that might arise from parsing JSON with a YAML parser or the other way around is by [being explicit about your types](https://hitchdev.com/strictyaml/why/implicit-typing-removed/). With this in mind, CUE can still be useful in my quest to standardize YAML configurations across projects.

I was especially interested in the validation capabilities, as I deal with a lot of YAML configuration files that describe Databricks or Kubernetes resources. Having all these configurations across projects can make it challenging to enforce policies before the deployment with given configuration takes place. The [databricks-cli](https://docs.databricks.com/aws/en/dev-tools/cli/bundle-commands) implements a command `databricks bundle validate` that parses the YAML configuration of a particular asset bundle and performs validation. This proved to be an effective mechanism to prevent deploying faulty configurations. Inspired by this, I created [bundlelint](https://danielsteman.com/blog/16) at the start of the year to do the same, but then with additional custom rules. It was a fun to build project but I quickly learned that the tool wasn't flexible enough. A DevOps colleague recognised these challenges and pointed me to CUE. The philosophy of `bundlelint` would remain, but it would leverage the powerful CUE language to let users come up with their own custom rules for Databricks asset bundle configurations. The fact that CUE integrates well with Go made it an even more appealing tool.

## Use case

At this point, my team is deploying asset bundles at scale and we want to make some guarantees about these deployments. For example, we want to guarantee that notifications are sent out whenever a production job fails. We can do this with the following schema:

```cue
#Job: {
  // Other fields are allowed (open struct by default)
  ...
  webhook_notifications: {
    on_failure: [{
      id: "sup"
    }]
  }
}

targets: {
  prod: {
    resources: {
      jobs: [string]: #Job
    }
  }
}
```

`databricks.yaml`, the root configuration file of an asset bundle, can `include` other `yaml` files. `bundlecues` contains logic to merge the root configuration with all included configurations. Now, we can use this Cue schema to validate the merged configuration. 
