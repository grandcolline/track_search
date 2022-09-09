FROM rust:1.63 as builder
RUN apt update && apt install -y protobuf-compiler

WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/track-search /
COPY adapter/application/html/templates /adapter/application/html/templates
CMD ["./track-search"]
