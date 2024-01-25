# Use Node base image to compile css with tailwind
FROM node:18 as styler

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy js stuff
COPY tailwind.config.js package.json yarn.lock ./
COPY templates/input.css ./templates/input.css

# Compile and minify css
RUN yarn install
RUN yarn prod

# Use a Rust base image
FROM rust:latest as builder

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Copy the source code to the container
COPY src ./src

# DB migrations
COPY migrations ./migrations

# Build the Rust project
RUN cargo build --release

# Create a smaller release image
FROM debian:buster-slim

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy css from styler
COPY --from=styler /usr/src/app/assets/output.css ./assets/output.css

# Copy the built executable from the builder stage to the final image
COPY --from=builder /usr/src/app/target/release/webrs ./

# Copy articles to migrate
COPY articles ./articles

# Copy assets such as favicon
COPY assets ./assets

# Run the binary
CMD ["./webrs"]