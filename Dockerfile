# Builder stage
FROM rust:1.75 as builder

RUN USER=root cargo new --bin rstnt_api
WORKDIR /rstnt_api

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

ARG PORT
ARG DATABASE_URL

ENV PORT $PORT
ENV DATABASE_URL $DATABASE_URL

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY ./migrations ./migrations
COPY ./.sqlx ./.sqlx

RUN cargo install sqlx-cli

RUN rm -f ./target/release/deps/rstnt_api*
RUN cargo build --release

# Final stage
FROM debian:buster-slim

RUN apt-get update && apt-get install -y \
    openssl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /rstnt_api/target/release/rstnt_api /app/rstnt_api


EXPOSE $PORT

CMD ["./rstnt_api"]
