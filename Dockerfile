FROM rust:1.75.0-buster as builder
WORKDIR /app
COPY Cargo.lock /app/
COPY Cargo.toml /app/
COPY src /app/src/
COPY front /app/front/
COPY .sqlx /app/.sqlx
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/millennium_falcon .
ENTRYPOINT [ "./millennium_falcon" ]
