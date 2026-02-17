# Build stage
FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install OpenSSL - required for many Rust crates (like reqwest, sqlx)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/nc-teltonika-server /usr/local/bin/nc-teltonika-server

CMD ["nc-teltonika-server"]
