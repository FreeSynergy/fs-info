# fs-info — library crate, no binary
# This Containerfile is a placeholder for future CLI/daemon builds.
# Current use: cargo build --release (library only)
FROM docker.io/rust:1.83-slim AS builder

WORKDIR /build

COPY fs-libs/ fs-libs/
COPY fs-info/ fs-info/

WORKDIR /build/fs-info
RUN cargo build --release
