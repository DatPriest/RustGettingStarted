FROM rust:1.49 as builder

RUN apt-get update && \
    apt-get install musl-tools -y && \
    rustup target add x86_64-unknown-linux-musl


WORKDIR /usr/src/task

ENV USER root

COPY . .
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl


FROM alpine:3.14.2 as testing

COPY --from=builder /usr/src/task/target/x86_64-unknown-linux-musl/release/testing /usr/local/bin/testing
WORKDIR /usr/local/bin

ENV USER root

RUN chmod 777 testing

CMD ["testing"]
