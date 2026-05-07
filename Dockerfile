# Dockerfile for building Azaka contracts

FROM rust:1.81.0 as builder

# Install wasm target
RUN rustup target add wasm32-unknown-unknown

# Install Stellar CLI
RUN cargo install --locked stellar-cli

WORKDIR /app

# Copy workspace files
COPY Cargo.toml rust-toolchain.toml ./
COPY contracts/ ./contracts/

# Build contracts
RUN cargo build --target wasm32-unknown-unknown --release

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/wasm32-unknown-unknown/release/*.wasm /contracts/
COPY --from=builder /usr/local/cargo/bin/stellar /usr/local/bin/

WORKDIR /contracts

CMD ["ls", "-lh"]
