FROM rust:1.69.0 AS builder

WORKDIR /app

COPY src/ src/
COPY Cargo.toml .

RUN cargo build --release

# FROM ubuntu:22.04
FROM debian:bookworm-slim
# 使用 scratch build 出來的 image 會有找不到檔案的問題
# FROM scratch

COPY --from=builder /app/target/release/testsomething /app/testsomething

CMD ["/app/testsomething"]