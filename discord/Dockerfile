# Use the official Bun image
FROM oven/bun:1-slim AS base

WORKDIR /usr/src/app

# Install system dependencies if needed
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy package files first for better dependency caching
COPY package.json bun.lock* ./

# Install dependencies
RUN bun install --frozen-lockfile --production

# Copy source code
COPY . .

# Create non-root user
RUN groupadd -r appuser && useradd -r -g appuser appuser
RUN chown -R appuser:appuser /usr/src/app
USER appuser

EXPOSE 3001

# Deploy commands and start the bot
CMD ["sh", "-c", "bun run deploy && bun run start"]
