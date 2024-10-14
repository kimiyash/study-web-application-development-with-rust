# マルチステージビルドを使用し、rust のプログラムをビルドする
FROM rust:1.78-slim-bookworm AS builder
WORKDIR /app

ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

COPY . .
RUN cargo build --release

# 不要なソフトウェアを同期する必要はないので、軽量な bookworm-slim を利用する。
FROM debian:bookworm-slim
WORKDIR /app

# 後続の説明で利用するため、ユーザーを作成しておく
RUN adduser book && chown -R book /app
USER book
COPY --from=builder ./app/target/release/app ./target/release/app

# 8080番ポートを開放し、アプリケーションを起動する
ENV PORT 8080
EXPOSE $PORT
ENTRYPOINT ["./target/release/app"]