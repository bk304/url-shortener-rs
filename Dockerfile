###### Stage 1: Build the application
FROM rust:latest as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src/main.rs ./src/main.rs
RUN cargo fetch

COPY .sqlx ./.sqlx
COPY migrations ./migrations
COPY src ./src

RUN cargo build --release

###### Stage 2: Final image
FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/url-shortener-rs .

CMD ["./url-shortener-rs"]
EXPOSE 3000
