FROM debian:bullseye-slim
ARG DATABASE_URL 
ENV DATABASE_URL "file:/app/sqlite.db"
RUN apt-get update && apt-get install -y curl cron sqlite3 && rm -rf /var/lib/apt/lists/* /etc/cron.*/*
COPY target/release/jinjiang /usr/local/bin/jinjiang
COPY target/release/jinjiang-cli /usr/local/bin/jinjiang-cli

COPY docker/crontab /etc/cron.d/jinjiang
RUN chmod 0644 /etc/cron.d/jinjiang
RUN crontab /etc/cron.d/jinjiang
RUN touch /var/log/cron.log
