ARG BASE_IMAGE=ekidd/rust-musl-builder:latest
FROM ${BASE_IMAGE} AS builder
#FROM rust:1.49 as builder

COPY --chown=rust:rust . ./

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl


FROM alpine:3.14.2

RUN apk update
#RUN apk add pkg-config libssl-dev openssl-dev

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/rust_api /usr/local/bin/
WORKDIR /usr/local/bin

#RUN chmod +x /usr/local/bin/rust_api

CMD ["/usr/local/bin/rust_api"]
#CMD ["ls", "-lisan"]
