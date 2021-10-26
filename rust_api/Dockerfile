# ARG BASE_IMAGE=ekidd/rust-musl-builder:latest
# FROM ${BASE_IMAGE} AS builder
# #FROM rust:1.49 as builder

# WORKDIR /usr/src/builder
# COPY . .
# CMD [ "ls", "-h" ]

#RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl


# FROM alpine:3.14.2

# RUN apk update
# #RUN apk add pkg-config libssl-dev openssl-dev

# COPY --from=builder /usr/src/builder/target/x86_64-unknown-linux-musl/release/rust_api /usr/local/bin/
# COPY --from=builder /usr/src/builder/Rocket.toml /usr/local/bin/Rocket.toml
# COPY --from=builder /usr/src/builder/config.toml /usr/local/bin/config.toml
# COPY --from=builder /usr/src/builder/Rocket.toml /usr/share/Rocket.toml
# COPY --from=builder /usr/src/builder/config.toml /usr/share/config.toml

# WORKDIR /usr/local/bin

# #RUN chmod +x /usr/local/bin/rust_api

# EXPOSE 8000

# CMD ["/usr/local/bin/rust_api"]
# #CMD ["ls", "-lisan"]









# Build Stage
FROM rust:latest as cargo-build
#RUN apt-get update && \
#    apt-get install musl-tools -y && \
#    rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/rust_api
COPY . .
#RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
RUN cargo build --release
# Final Stage
# FROM alpine:latest
FROM ubuntu:latest

#COPY --from=cargo-build /usr/src/rust_api/target/x86_64-unknown-linux-musl/release/rust_api /usr/local/bin/rust_api
COPY --from=cargo-build /usr/src/rust_api/target/release/rust_api /usr/local/bin/rust_api
COPY --from=cargo-build /usr/src/rust_api/config.toml /usr/share/config.toml
CMD [ "rust_api", "--config", "/usr/share/config.toml" ]
#CMD [ "top"]
