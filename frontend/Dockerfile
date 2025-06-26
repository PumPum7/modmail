# Use Bun as the base image for building and running
FROM oven/bun:1 AS base
WORKDIR /app

# Install dependencies in a separate layer for better caching
FROM base AS deps
COPY package.json bun.lock* ./
RUN bun install --frozen-lockfile

# Build the application
FROM base AS builder
COPY --from=deps /app/node_modules ./node_modules
COPY . .

# Build the SvelteKit app
RUN bun run build

# Production image
FROM base AS runner
WORKDIR /app

ENV NODE_ENV=production

# Copy built application
COPY --from=builder /app/build ./build
COPY --from=builder /app/package.json ./
COPY --from=deps /app/node_modules ./node_modules

# Expose port
EXPOSE 5173

# Start the application
CMD ["bun", "run", "preview", "--host", "0.0.0.0", "--port", "5173"] 