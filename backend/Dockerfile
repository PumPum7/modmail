# Build stage
FROM rust:1.87.0-slim AS builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./

# Pre-fetch dependencies to leverage Docker cache without compiling the full project
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
# Fetch dependencies
RUN cargo fetch
# Clean dummy src
RUN rm -rf src

# Copy source code and migrations
COPY src ./src
COPY migrations ./migrations

# Build the actual application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies including curl for health checks
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary and sqlx-cli from builder stage
COPY --from=builder /usr/src/app/target/release/backend /app/backend
COPY --from=builder /usr/src/app/migrations /app/migrations

# Make the binary executable
RUN chmod +x /app/backend

# Create non-root user
RUN useradd -r -s /bin/false appuser
RUN chown -R appuser:appuser /app
USER appuser

EXPOSE 8080

# Create a startup script that waits for DB and handles errors properly
USER root
RUN echo '#!/bin/bash\n\
set -e\n\
echo "Waiting for database to be ready..."\n\
while ! pg_isready -h db -p 5432 -U $POSTGRES_USER; do\n\
  sleep 2\n\
done\n\
echo "Database is ready, running migrations..."\n\
diesel migration run || {\n\
  echo "Migration failed, but continuing..."\n\
}\n\
echo "Starting backend..."\n\
exec ./backend\n\
' > /app/start.sh && chmod +x /app/start.sh

# Install postgresql-client for pg_isready
RUN apt-get update && apt-get install -y postgresql-client && rm -rf /var/lib/apt/lists/*

USER appuser

CMD ["/app/start.sh"]
