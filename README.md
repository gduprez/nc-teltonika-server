# NC-Teltonika-Server (Rust Version)

This is a Rust implementation of the Teltonika TCP Server, ported from the original Node.js version.

## Prerequisites

- Rust (install via `rustup`)
- PostgreSQL database
- SSH client (if tunneling is used)

## Configuration

Copy `.env` from the project root or create one with the following variables:

```
HTTP_SERVER_PORT=6000
DEBUG=1
DB_HOST=localhost
DB_PORT=5432
DB_USER=postgres
DB_PASSWORD=yourpassword
DB_NAME=nauticoncept
SSH_USER=user
SSH_HOST=host.com
SSH_PRIVATE_KEY_PATH=/path/to/key
NAUTICONCEPT_API_URL=https://api.nauticoncept.com
```

## Running

1. **Build and Run**:
   ```bash
   cargo run
   ```

2. **Build Release**:
   ```bash
   cargo build --release
   ```
   The binary will be in `target/release/nc-teltonika-server`.

## Features

- TCP Server listening on configured port.
- Parses Teltonika AVL Data (Codec 8 Extended supported).
- Handles IMEI handshake.
- Saves data to PostgreSQL database.
- Supports SSH Tunneling for database connection.
- Sends Webhook notifications to Nauticoncept API.
- Sends error notifications to Microsoft Teams.
