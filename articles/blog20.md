# Home lab

I've been running home assistant on an Intel NUC for a while now and it's great. This small machine is very quiet, hardly uses 10% of its memory and even less CPU and doesn't consume energy significant enough to be a burden on the energy bill. But, the leftover memory and compute started to bug me. Surely there is better use for it. A field that I always like to explore is distributed computing. I've been working a lot with compute clusters, provisioning and configuring resources declaratively both for web applications and data heavy workloads. But, in a professional setting there is only so much you can take ownership of. When the goal is to deliver a project before a deadline, it makes more sense to split the work amongst people in the team. As a consequence, I found myself working on parts of the stack and usually the core infrastructure was already set up. Now that I had some left over resources on the premise (my house), I decided to setup some core infrastructure and over engineer the crap out of it. No deadlines, no responsibilities, just the physical limits of that Intel NUC (4 cores and 12GB of memory). 

## GitOps

At the start of a project it's very tempting to provision resources through the graphical interface of the infrastructure provider (e.g. AWS console). For most, this is intuitive, quick and sufficiently managable in the mid-term, especially if a project doesn't require fancy resources (e.g. simple lambda + API gateway stack). However, I'm a big advocate of infrastructure-as-code, like you might have inferred from my other blogs 😅, so for this project, I want every simple component to be [emphemeral](https://en.wikipedia.org/wiki/Ephemerality).

## Virtualization

I only have one machine, yet I'm looking to distribute workloads. How is this possible? Virtualization. For that reason, I booted [Proxmox](https://www.proxmox.com/en/) and setup a couple of virtual machines (VM). 
