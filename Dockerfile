FROM rust:1.74.0-slim-bookworm AS builder

WORKDIR /app

COPY src/ src/
COPY Cargo.toml .

RUN cargo build --release

RUN strip -s /app/target/release/testsomething

FROM ubuntu:22.04

COPY --from=builder /app/target/release/testsomething /app/testsomething

CMD ["/app/testsomething"]