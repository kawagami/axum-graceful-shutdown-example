FROM rust:1.69.0 AS builder

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

COPY src/ src/
COPY Cargo.toml .

RUN cargo build --target x86_64-unknown-linux-musl --release

# 修復 scratch 找不到檔案的問題
# https://kerkour.com/rust-small-docker-image#from-scratch
# 以上網址有說原因
FROM scratch

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/testsomething /app/testsomething

CMD ["/app/testsomething"]