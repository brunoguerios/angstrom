# syntax=docker.io/docker/dockerfile:1.7-labs

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

RUN apt-get update && apt-get -y upgrade && apt-get install -y libclang-dev pkg-config

FROM ghcr.io/foundry-rs/foundry:latest AS foundry
COPY . .

# Builds a cargo-chef plan
FROM chef AS planner
COPY --exclude=.git --exclude=dist . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

ARG BUILD_PROFILE=release
ENV BUILD_PROFILE=$BUILD_PROFILE

ARG RUSTFLAGS=""
ENV RUSTFLAGS="$RUSTFLAGS"

ARG FEATURES=""
ENV FEATURES=$FEATURES

RUN cargo chef cook --profile $BUILD_PROFILE --features "$FEATURES" --recipe-path recipe.json

RUN cargo build --profile $BUILD_PROFILE --features "$FEATURES" --locked --bin angstrom --manifest-path ./bin/angstrom/Cargo.toml


FROM ubuntu AS runtime
WORKDIR /app

COPY --from=builder /app/target/$BUILD_PROFILE/angstrom /app/
RUN chmod +x /app/angstrom

EXPOSE 30303 30303/udp 9001 8545 8546 
ENTRYPOINT ["/usr/local/bin/angstrom"]
