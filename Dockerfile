# syntax=docker/dockerfile:1
FROM rust:1.81 AS env-build

# set work directory and copy source
WORKDIR /srv
COPY . /srv/

# build binary and verify checksum
RUN cargo build --release \
  && cp target/release/factorio-up /srv/factorio-up-glibc-amd64 \
  && sha256sum factorio-up-glibc-amd64 > SHA256SUMS
