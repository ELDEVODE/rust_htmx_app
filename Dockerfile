# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
RUN USER=root cargo new --bin rust_htmx_app
WORKDIR /rust_htmx_app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Build the dependencies separately to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code to the container
COPY src ./src

RUN rm ./target/release/deps/rust_htmx_app*
# Build the application
RUN cargo build --release

FROM debian:buster-slim
COPY --from=build /rust_htmx_app/target/release/rust_htmx_app .
# Set the command to run the application
CMD ["cargo", "run", "--release"]