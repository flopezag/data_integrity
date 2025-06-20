# -------- Stage 1: Build --------
FROM rust:1.76 as builder

# Create app directory
WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY build.rs ./
COPY doc ./doc
COPY openapi.rs ./src/

# Build in release mode
RUN cargo build --release

# -------- Stage 2: Runtime --------
FROM debian:bookworm-slim

# Install minimal libs required for Rust binaries and upgrade all packages to fix vulnerabilities
RUN apt-get update && apt-get upgrade -y && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m appuser

# Set working directory
WORKDIR /app

# Copy compiled binary from builder
COPY --from=builder /app/target/release/ngsild-signer .

# Use non-root user
USER appuser

# Expose the Axum port
EXPOSE 3000

# Start the service
CMD ["./ngsild-signer"]
