# web-rs

Let's build a blog page with Axum, Askama, HTMX and Tailwind. I like to write articles in Markdown files, so I'm checking in markdown files in the repo and I parse and ingest them into a postgres database. The parser assumes that articles metadata is provided at the top of the document:

```
% id: 1
% title: Some blog title 🤖🧠
% date: 1970-01-01
% tags: ml, devops, rust
```

## Development

Run Postgres locally:

```
docker run -d \
    --name webrs-postgres \
    -e POSTGRES_DB=webrs \
    -e POSTGRES_USER=admin \
    -e POSTGRES_PASSWORD=admin \
    -p 5432:5432 \
    postgres:latest
```

Run the server:

```
cargo install cargo-watch
cargo watch -x "run --bin web-rs"
```

Run Tailwind (styling):

```
yarn install
yarn dev
```

[Open your browser](http://localhost:3000)

The corresponding database URL would be `postgresql://admin:admin@localhost/webrs` which is expected to be passed as environment variable `DATABASE_URL`. Just for development purposes, this variable can also be set in `./.env`.

Likewise, OPENAI_API_KEY is set to generate summaries of the articles. This is only needed for initial ingestion.

## Build release

Build Docker image and deploy wherever you like.

```
docker build -t webrs .
```

## Troubleshooting

If the page is not loading, check if `sqlx` made a connection and didn't timeout. A timeout indicates that Postgres is not up.

CockroachDB doesn't support locking the database prior to migrations. Hence, `sqlx::migrate!().set_locking(false)`.

## To do

- [ ] Use `aide`, or something like that, to generate openapi spec
- [ ] Render `shiki` code snippets on the server
- [ ] Render the tech radar on the server
- [x] Get rid of the gaps in /blogs
- [x] Logically order /blogs
- [ ] Add pagination for /blogs
- [ ] Host somewhere
- [x] Responsive navbar
