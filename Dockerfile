FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo prisma generate && cargo install --path . && cargo build --release --all

FROM debian:bullseye-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/jinjiang /usr/local/bin/jinjiang
COPY --from=builder /usr/src/app/target/release/jinjiang-cli /usr/local/bin/jinjiang-cli
COPY --from=builder /usr/src/app/target/release/prisma-cli /usr/local/bin/prisma-cli

CMD ["jinjiang"]
