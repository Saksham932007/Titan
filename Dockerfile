# Multi-stage build for production-ready Titan web server

# Stage 1: Build
FROM rust:1.75 as builder

WORKDIR /usr/src/titan

# Copy manifest files
COPY Cargo.toml ./

# Copy source code
COPY src ./src

# Build release binary
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /usr/src/titan/target/release/titan /app/titan

# Copy public directory (create it if needed)
RUN mkdir -p /app/public
COPY public /app/public

# Set environment variables
ENV IP=0.0.0.0
ENV PORT=8080

# Expose port
EXPOSE 8080

# Run the binary
CMD ["/app/titan"]
