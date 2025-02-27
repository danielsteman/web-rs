# web-rs

Let's build a blog page with Axum, Askama, HTMX and Tailwind. I like to write articles in Markdown files, so I'm checking in markdown files in the repo and I parse and ingest them into a postgres database. The parser assumes that articles metadata is provided at the top of the document:

```
% id: 1
% title: Some blog title 🤖🧠
% date: 1970-01-01
% tags: ml, devops, rust
```

This project is deployed on serverless compute (AWS Lambda), using the Rust runtime. To make things easier, it uses `cargo-lambda` to [run, build and deploy](https://www.cargo-lambda.info/).

```bash
brew tap cargo-lambda/cargo-lambda
brew install cargo-lambda
```

## Development

Run Postgres locally:

```bash
docker run -d \
    --name webrs-postgres \
    -e POSTGRES_DB=webrs \
    -e POSTGRES_USER=admin \
    -e POSTGRES_PASSWORD=admin \
    -p 5432:5432 \
    postgres:latest
```

Run the server locally:

```bash
cargo lambda watch
```

Run Tailwind (styling):

```bash
yarn install
yarn dev
```

[Open your browser](http://localhost:3000)

The corresponding database URL would be `postgresql://admin:admin@localhost/webrs` which is expected to be passed as environment variable `DATABASE_URL`. Just for development purposes, this variable can also be set in `./.env`.

Likewise, OPENAI_API_KEY is set to generate summaries of the articles. This is only needed for initial ingestion.

## Build release

```bash
cargo lambda build --release
```

Or build Docker image and deploy wherever you like.

```bash
docker build -t webrs .
```

## Deploy

Compile and copy assets to target:

```bash
yarn prod
```

I'm using the [AWS Serverless Application Model (SAM)](https://aws.amazon.com/serverless/sam/) to deploy the Rust binary, along with assets, on S3 and AWS Lambda:

```bash
AWS_ACCESS_KEY_ID={...} AWS_SECRET_ACCESS_KEY={...} sam deploy --parameter-overrides DatabaseUrl="$DATABASE_URL"
```

Retrieve the API gateway domain name to route traffic from a custom domain name:

```bash
aws apigateway get-domain-names --region eu-central-1 | jq '.items.[0].regionalDomainName'
```

## Content delivery

Copy files to cloudfront s3 bucket

```bash
aws s3 cp ./cloudfront/ s3://danielsteman.com/ --recursive
```

Cloudfront is the AWS CDN solution and requires a certificate in us-east-1, since it's a global service that kind of makes sense.

Create a certificate in Cloudflare and import it in AWS ACM along with the [Cloudflare root certificate](https://developers.cloudflare.com/ssl/origin-configuration/origin-ca/#cloudflare-origin-ca-root-certificate) as chain certificate.

## Load testing

Install `artillery` globally with `npm install -g artillery@latest`.

## Troubleshooting

If the page is not loading, check if `sqlx` made a connection and didn't timeout. A timeout indicates that local Postgres is not up.

CockroachDB doesn't support locking the database prior to migrations. Hence, `sqlx::migrate!().set_locking(false)`.

CockroachDB is going to log warnings on startup due to `sqlx` migration queries that take time to execute. This is only on startup though, so it shouldn't be an issue.

## To do

- [ ] Use `aide`, or something like that, to generate openapi spec
- [ ] Render `shiki` code snippets on the server
- [ ] Render the tech radar on the server
- [ ] Prevent overfetching on data on /blogs
- [x] Get rid of the gaps in /blogs
- [x] Logically order /blogs
- [x] Add pagination for /blogs
- [x] Host somewhere
- [x] Responsive navbar
