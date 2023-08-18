FROM rust:latest AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo prisma generate \
    && cargo install --path . \
    && cargo build --release --all

FROM debian:bullseye-slim
ARG DATABASE_URL 
ENV DATABASE_URL "file:/app/sqlite.db"
RUN apt-get update && apt-get install -y curl cron && rm -rf /var/lib/apt/lists/* /etc/cron.*/*
COPY --from=builder /usr/src/app/target/release/jinjiang /usr/local/bin/jinjiang
COPY --from=builder /usr/src/app/target/release/jinjiang-cli /usr/local/bin/jinjiang-cli
