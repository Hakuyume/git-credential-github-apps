FROM rust:1.74.0-bookworm

RUN apt-get update && apt-get install --yes musl-tools
RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.lock Cargo.toml /tmp/
COPY src/ /tmp/src/
RUN cd /tmp/ && cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.18

RUN apk add --no-cache ca-certificates git git-lfs
COPY --from=0 /tmp/target/x86_64-unknown-linux-musl/release/git-credential-github-apps /usr/local/bin/
