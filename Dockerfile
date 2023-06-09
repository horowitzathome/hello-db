# syntax=docker/dockerfile:1.3
FROM rust:1.65.0 AS builder
#FROM rust:alpine3.17 AS builder

ARG TARGETPLATFORM
ARG TARGET
ARG RUSTARGS

WORKDIR /root

RUN echo 'TARGETPLATFORM = '${TARGETPLATFORM}

# RUN apt-get update -y && apt-get install -y --no-install-recommends musl-dev

# RUN apk add --no-cache musl-dev=1.2.3-r4
# RUN rustup update && rustup target add x86_64-unknown-linux-musl

# RUN cargo install diesel_cli --no-default-features --features postgres

RUN --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} cargo install cargo-strip && cargo install diesel_cli --no-default-features --features postgres

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