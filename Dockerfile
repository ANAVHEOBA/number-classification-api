# Build stage
FROM rust:1.84.0-slim-bullseye as builder

WORKDIR /app
COPY . .

# Build dependencies - this is the caching Docker layer!
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/number-classification-api .

# Install necessary runtime libraries
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Expose the port
EXPOSE 8080

# Run the binary
CMD ["./number-classification-api"]