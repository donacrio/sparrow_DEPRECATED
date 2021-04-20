### Build dockerfile used to compile binaries on x86_64-unknown-linux-musl target

# Debian-based image for openssl compilation
FROM ekidd/rust-musl-builder as builder
RUN rustup target add x86_64-unknown-linux-musl

# Build the binary
RUN mkdir sparrow
WORKDIR /home/rust/src/sparrow
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./ ./
RUN cargo build --target x86_64-unknown-linux-musl --release
