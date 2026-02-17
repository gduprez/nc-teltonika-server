#!/bin/bash
# Script to build the release binary for Linux (x86_64) using Docker.
# This avoids "Exec format error" by compiling in a compatible environment.

echo "Building for Linux (x86_64)..."
docker run --platform linux/amd64 --rm \
  -u "$(id -u):$(id -g)" \
  -v "$(pwd)":/usr/src/myapp \
  -w /usr/src/myapp \
  -e CARGO_TARGET_DIR=target_linux \
  rust:latest \
  cargo build --release

if [ $? -eq 0 ]; then
  echo "Build successful!"
  echo "Binary location: target_linux/release/nc-teltonika-server"
else
  echo "Build failed."
  exit 1
fi
