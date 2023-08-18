FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path . && cargo prisma generate && cargo build --release --all

FROM debian:stable
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/jinjiang /usr/local/bin/jinjiang
COPY --from=builder /usr/src/app/target/release/jinjiang-cli /usr/local/bin/jinjiang-cli
COPY --from=builder /usr/src/app/target/release/prisma-cli /usr/local/bin/prisma-cli

CMD ["jingjiang"]
