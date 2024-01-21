% id: 8
% title: SaaS-starter wrapped in a monorepo with microservices
% date: 2023-06-20
% tags: web

Those who are in the business of software development might be familiar with the phenomenon of people coming up to you with a business pitch, asking you for a technical implementation. I always admire entrepreneurship and when I can, I try to help out. After speaking to several people with several ([SaaS](https://en.wikipedia.org/wiki/Software_as_a_service)) ideas, I realised that implementing these ideas would involve common components, such as user authentication, a billing system and a [CRM](https://en.wikipedia.org/wiki/Customer_relationship_management), to name a view.

This problem is well known and "SaaS-starters" are widely available, either [paid](https://supastarter.dev/) or [open-source](https://github.com/Blazity/next-saas-starter). My motivation to build my own is to take on the challenge of building a large(r)-scale web application and to understand the project throughout, so that implementating the starter should be easy later on. In retrospect, it also helped me to understand alternative starters better.

To make things easy I went for a Typescript stack: backend services with [Express](https://expressjs.com/) and a frontend with [React](https://react.dev/). There are several arguments for and against these frameworks, I chose them because they're familiar and I wanted to build something with relative speed. One advantage of using Typescript in either frontend and backend is that I'm able to share types across them, which is pretty neet in a client-server architecture. Pro tip: check out [path mapping](https://www.typescriptlang.org/docs/handbook/module-resolution.html#path-mapping).

With the anticipation that a SaaS project can become complex quickly, I opted for a micro services architecture, where the backend in split in several services where each service is separated from the other based on their responsibilities. I also decided to place them in a single repository. This approach also goes by the alias of [monorepo](https://en.wikipedia.org/wiki/Monorepo). Despite the fact that services are not isolated in separate repositories, it should still be convenient to collaborate, since each service has its own depedencies and build scripts (e.g. `package.json` and `Dockerfile`s). The advantage of a monorepo, especially at the start of a project, is that it's easy to navigate through and allows for [atomic commits](https://en.wikipedia.org/wiki/Atomic_commit). At work, we keep services in dedicated repositories, each which its own CI/CD pipeline Terraform components. This has advantages when you're scaling, but also increases overhead since there are more than twenty pipelines that need to be green and because each repository requires boilerplate code that gets outdated quickly. Hence, I wanted to experience the alternative: monorepo. The project structure looks something like this:

```bash
├── README.md
├── infrastructure
│   ├── firebase_auth.tf
│   ├── firebase_project.tf
│   ├── firebase_storage.tf
│   ├── gcp_project.tf
│   ├── gke.tf
│   ├── main.tf
│   └── variables.tf
├── packages
│   ├── chat
│   │   ├── node_modules
│   │   ├── package.json
│   │   ├── src
│   │   ├── swagger.yaml
│   │   ├── tsconfig.json
│   │   └── yarn.lock
│   ├── documents
│   │   ├── node_modules
│   │   ├── package.json
│   │   ├── src
│   │   ├── swagger.yaml
│   │   ├── tsconfig.json
│   │   └── yarn.lock
│   └── web
│       ├── Dockerfile
│       ├── index.html
│       ├── nginx
│       ├── node_modules
│       ├── package.json
│       ├── public
│       ├── src
│       ├── tsconfig.json
│       ├── vite.config.ts
│       └── yarn.lock
```

In `/infrastructure` I keep all Terraform code to deploy infrastructure that hosts the services. Terraform really shines in this template repository, because it makes the infrastructure that this project runs on reproducable and requires just a couple of parameters and manual actions (just as signing up for Terraform Cloud, which is completely free by the way).

Each service (in `/packages`) can be build and deployed independently. You might wonder if this wouldn't cause the CI pipeline to do redundant work (such as the lenghty process of building Docker images) everytime it runs. In Github Actions there is small trick to overcome this issue, using `paths`. For example:

```yaml
on:
  push:
    paths:
      - "web/**"
```

Alongside the Express services, I use [Firebase](https://firebase.google.com/) services, which makes it really easy to integrate authentication and a documents database using the [client library](https://firebase.google.com/docs/firestore/client/libraries) in the frontend package (`web`), a Typescript React app created with Vite in this project. By reusing [Chakra UI](https://chakra-ui.com/) components it's easy to put Firebase to work, for example through the Modal that contains a login screen and an option to authenticate with Google.

![Landing page](../assets/images/saas-starter-0.png)

![Sign up page](../images/saas-starter-1.png)

For the next iteration I'm planning to integrate [Stripe](https://stripe.com/) to handle the payment checkout. The biggest challenge would be to align the user database in Firebase with the customer database in Stripe. Luckily, this is made easy by the [Firebase-provided cloud functions](https://firebase.google.com/docs/tutorials/payments-stripe).

By now, I started with an implementation of this starter, in another repository, and another issue became apparent. As I'm working on an implementation I start to notice that I'm developing features or fixes that are still relevant for the starter. Thus far, I resolved this by simply merging relevant changes back into the upstream, but this requires caution because it's easy to merge changes that belong to the implementation and not to the starter. In another iteration I'm planning to test if [Git Submodules](https://git-scm.com/book/en/v2/Git-Tools-Submodules) are of any use in this context.
