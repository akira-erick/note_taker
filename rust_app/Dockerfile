FROM rust:latest as builder

WORKDIR /usr/src/note_taker

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /usr/src/note_taker

COPY --from=builder /usr/src/note_taker/target/release/note_taker .

EXPOSE 8080

CMD ["./note_taker"]