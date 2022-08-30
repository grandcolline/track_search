FROM rust:1.62 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/bootstrap /
COPY adapter/application/html/templates /adapter/application/html/templates
EXPOSE 8080
CMD ["./bootstrap"]
