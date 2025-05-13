FROM rust:1.70 as builder

WORKDIR /usr/src/note_taker

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/note_taker

COPY --from=builder /usr/src/note_taker/target/release/note_taker .

EXPOSE 8080

CMD ["./note_taker"]