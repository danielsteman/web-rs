% id: 20
% title: Trivy Supply Chain Attack 🔗☠️
% date: 2026-05-08
% tags: devsecops

The Trivy hack has been discussed a lot over the past month, but I still wanted to take the opportunity to write about the details of this exploit, both to share knowledge and deepen my own knowledge to guard codebases that I contribute to against adversaries. I'll do this by writing about the events in a chronological order and get into more details to find out what _actually_ happens.

## The exploit

An AI-powered hacker bot, [hackerbot-claw](https://www.linkedin.com/posts/cybersecurity-supplychainsecurity-githubactions-share-7433820997457469440-SUuo/), exploited a misconfigured [pull_request_target workflows](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows#pull_request_target) to get repository secrets from the Trivy project. A workflow of this kind can be triggered by an external pull request and may be configured in an unsafe manner. This how credentials were stolen from the Trivy project:

### 1. Forking the repo and adding exfiltration code

A hacker forked the Trivy repository and added code to [exfiltrate](https://www.ibm.com/think/topics/data-exfiltration) secrets. Github Actions injects secrets into the runner's processes as environment variables, so the malicious code targeted those variables, scanning for credentials like Github personal access tokens (PAT), AWS keys, and container registry credentials. The fork otherwise looked legitimate. The only meaningful change was the harvesting logic, hidden in a place where it would be invoked once a workflow ran.

### 2. Sending secrets to an external server

Once the exfiltration code collected the secrets, it bundled them up and shipped them off to an attacker-controlled server over HTTPS. HTTPS is convenient for an attacker because it blends in with the legitimate outbound traffic that any CI runner produces (`git fetch`, `npm install`, `docker pull`, etc.), so it doesn't trigger any obvious red flags in network logs. At this point the code just sits in the fork; it doesn't do anything until something runs it in a context where real secrets are available.

### 3. Opening a pwn request

The hacker then opened a [PR](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/about-pull-requests) from the fork against the original Trivy repository, also known as a [pwn request](https://www.praetorian.com/blog/pwn-request-hacking-microsoft-github-repositories-and-more/). The trick is to target a workflow that uses [`pull_request_target`](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows#pull_request_target). Unlike the regular `pull_request` trigger, `pull_request_target` runs in the context of the _base_ repository with access to its secrets, which is exactly what the attacker needs. The dangerous combination is when such a workflow then checks out and executes code from the fork, which is what happened here.

### 4. Checkout inside the `apidiff` workflow

Checkout the forked repository inside the [`apidiff`](https://pkg.go.dev/golang.org/x/exp/apidiff) workflow, which checks if a module has breaking changes for [semantic versioning](https://semver.org/). The maintainers of Trivy were under the impression that this was safe since the checkout is only done for static analysis, as [the comment of the now removed workflow suggests](https://github.com/aquasecurity/trivy/pull/10259/changes). The `Go` code in the forked repository is still executed during analysis and when this code is ran in the context of the original Trivy repository, it has access to environment variables in that environment, such as an organisation-scoped Github PAT.

### 5. Incomplete secret rotation

The team noticed and rotated secrets, but not _all secrets_. After doing some digging it doesn't seem to be public information which secrets weren't rotated, it was only stated that rotation was "incomplete". I guess it makes sense in an organisation that has _many_ secrets in _many_ places, that there _might be_ a secret that was not in scope of the rotation process. This did make me review my rotation process to make sure that everything is in scope, because it only takes one forgotten secret to nullify all other measures. Anyways, no real harm was done using these credentials, yet. This is what happened next:

### 6. TeamPCP spoofs commits and force-pushes tags

Three weeks later, another actor got involved: [TeamPCP](https://cyble.com/threat-actor-profiles/teampcp/). They used stolen credentials to [spoof](https://en.wikipedia.org/wiki/Spoofing_attack) commits to the [Trivy action repository](https://github.com/aquasecurity/trivy-action). Each commit cloned the original commit metadata, timestamp and commit message to make them go unnoticed in the Git history and therefor hard to catch. These commits contained changes in the [`entrypoint.sh` file](https://github.com/aquasecurity/trivy-action/blob/master/entrypoint.sh) that silently scanned the environment it was running on for credentials. Another change was made in `action.yaml` with the same credential-stealing logic. Then, the attacker force-pushed version tags on [`aquasecurity/setup-trivy`](https://github.com/aquasecurity/setup-trivy) and [`aquasecurity/trivy-action`](https://github.com/aquasecurity/trivy-action) to point at the malicious commits. Anyone using these workflows with a tag such as `0.28.0` would then pull the workflow with the malicious `entrypoint.sh` and run that on their Github runners. The changes in `entrypoint.sh` have now been reverted and are not public information anymore, but [this write up](https://www.abgeo.dev/blog/trivy-github-actions-compromised-full-payload-analysis) describes how to script actually acquired credentials on its host. To summarize: it looks for all processes that run on the Github runner. For each process, it reads [`/proc/<PID>/environ`, its environment](https://man7.org/linux/man-pages/man5/proc_pid_environ.5.html), and [`/proc/<PID>/mem`, its memory](https://man7.org/linux/man-pages/man5/proc_pid_mem.5.html), that contain environment variables that either hold a secret value or a reference to the file on disk that contains the secret value. The latter is useful because SSH keys or TLS private keys are contained in files and often referenced with a file path (e.g. `/home/user/.ssh/id_ed25519`).

We can actually try this harvesting technique ourselves. I used [Lima](https://github.com/lima-vm/lima) to spin up a [virtual machine (VM)](https://en.wikipedia.org/wiki/Virtual_machine). Open a shell in a VM after installing `lima`:

```bash
limactl start
# use the default config for this example
lima
```

We'll export some fake secrets and trigger a `sleep` process

```bash
export AWS_SECRET_ACCESS_KEY=FAKE_AWS_SECRET
export GITHUB_TOKEN=ghp_fake_token
export SSH_KEY_PATH=/home/danielsteman.linux/.ssh/id_ed25519

sleep 999999
```

We can find this process with [`ps`](https://man7.org/linux/man-pages/man1/ps.1.html), which I believe stands for "process snapshot", plus [some parameters](https://unix.stackexchange.com/questions/106847/what-does-aux-mean-in-ps-aux).

```bash
ps aux | grep sleep
```

We can already find the secrets in the environment of the process. Let's assume that the process we started earlier has PID `1234`. We can `cat` and format the environment like this.

```bash
cat /proc/1234/environ | tr '\0' '\n'
```

Which would return our fake secrets, among other variables.

```bash
SHELL=/bin/bash
SSH_KEY_PATH=/home/lima/.ssh/id_ed25519
PWD=/Users/danielsteman/repos/web-rs
LOGNAME=danielsteman
XDG_SESSION_TYPE=tty
HOME=/home/danielsteman.linux
LANG=C.UTF-8
GITHUB_TOKEN=ghp_fake_token
AWS_SECRET_ACCESS_KEY=FAKE_AWS_SECRET
```

A more advanced way of harvesting secrets would be to read them from a process's memory. According to Aqua Security this is what TeamPCP did in their malware. In their post mortem, its stated that secrets were also retrieved from `/proc/{pid}/mem`, but as opposed to `/proc/{pid}/environ`, this is not a regular file that you can `cat` but a [virtual file](https://docs.kernel.org/filesystems/vfs.html).

Virtual files don't take up space on disk, but are backed by kernel handler functions. Following Unix's "everything is a file" philosophy, files and virtual files share the open/read/write semantics. When a virtual file is opened, a kernel handler function is called that generates and returns data. For example, `/proc/cpuinfo` is a virtual file that I can open. When we `open()` it, we do a lookup the proc file system, `procfs`, which has a set of file operations. When we `read()` the `/proc/cpuinfo`, we call `cpuinfo_read` which generates and returns CPU data to us. So it's very similar to reading a file, but the data that you get back is generated on the fly.

We got a little side tracked, but this is good to understand to further explain how secrets can be stolen from process memory. The malware targeted the `Runner.Worker` process on the Github runner of a victim. The readable memory regions of this process can be found in `/proc/{pid}/maps`, which was accessible because a Github workflow is ran with the `runner` user and the process is also owned by the `runner` user. For each region, `/proc/{pid}/mem` was dumped and parsed to extract secrets.

```bash
# start a random process
sleep 999999 &
# returns the PID: 1234
```

We take that PID and find the readable memory regions. I created a simple Python script to make it more readable.

```python
import re

pid = 1234

with open(f"/proc/{pid}/maps") as maps, open(f"/proc/{pid}/mem", "rb") as mem:
	for line in maps:
		# parse start and end address from each maps line
		m = re.match(r'([0-9a-f]+)-([0-9a-f]+) r', line)
		# skip non-readable regions
		if not m:
			continue
		start = int(m.group(1), 16)
		end = int(m.group(2), 16)
		# seek to the start address and read the region
		mem.seek(start)
		data = mem.read(end - start)
		# search for secrets
		if b'"isSecret":true' in data:
			print(data)
```

### 7. Persistent backdoor via an ICP canister

While hijacking version tags on Trivy's Github actions, the attacker also created a backdoor in Trivy's binary. This binary installed a [persistent backdoor](https://www.offsec.com/metasploit-unleashed/persistent-backdoors/), in this case a systemd user service in `~/.config/systemd/user/sysmon.py` that polled a URL that pointed to an [ICP hosted canister](https://docs.internetcomputer.org/building-apps/essentials/canisters). Before continuing, let's clear some things up. A canister is a [smart contract](https://www.freecodecamp.org/news/smart-contracts-for-dummies-a1ba1e0b9575/) compiled to [Web Assembly](https://danielsteman.com/blog/4) that can store state and respond to HTTP requests. ICP is decentralised internet, making it very suitable to host malware since it's not possible to take down a website. This means that the canister's lifecycle is governed by the blockchain's consensus protocol. Taking it down would involve the [Network Nervous System](https://nns.ic0.app/), which is a decentralized voting process, and deserves a complete blog post on its own. After some more digging I found that the malicious canister continued to live on the blockchain but [DFINITY](https://dfinity.org/) stopped the [boundary nodes](https://learn.internetcomputer.org/hc/en-us/articles/34212818609684-ICP-Edge-Infrastructure) to serve it. These boundary nodes are the primary interface for users of ICP and enforce security measures to protect the network. What fascinates me is that the _decentralised internet_ apparently still has a _central_ actor that can intervene, so it's not completely like the wild west, as you might think if you're not very familiar with the decentralised web, like me.

### 8. Hybrid encryption of stolen secrets

Stolen secrets were encrypted with [AES-256-CBC](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard) and then with [RSA-4096](https://en.wikipedia.org/wiki/RSA_cryptosystem). AES-256-CBC is a symmetric encryption algorithm, while RSA-4096 is asymmetric, meaning the former uses a single shared key for both encryption and decryption, whereas the latter relies on a pair of distinct keys. This hybrid solution makes sense: symmetric encryption is much more performant than asymmetric encryption, so when the attacker wants to encrypt a large blob of stolen credentials, AES allows for fast encryption and RSA is used to encrypt the AES _encryption key_, which is relatively small.

### 9. Typosquatted domain and dead-drop fallback

The exfiltration part is also interesting due to its fallback logic. Stolen secrets were initially sent to a typo squatted domain, scan.aquasecutiy.com, soit doesnt raise immediate attention in traffic logs. As a fallback, the malware created a public repository using the Github PAT of the victim named `tpcp-docs` and upload the encrypted secrets as release assets. This public repository served as a [dead drop](https://en.wikipedia.org/wiki/Dead_drop) location for the attacker. I was especially impressed with the fallback method because exfiltration through a Github public repository should in most cases be possible, since almost all runners need to be able to access github.com.

### 10. Hijacking Aqua Security's repos and Docker images

Next to stealing secrets of Trivy users, TeamPCP also stole secrets from Aqua Security itself. Days after the attack, [Aqua Security internal repositories were hijacked](https://opensourcemalware.com/blog/teampcp-aquasec-com-github-org-compromise) and showed "TeamPCP Owns Aqua Security" for a brief moment. On the same day, stolen Docker Hub credentials were sued to force-push malicious Docker images.

### Overview

To give you some more context so you can estimate the number of credentials that were stolen, here is an overview of the exposure window per attack vector:

| Vector                                   | Compromise start (UTC) | Contained (UTC) | Exposure window                    |
| ---------------------------------------- | ---------------------- | --------------- | ---------------------------------- |
| trivy binary v0.69.4                     | Mar 19, ~18:22         | Mar 19, ~21:42  | ~3 hours                           |
| setup-trivy (all 7 tags)                 | Mar 19, ~17:43         | Mar 19, ~21:44  | ~4 hours                           |
| trivy-action (76 of 77 tags)             | Mar 19, ~17:43         | Mar 20, ~05:40  | ~12 hours                          |
| Docker Hub images (v0.69.5, v0.69.6)     | Mar 22, ~16:00         | Mar 22          | Hours (exact removal time unclear) |
| Internal Aqua repos defaced (44 repos)   | Mar 22, ~20:31         | Mar 22          | Hours (exact removal time unclear) |
| ICP canister (C2 for sysmon.py backdoor) | Mar 19, ~17:43         | Mar 22, 21:31   | ~3 days                            |

## Counter measures

What makes it interesting to learn about hacks like these, is to think about what would've been necessary to be resilient. It feels good to know that you built a platform that cannot be [pwnd](https://www.urbandictionary.com/define.php?term=pwnd), even by advanced supply chain attacks like the one we're discussing. So instead of waiting for things to go wrong, let's look at some of the things you can do to make your platform resilient.

### Pin open source Github action templates

Recent attacks have once again shown the importance of the [zero trust](https://www.cloudflare.com/learning/security/glossary/what-is-zero-trust/) philosophy when building a platform. Part of this is not trusting mutable version tags of open source software. It's better to pin on the immutable commit hash (the SHA). In a Github workflow, instead of this:

```yaml
- name: Run Trivy (pinned to version tag)
	uses: aquasecurity/trivy-action@v0.36.0
```

Do this:

```yaml
- name: Run Trivy (pinned to version tag)
  # https://github.com/aquasecurity/trivy-action/commit/ed142fd0673e97e23eac54620cfb913e5ce36c25
	uses: aquasecurity/trivy-action@ed142fd0673e97e23eac54620cfb913e5ce36c25 # v0.36.0
```

It's even better to add the URL to the commit so you can easily verify if the commit belongs to the original repository. It's also convenient to add the version number.

### Disable post-install scripts

Some [npm](https://www.npmjs.com/)(Node Package Manager) dependencies rely on scripts that run after you install the package, to automatically bootstrap the environment, or compile source code, for example. This is convenient but also a security risk, because post install scripts can execute maliscious code. Luckily, you can disable these scripts at install time:

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

It is advised to do this in your CI to prevent that your build machine will be compromised by a malicious open source package, such as the secret scanner hidden in a Trivy release.

### Get rid of long-lived secrets

Stolen secrets are worthless if they are expired, so we better make sure that the life time of secrets is short. This can be achieved with frequent (daily) secrets rotation or using OpenID for authentication. Check out [my other post about OIDC](https://danielsteman.com/blog/14) for more information about setting this up in your CI/CD pipeline.

The Github runner exchanges an OIDC token for AWS Security Token Service (STS) credentials that have a 1 hour lifetime by default. The STS secrets consist of a `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` and `AWS_SESSION_TOKEN`, which can be used to access an AWS account. If these credentials get stolen, a hacker would have 1 hour to do whatever the associated IAM role allows. This is already much better than a long lived developer credential, but it's not great. It's possible to shorten the life time of these tokens further, shortening the exposure window. Also, most CI/CD setups use a single role for several tasks in the pipeline. You can consider to create more roles with tighter permissions, scoped to the work that each tasks does, keeping the [least privileges principle](https://en.wikipedia.org/wiki/Principle_of_least_privilege_) in mind. In a scenario where the STS secrets are compromised, the attacker wouldn't be able to do much.

Outside of the CI/CD runners, on developer machines, it's possible to realise short lived secrets using [SSO](https://aws.amazon.com/iam/identity-center/).

### Limit outbound traffic

Exfiltration is only possible if stolen credentials leave the owner's environment. This can be prevented by using self-hosted runners that live in a private network (AWS offers Virtual Private Cloud, or VPC). There is a number of ways to set this up and it really depends on how to rest of your platform is setup, but with a VPC it's easy to manage firewall rules and security groups that prevent any traffic going to a malicious address. When using self-hosted Github runners, it's also possible to use a [proxy server](https://docs.github.com/en/actions/how-tos/manage-runners/use-proxy-servers) like [nginx](https://nginx.org/) that has a deny-all policy and has an allow list of addresses that you trust.

<pre class="mermaid">
  flowchart LR
    subgraph vpc["Private network / VPC"]
      runner["GitHub runner"]
      proxy["Proxy (deny-all + allowlist)"]
      runner -->|"outbound HTTP(S)"| proxy
    end
    proxy -->|"allowed"| trusted["Trusted hosts"]
    proxy -.->|"blocked"| untrusted["Untrusted / malicious"]
</pre>

If your Github runner would've been compromised by the malicious Trivy workflows, exfiltration of secrets to the typosquatted address (scan.aquasecutiy.org) would not have been possible. This would not have closed off the fallback method though (write encrypted secrets to a user's public repo release assets and use it as a dead drop).

### Software Bill of Materials (SBOM)

You could compare the SBOM with an ingredient list. It states what components have been used to build your application. Often times, software applications are built using hundred if not thousands of other software components, that all have their own versioning and are built from other software components. You can imagine that this becomes complex very fast and it would be impossible to keep track without a programmatic approach. _Why is it important to keep track of the ingredients list in the first place?_ - you might ask yourself. It can help you to determine that your application doesn't include any malicious components. It also serves as an audit trail that is valuable when handling an incident. Next to vulnerabilities, we can also analyse the licenses of underlying packages, which is important to do for compliance reasons. If one of the packages that you use is stating that you have to mention its maintainers somewhere in your software, you have to do that otherwise you'd violate the license of the software you're using. There are two industry standard formats of the SBOM:

- CycloneDX - [OWASP](https://owasp.org/) maintained, used for supply chain analysis
- SPDX (Software Package Data Exchange) - [Linux Foundation](https://www.linuxfoundation.org/) maintained, used to assess license compliance

Several security tools, such as Trivy or [Snyk](https://snyk.io/) can generate a SBOM for you. An example CycloneDX SBOM in JSON format looks like this:

```json
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "serialNumber": "urn:uuid:4b1e5fa2-3c8d-4a1f-b2f7-9e0a1c23d456",
  "version": 1,
  "metadata": {
    "timestamp": "2026-05-08T10:30:00Z",
    "tools": [
      {
        "vendor": "CycloneDX",
        "name": "cyclonedx-python-lib",
        "version": "5.1.0"
      }
    ],
    "component": {
      "type": "application",
      "bom-ref": "app-root",
      "name": "my-web-api",
      "version": "2.4.1",
      "description": "Internal REST API service"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "pkg:pypi/fastapi@0.111.0",
      "name": "fastapi",
      "version": "0.111.0",
      "purl": "pkg:pypi/fastapi@0.111.0",
      "licenses": [{ "license": { "id": "MIT" } }],
      "hashes": [{ "alg": "SHA-256", "content": "a3f1c2d4e5b6..." }],
      "externalReferences": [
        { "type": "website", "url": "https://fastapi.tiangolo.com" }
      ]
    },
    {
      "type": "library",
      "bom-ref": "pkg:pypi/pydantic@2.7.1",
      "name": "pydantic",
      "version": "2.7.1",
      "purl": "pkg:pypi/pydantic@2.7.1",
      "licenses": [{ "license": { "id": "MIT" } }],
      "hashes": [{ "alg": "SHA-256", "content": "b9e2a4f7c1d3..." }]
    },
    {
      "type": "library",
      "bom-ref": "pkg:pypi/uvicorn@0.29.0",
      "name": "uvicorn",
      "version": "0.29.0",
      "purl": "pkg:pypi/uvicorn@0.29.0",
      "licenses": [{ "license": { "id": "BSD-3-Clause" } }],
      "hashes": [{ "alg": "SHA-256", "content": "c7d8e9f0a1b2..." }]
    },
    {
      "type": "library",
      "bom-ref": "pkg:pypi/sqlalchemy@2.0.30",
      "name": "sqlalchemy",
      "version": "2.0.30",
      "purl": "pkg:pypi/sqlalchemy@2.0.30",
      "licenses": [{ "license": { "id": "MIT" } }],
      "hashes": [{ "alg": "SHA-256", "content": "d4e5f6a7b8c9..." }]
    },
    {
      "type": "library",
      "bom-ref": "pkg:pypi/starlette@0.37.2",
      "name": "starlette",
      "version": "0.37.2",
      "purl": "pkg:pypi/starlette@0.37.2",
      "licenses": [{ "license": { "id": "BSD-3-Clause" } }],
      "hashes": [{ "alg": "SHA-256", "content": "e1f2a3b4c5d6..." }]
    }
  ],
  "dependencies": [
    {
      "ref": "app-root",
      "dependsOn": [
        "pkg:pypi/fastapi@0.111.0",
        "pkg:pypi/uvicorn@0.29.0",
        "pkg:pypi/sqlalchemy@2.0.30"
      ]
    },
    {
      "ref": "pkg:pypi/fastapi@0.111.0",
      "dependsOn": ["pkg:pypi/starlette@0.37.2", "pkg:pypi/pydantic@2.7.1"]
    },
    { "ref": "pkg:pypi/uvicorn@0.29.0", "dependsOn": [] },
    { "ref": "pkg:pypi/sqlalchemy@2.0.30", "dependsOn": [] },
    { "ref": "pkg:pypi/starlette@0.37.2", "dependsOn": [] },
    { "ref": "pkg:pypi/pydantic@2.7.1", "dependsOn": [] }
  ]
}
```

With this file, it's possible evaluate each dependency against the [Common Vulnerabilities and Exposures (CVE)](https://www.cve.org/) database. There is a number of SaaS tools that do this effectively and suggest remediation in the form of a version bump or otherwise.

## Wrapping up

What I take away from picking this attack apart is that no single misconfiguration was catastrophic on its own. A `pull_request_target` workflow that checks out a fork, an incomplete secret rotation, mutable version tags, a CI runner that can talk to anywhere on the internet, each of these is something most teams have lived with at some point. The damage came from chaining them together, and the attacker was patient enough to wait three weeks between the initial credential theft and the tag poisoning.

That's the part that stuck with me. There is no single setting in Github that would've prevented this end to end. Pinning to commit SHAs would've stopped the tag poisoning but not the original `apidiff` exploit. Disabling post-install scripts wouldn't have helped a victim of the malicious binary. Short-lived OIDC credentials would've shrunk the blast radius but not closed the door. You need most of the counter measures above, most of the time, and you need to assume that one of them will fail.

The other thing worth saying is that the attacker here was a bot. [hackerbot-claw](https://www.linkedin.com/posts/cybersecurity-supplychainsecurity-githubactions-share-7433820997457469440-SUuo/) found the misconfigured workflow by scanning, not by manual reconnaissance. That changes the economics. A misconfiguration in a small project that a human attacker would never have bothered with is now reachable in minutes by something that doesn't sleep. This is something to keep in mind the next time you're thinking about leaving a "we'll fix that later" comment in a CI workflow yaml.

I already took action and eliminated most of the attack surface. If you've read this far, maybe you should too.
