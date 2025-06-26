FROM oven/bun

WORKDIR /app
COPY package.json package.json
RUN bun install

COPY . .
# COPY env_prod .env
RUN bun run build

EXPOSE 5173
ENTRYPOINT ["sh", "-c"]
CMD ["PROTOCOL_HEADER=x-forwarded-proto HOST_HEADER=x-forwarded-host bun build/index.js"]