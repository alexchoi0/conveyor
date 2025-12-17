FROM rust:1.75-bookworm AS builder

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Build the operator
RUN cargo build --release -p etl-operator

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/etl-operator /usr/local/bin/etl-operator

ENTRYPOINT ["/usr/local/bin/etl-operator"]
