FROM rust:1.69.0 AS builder

WORKDIR /app

COPY src/ src/
COPY Cargo.toml .

RUN cargo build --release

FROM ubuntu:22.04

COPY --from=builder /app/target/release/testsomething /app/testsomething

CMD ["/app/testsomething"]