FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libssl3 sqlite3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY target/release/backend /app/backend
COPY frontend/dist/ /app/dist/

ENV DATABASE_URL="sqlite:/app/data/data.db?mode=rwc"
ENV DIST_DIR="/app/dist"

RUN mkdir -p /app/data

EXPOSE 8080

CMD ["/app/backend"]
