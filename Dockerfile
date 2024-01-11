FROM rust:1.75.0-buster as builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y clang

WORKDIR /app
COPY . /app/

RUN cargo build --release

FROM scratch
USER 1000
COPY --from=builder /app/target/release/millennium_falcon .
ENTRYPOINT [ "./millennium_falcon" ]
