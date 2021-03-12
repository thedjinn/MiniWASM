FROM rust:1.50.0-buster

RUN set -ex; \
    apt-get update && \
    apt-get install --no-install-recommends -y binaryen wabt && \
    rustup target add wasm32-unknown-unknown && \
    mkdir -p /app

WORKDIR /app
