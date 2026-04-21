A hacker bot exploited a misconfigured [pull_request_target workflows](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows#pull_request_target) to get repository secrets. A workflow of this kind can be triggered by an external pull request. When I read this, I thought: how? Github masks secrets in workflows when you try to print them or write them to a file. What happened is: 

1. A hacker forked the Trivy repository, added code to [exfiltrate](https://www.ibm.com/think/topics/data-exfiltration) environment variables
2. Send them to an external server
3. Open a [PR](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/about-pull-requests) with the original Trivy repository as base
4. Check out the forked repository inside the [`apidiff`](https://pkg.go.dev/golang.org/x/exp/apidiff) workflow, which checks if a module has breaking changes for [semantic versioning](https://semver.org/). The maintainers of Trivy were under the impression that this was safe since the checkout is only done for static analysis, as [the comment of the now removed workflow suggests](https://github.com/aquasecurity/trivy/pull/10259/changes). The `Go` code in the forked repository is still executed during analysis and when this code is ran in the context of the original Trivy repository, it has access to environment variables in that environment, such as an organisation-scoped Github PAT. 
5. The team noticed and rotated secrets, but not _all secrets_. 

