# syntax=docker/dockerfile:1.4

FROM rust:1.63 as builder
RUN apt update && apt install -y protobuf-compiler

WORKDIR /app
COPY --link . .
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --link --from=builder /app/target/release/track-search /
COPY adapter/application/html/templates /adapter/application/html/templates
CMD ["./track-search"]
