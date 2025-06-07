# Build stage
FROM rust:1.82-slim as builder

WORKDIR /usr/src/app
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Copy the entire workspace structure
COPY Cargo.toml .
COPY backend-core backend-core
COPY solana-integration solana-integration
COPY solana-service solana-service

# Build the service
WORKDIR /usr/src/app
RUN cargo build --release --package solana-service

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/solana-service .

ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=3001
ENV RUST_LOG=info

EXPOSE 3001

CMD ["./solana-service"] 