### Module image used to package a built binary from the build image

ARG BUILDER_IMAGE=sparrow/sparrow-builder
# Sparrow base image containing all built binaries
FROM ${BUILDER_IMAGE} as builder
ARG MODULE_NAME

RUN chmod +x ./target/x86_64-unknown-linux-musl/release/${MODULE_NAME}

# Empty image to execute binary
FROM scratch
ARG MODULE_NAME

COPY --from=builder /home/rust/src/sparrow/target/x86_64-unknown-linux-musl/release/${MODULE_NAME} ./app
COPY --from=builder /home/rust/src/sparrow/.env ./.env
# Add SSL certificates
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs

ENTRYPOINT ["./app"]
