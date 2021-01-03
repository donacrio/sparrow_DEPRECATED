### Debian-based image for openssl compilation
FROM ekidd/rust-musl-builder as builder
ARG MODULE_NAME
RUN rustup self update
RUN rustup target add x86_64-unknown-linux-musl

# Build the binary
RUN mkdir sparrow
WORKDIR /home/rust/src/sparrow
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./ ./
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN chmod +x ./target/x86_64-unknown-linux-musl/release/${MODULE_NAME}

# Empty image to execute binary
FROM scratch
ARG MODULE_NAME

# Add the binary
COPY --from=builder /home/rust/src/sparrow/target/x86_64-unknown-linux-musl/release/${MODULE_NAME} ./app

# Add SSL certificates
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs
CMD ["./app"]
