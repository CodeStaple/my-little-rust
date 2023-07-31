# Start from the official Rust image so we don't have to install Rust manually
FROM rust:1.56 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin simple_api
WORKDIR /simple_api

# Copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy your source tree
COPY ./src ./src

# Build for release. 
# This will compile your application and create an executable binary under the target/release directory.
RUN cargo build --release

# Our second stage will use the Debian image
FROM debian:buster-slim

# Install OpenSSL, needed for our application
RUN apt-get update \
    && apt-get install -y ca-certificates libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder to this new stage
COPY --from=builder /simple_api/target/release/simple_api /usr/local/bin/

# Run the binary
CMD ["simple_api"]

