FROM rust:1.55 as builder
ENV USER root
COPY . .
WORKDIR /usr/bin/
RUN cargo build


FROM debian:buster-slim
WORKDIR /usr/bin/target/debug/
RUN ls -h

COPY --from=builder . .
CMD [ "ls" ]
