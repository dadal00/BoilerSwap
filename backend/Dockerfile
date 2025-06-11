FROM rust:1-alpine3.21@sha256:fa7c28576553c431224a85c897c38f3a6443bd831be37061ab3560d9e797dc82

ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN apk add --no-cache musl-dev openssl-dev pkgconfig

WORKDIR /app
COPY ./ /app

RUN cargo build --release
RUN strip target/release/backend

FROM alpine:3.21@sha256:a8560b36e8b8210634f77d9f7f9efd7ffa463e380b75e2e74aff4511df3ef88c

RUN apk add --no-cache libgcc

COPY --from=0 /app/target/release/backend .
CMD ["/backend"]
