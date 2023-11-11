FROM rust:1.73.0 as builder

WORKDIR /usr/src/hyper-test
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo install --path .

FROM ubuntu:latest

COPY --from=builder /usr/local/cargo/bin/hyper-test /usr/local/bin/hyper-test

CMD ["hyper-test"]
