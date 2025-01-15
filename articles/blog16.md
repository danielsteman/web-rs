# BundleLint

As you might have read in my previous posts, I have been using Databricks asset bundles quite extensively. If you are not familiar, these are packages consisting of Python code and `yaml` specifications to embrace the as-code philosophy. It reminds me a bit of how Kubernetes works, where each object is declared as a `yaml` block. It's super convenient to declare resources as code because it makes them reproducable and changes to resources are managed by some continuous integration and continuous delivery (CI/CD) process that involves a(n) (automated) code review (unless you push to `main`). Part of the automated review is [linting](https://stackoverflow.com/questions/8503559/what-is-linting). In many of my projects this is already done [before a new commit is created](https://danielsteman.com/blog/10), but it is done more extensively in the CI pipeline, which needs to pass for every [merge request](https://docs.gitlab.com/ee/user/project/merge_requests/). In case of Databricks asset bundles, you'd use the `databricks-cli` [bundle commands](https://docs.databricks.com/en/dev-tools/cli/bundle-commands.html). For example:

```bash
databricks bundle validate
```

This validates the `yaml` specifications in a bundle and will raise warnings for unknown keys or even errors for incorrect references. Up until now, this guard rail prevented many errors in to-be-merged code, but I wanted to take a step further. As the number of bundles grow, so does the potential for shipping bugs. This led to the creation of [BundleLint](https://github.com/danielsteman/bundlelint). It is a command line tool that can perform more custom and opinionated checks. An example of such a check is making sure that Databricks workflows that are deployed to the production target (AKA production workspace) send notifications to Slack when they fail.

Like the `databricks-cli`, `bundlelint` was built using [cobra](https://github.com/spf13/cobra), a simple interface for Go that takes care of a lot of common CLI functionality so you don't have to. What I liked about it is that you can easily register functions that need to be executed when a CLI command is ran. The function takes

```bash
❯ bundlelint -h
A CLI to govern your Databricks asset bundles with flexibility.

Usage:
  bundlelint [bundle_path] [flags]

Flags:
  -h, --help      help for bundlelint
  -v, --version   version for bundlelint
```

Using a compiled language makes it easier to deploy the application because you don’t have to worry as much about system dependencies, such as a JVM for Java, `node_modules` for Javascript or a Python interpreter for, yes you guessed it, Python. With Go, you can [cross compile](https://golangcookbook.com/chapters/running/cross-compiling/) for all architectures and [release](https://github.com/danielsteman/bundlelint/releases/tag/v1.0.0) the pre built tool for Mac, Mac silicon, Linux etc. The binary can be compressed using [tar](https://www.gnu.org/software/tar/), reducing the file size that was quite small to begin with, and preserving the file's permissions, which is required to make the `bundlelint` binary [executable](https://en.wikipedia.org/wiki/Chmod). `tar` also allows us to compress the same file name, `bundlelint`, multiple times and call the output differently. This is useful for cross compilation, as you can build `bundlelint` several times and create a compressed file with the architecture in its name after each time you compile. In the following snippet, I'm compiling for both ARM64 (Apple silicon) and AMD64 (Good ol' Intel 📉).

```bash
GOOS=darwin GOARCH=arm64 go build -o bundlelint
tar -czvf bundlelint_1.0.0_darwin_arm64.tar.gz bundlelint

GOOS=darwin GOARCH=amd64 go build -o bundlelint
tar -czvf bundlelint_1.0.0_darwin_amd64.tar.gz bundlelint
```

## Homebrew

[formula](https://github.com/danielsteman/homebrew-tap/blob/main/Formula/bundlelint.rb)
