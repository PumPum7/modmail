FROM oven/bun:1

WORKDIR /usr/src/app

COPY package.json bun.lock .

RUN bun install

COPY . .

RUN bun run deploy
CMD ["bun", "run", "start"]
