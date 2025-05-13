FROM rust:latest as builder

WORKDIR /usr/src/note_taker

COPY Cargo.toml Cargo.lock ./

COPY ./src ./src

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libc6-dev \
    wget \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/note_taker

COPY --from=builder /usr/src/note_taker/target/release/note_taker .

EXPOSE 8080

CMD ["./note_taker"]