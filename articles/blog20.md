The Trivy hack has been discussed a lot over the past month, but I still wanted to take the opportunity to write about the details of this exploit, both to share knowledge and deepen my own knowledge to guard codebases that I contribute to against adversaries. I'll do this by writing about the events in a chronological order and get into more details when it gets complex. 

## The exploit

An AI-powered hacker bot, [hackerbot-claw](https://www.linkedin.com/posts/cybersecurity-supplychainsecurity-githubactions-share-7433820997457469440-SUuo/), exploited a misconfigured [pull_request_target workflows](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows#pull_request_target) to get repository secrets from the Trivy project. A workflow of this kind can be triggered by an external pull request and may be configured in an unsafe manner. This how credentials were stolen from the Trivy project:

1. A hacker forked the Trivy repository, added code to [exfiltrate](https://www.ibm.com/think/topics/data-exfiltration) environment variables
2. Send them to an external server
3. Open a [PR](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/about-pull-requests) with the original Trivy repository as base. Also known as a [pwd request](https://www.praetorian.com/blog/pwn-request-hacking-microsoft-github-repositories-and-more/).
4. Checkout the forked repository inside the [`apidiff`](https://pkg.go.dev/golang.org/x/exp/apidiff) workflow, which checks if a module has breaking changes for [semantic versioning](https://semver.org/). The maintainers of Trivy were under the impression that this was safe since the checkout is only done for static analysis, as [the comment of the now removed workflow suggests](https://github.com/aquasecurity/trivy/pull/10259/changes). The `Go` code in the forked repository is still executed during analysis and when this code is ran in the context of the original Trivy repository, it has access to environment variables in that environment, such as an organisation-scoped Github PAT. 

The team noticed and rotated secrets, but not _all secrets_. After doing some digging it doesn't seems to be public information which secrets weren't rotated, it was only stated that rotation was "incomplete". I guess it makes sense in an organisation that has _many_ secrets in _many_ places, that there _might be_ a secret that was not in scope of the rotation process. This did make me review my rotation process to make sure that everything is in scope, because it only takes one forgotten secret to nullify all other measures. Anyways, no real harm was done using these credentials, yet. This is what happened next:

6. Three weeks later, another actor got involved: [TeamPCP](https://cyble.com/threat-actor-profiles/teampcp/). They used stolen credentials to [spoof](https://en.wikipedia.org/wiki/Spoofing_attack) commits to the [Trivy action repository](https://github.com/aquasecurity/trivy-action). Each commit cloned the original commit metadata, timestamp and commit message to make them go unnoticed in the Git history and therefor hard to catch. These commits contained changes in the [`entrypoint.sh` file](https://github.com/aquasecurity/trivy-action/blob/master/entrypoint.sh) that silently scanned the environment it was running on for credentials. Another change was made in `action.yaml` with the same credential-stealing logic. Then, the attacker force-pushed version tags on [`aquasecurity/setup-trivy`](https://github.com/aquasecurity/setup-trivy) and [`aquasecurity/trivy-action`](https://github.com/aquasecurity/trivy-action) to point at the malicious commits. Anyone using these workflows with a tag such as `0.28.0` would then pull the workflow with the malicious `entrypoint.sh` and run that on their Github runners. The changes in `entrypoint.sh` have now been reverted and are not public information anymore, but [this write up](https://www.abgeo.dev/blog/trivy-github-actions-compromised-full-payload-analysis) describes how to script actually acquired credentials on its host. To summarize: it looks for all processes that run on the Github runner. For each process, it reads [`/proc/<PID>/environ`, its environment](https://man7.org/linux/man-pages/man5/proc_pid_environ.5.html), and [`/proc/<PID>/mem`, its memory](https://man7.org/linux/man-pages/man5/proc_pid_mem.5.html), that contain environment variables that either hold a secret value or a reference to the file on disk that contains the secret value. The latter is useful because SSH keys or TLS private keys are contained in files and often referenced with a file path (e.g. `/home/user/.ssh/id_ed25519`).
7. While hijacking version tags on Trivy's Github actions, the attacker also created a backdoor in Trivy's binary. This binary installed a [persistent backdoor](https://www.offsec.com/metasploit-unleashed/persistent-backdoors/), in this case a systemd user service in `~/.config/systemd/user/sysmon.py` that polled a URL that pointed to an [ICP hosted canister](https://docs.internetcomputer.org/building-apps/essentials/canisters). Before continuing, let's clear some things up. A canister is a [smart contract](https://www.freecodecamp.org/news/smart-contracts-for-dummies-a1ba1e0b9575/) compiled to [Web Assembly](https://danielsteman.com/blog/4) that can store state and respond to HTTP requests. ICP is decentralised internet, making it very suitable to host malware since it's not possible to take down a website. This means that the canister's lifecycle is governed by the blockchain's consensus protocol. Taking it down would involve the [Network Nervous System](https://nns.ic0.app/), which is a decentralized voting process, and deserves a complete blog post on its own. After some more digging I found that the malicious canister continued to live on the blockchain but [DFINITY](https://dfinity.org/) stopped the [boundary nodes](https://learn.internetcomputer.org/hc/en-us/articles/34212818609684-ICP-Edge-Infrastructure) to serve it. These boundary nodes are the primary interface for users of ICP and enforce security measures to protect the network. What fascinates me is that the _decentralised internet_ apparently still has a _central_ actor that can interfene, so it's not completely like the wild west, as you might think if you're not very familiar with the decentralised web, like me. 
8. Stolen secrets were encrypted with [AES-256-CBC](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard) and then with [RSA-4096](https://en.wikipedia.org/wiki/RSA_cryptosystem). AES-256-CBC is a symmetric encryption algorithm, while RSA-4096 is asymmetric, meaning the former uses a single shared key for both encryption and decryption, whereas the latter relies on a pair of distinct keys. This hybrid solution makes sense: symmetric encryption is much more performant than asymmetric encryption, so when the attacker wants to encrypt a large blob of stolen credentials, AES allows for fast encryption and RSA is used to encrypt the AES _encryption key_, which is relatively small. 
9. Next to stealing secrets of Trivy users, TeamPCP also stole secrets from Aqua Security itself. Days after the attack, [Aqua Security internal repositories were hijacked](https://opensourcemalware.com/blog/teampcp-aquasec-com-github-org-compromise) and showed "TeamPCP Owns Aqua Security" for a brief moment. On the same day, stolen Docker Hub credentials were sued to force-push malisicous Docker images. 

To give you some more context so you can estimate the number of credentials that were stolen, here is an overview of the exposure window per attack vector:

| Vector | Compromise start (UTC) | Contained (UTC) | Exposure window |
|---|---|---|---|
| trivy binary v0.69.4 | Mar 19, ~18:22 | Mar 19, ~21:42 | ~3 hours |
| setup-trivy (all 7 tags) | Mar 19, ~17:43 | Mar 19, ~21:44 | ~4 hours |
| trivy-action (76 of 77 tags) | Mar 19, ~17:43 | Mar 20, ~05:40 | ~12 hours |
| Docker Hub images (v0.69.5, v0.69.6) | Mar 22, ~16:00 | Mar 22 (same day) | Hours (exact removal time unclear) |
| Internal Aqua repos defaced (44 repos) | Mar 22, ~20:31 | Mar 22 (same day) | Hours (exact removal time unclear) |
| ICP canister (C2 for sysmon.py backdoor) | Mar 19, ~17:43 | Mar 22, 21:31 | ~3 days |

## Counter measures

### Pin open source Github action templates

This event once again showed the importance of the [zero trust](https://www.cloudflare.com/learning/security/glossary/what-is-zero-trust/) philosophy when building a platform. Part of this is not trusting mutable version tags of open source software. It's better to pin on the immutable commit hash (the SHA). In a Github workflow, instead of this: 

```yaml
- name: Run Trivy (pinned to version tag)
        uses: aquasecurity/trivy-action@0.24.0
```

Do this: 

```yaml
- name: Run Trivy (pinned to version tag)
        uses: aquasecurity/trivy-action@b3f1c2a9d7e4c6a1f2b8d9e0c123456789abcdef0
```

### Disable post-install scripts

Some [npm](https://www.npmjs.com/)(Node Package Manager) dependencies rely on scripts that run after you install the package, to automatically bootstrap the environment for example, or compile source code. This is convenient but also a security risk, because post install scripts can execute maliscious code. Luckily, you can disable these scripts at install time: 

```bash
npm install --ignore-scripts
```

```bash
yarn install --ignore-scripts
```

```bash
bun install --ignore-scripts
```

```bash
deno install --no-npm-lifecycle-scripts
```

It is adviced to do this in your CI to prevent that your build machine will be compromised by a malicious open source package, such as the secret scanner hidden in a Trivy release.

### Get rid of long-lived secrets

Stolen secrets are worthless if they are expired, so we better make sure that the life time of secrets is short. This can be achieved with frequent (daily) secrets rotation or using OpenID for authentication. Check out [my other post about OIDC](https://danielsteman.com/blog/14) for more information about setting this up in your CI/CD pipeline. 

The Github runner exchanges a OIDC token for AWS Security Token Service (STS) credentials that have a 1 hour lifetime by default. The STS secrets consist of a `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` and `AWS_SESSION_TOKEN`, which can be used to access an AWS account. If these credentials get stolen, a hacker would have 1 hour to do whatever the associated IAM role allows. This is already much better than a long lived developer credential, but it's not great. It's possible to shorten the life time of these tokens further, shortening the exposure window. Also, most CI/CD setups use a single role for several tasks in the pipeline. You can consider to create more roles with tighter permissions, scoped to the work that each tasks does, keeping the [least privileges principle](https://en.wikipedia.org/wiki/Principle_of_least_privilege_) in mind. This makes the attack surface smaller, may the STS secrets be compromised. 
