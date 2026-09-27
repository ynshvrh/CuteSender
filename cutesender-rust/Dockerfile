# Stage 1: Build the application
FROM rust:slim AS builder

WORKDIR /app

# Cache dependencies by compiling an empty stub project
COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy actual source code and compile release binary
COPY src ./src
RUN touch src/main.rs && cargo build --release

# Stage 2: Minimal runtime image
FROM debian:bookworm-slim

# Install root certificates for outbound HTTPS requests
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/cutesender-rust /app/cutesender-rust

ENV PORT=3000
EXPOSE 3000

CMD ["/app/cutesender-rust"]
