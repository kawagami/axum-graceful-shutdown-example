FROM rust:1.74.0-slim-bookworm AS builder

WORKDIR /app

COPY src/ src/
COPY Cargo.toml .

RUN cargo build --release

RUN strip -s /app/target/release/testsomething

# 好像 sqlx 有用到不能用 scratch 的依賴
FROM ubuntu:22.04

COPY --from=builder /app/target/release/testsomething /app/testsomething

CMD ["/app/testsomething"]