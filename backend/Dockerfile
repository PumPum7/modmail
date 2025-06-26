FROM rust:1.87.0

WORKDIR /usr/src/app

COPY . .

RUN cargo install --path .

RUN cargo install sqlx-cli

CMD ["sh", "-c", "sqlx database setup && backend"]

