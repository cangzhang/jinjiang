FROM debian:bullseye-slim
ARG DATABASE_URL 
ENV DATABASE_URL "file:/app/sqlite.db"
RUN apt-get update && apt-get install -y curl sqlite3 && rm -rf /var/lib/apt/lists/*
ADD target/release/jinjiang /usr/local/bin/jinjiang
ADD target/release/jinjiang-cli /usr/local/bin/jinjiang-cli

CMD ["sh", "-c", "jinjiang"]
