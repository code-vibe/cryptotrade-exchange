# Build stage
FROM rustlang/rust:nightly-alpine AS builder

# Install dependencies for building
RUN apk add --no-cache musl-dev pkgconfig openssl-dev openssl-libs-static git bash

WORKDIR /app

# Copy workspace Cargo files (root + members)
COPY Cargo.toml Cargo.lock ./
COPY backend/api/Cargo.toml ./api/
COPY backend/core/Cargo.toml ./core/

# Create dummy src files to build deps
RUN mkdir -p api/src core/src && \
    echo "fn main() {}" > api/src/main.rs && \
    echo "fn main() {}" > core/src/lib.rs

# Build deps (cache layer)
RUN cargo build --release --bin cryptotrade-api || true
RUN rm -rf api/src core/src

# Copy actual source code
COPY backend/ ./backend/

# Build the final binary
RUN cargo build --release --bin cryptotrade-api

# Runtime stage
FROM alpine:latest

RUN apk add --no-cache ca-certificates libgcc

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/cryptotrade-api /app/

# Non-root user
RUN addgroup -g 1001 -S appgroup && \
    adduser -S appuser -u 1001 -G appgroup
USER appuser

EXPOSE 8080
CMD ["./cryptotrade-api"]
