FROM rust:1.49-alpine3.11

WORKDIR /mnt/x/Dev/Projects/RustGettingStarted/rust_api
COPY . .

RUN cargo install --path /var/www/

CMD [ "cargo", "run" ]
