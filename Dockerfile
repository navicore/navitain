# Stage 1: Building
FROM rust:1.74 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin navitain
WORKDIR /navitain

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# This build step will cache our dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release
RUN rm -f ./target/release/deps/navitain*
RUN cargo build --release

# Stage 2: Production
FROM debian:bullseye-slim

# Install openssl
RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*

# Copy the binary to the production image from the builder stage
COPY --from=builder /navitain/target/release/navitain /usr/local/bin/navitain

# Set the startup command to run your binary
CMD ["navitain"]
