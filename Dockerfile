# syntax=docker/dockerfile:1
FROM rust:1.81 AS env-build

# set work directory and copy source
WORKDIR /srv
COPY . /srv/

# cache dependencies and objects and build binary
RUN \
  --mount=type=cache,id=cargo-cache,target=/usr/local/cargo/registry \
  --mount=type=cache,id=cargo-cache,target=/srv/target \
  cargo build --release \
  && cp target/release/factorio-up /srv/factorio-up
