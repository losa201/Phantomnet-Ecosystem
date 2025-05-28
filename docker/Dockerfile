FROM rust:1.75 AS builder
WORKDIR /build
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /build/target/release/phantomnetd /app/
COPY --from=builder /build/data /app/data
COPY docker/entrypoint.sh /app/
RUN chmod +x /app/entrypoint.sh
EXPOSE 3000
ENTRYPOINT ["/app/entrypoint.sh"]