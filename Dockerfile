# Use a Rust base image
FROM rust:latest as builder

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Build the project dependencies (this step can be cached)
RUN cargo build --release

# Copy the source code to the container
COPY src ./src

# DB migrations
COPY migrations ./migrations

# Tailwind
COPY tailwind.config.js package.json yarn.lock ./

# Build the Rust project
RUN cargo build --release

# Create a smaller release image
FROM debian:buster-slim

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the built executable from the builder stage to the final image
COPY --from=builder /usr/src/app/target/release/webrs ./

# Copy articles to migrate
COPY articles ./articles

# Copy assets such as favicon
COPY assets ./assets

# Run the binary
CMD ["./webrs"]