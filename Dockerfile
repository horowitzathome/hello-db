# syntax=docker/dockerfile:1.3
# FROM rust:1.65.0 AS builder
FROM rust:alpine3.17 AS builder

ARG TARGETPLATFORM
ARG TARGET
ARG RUSTARGS

WORKDIR /root

RUN rustup update && rustup target add x86_64-unknown-linux-musl

RUN --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} cargo install cargo-strip

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} --mount=type=cache,target=/root/target,id=${TARGETPLATFORM} \
    cargo build --release --target ${TARGET} ${RUSTARGS} && \
    cargo strip && \
    mv /root/target/${TARGET}/release/hello-db /root

FROM gcr.io/distroless/static:nonroot

WORKDIR /hello-db

# Copy our build
COPY --from=builder /root/hello-db /hello-db/hello-db
# EXPOSE 8080
ENTRYPOINT ["/hello-db/hello-db"]