FROM denoland/deno:alpine

ARG DATABASE_URL 
ENV DATABASE_URL "file:/app/sqlite.db"

RUN mkdir -p /app
ADD target/release/jinjiang /usr/local/bin/jinjiang
ADD target/release/jinjiang-cli /usr/local/bin/jinjiang-cli

CMD ["sh", "-c", "jinjiang"]
