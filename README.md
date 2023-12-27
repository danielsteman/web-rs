# web-rs

Let's build something with Axum, Askama, HTMX and Tailwind.

## Development

Run the server:

```
cargo install cargo-watch
cargo watch -x run
```

Run Tailwind (styling):

```
yarn install
yarn dev
```

[Open your browser](http://localhost:3000)

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

The corresponding database URL would be `postgresql://admin:admin@localhost/webrs` which is expected to be passed as environment variable `DATABASE_URL`. Just for development purposes, this variable can also be set in `./.env`.
