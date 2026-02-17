# NC-Teltonika-Server (Rust Version)

This is a professional-grade Rust implementation of the Teltonika TCP Server. It is designed for high performance, robustness, and observability.

## Features

- **High Performance**: Asynchronous TCP handling with Tokio.
- **Robustness**: 
  - Connection limiting (default 5000 concurrent).
  - Graceful error handling and shutdown.
- **Observability**: 
  - Built-in **Health Check** and **Prometheus Metrics** endpoint (default port `9090`).
  - **Structured Logging** (JSON) for production environments.
- **Teltonika Protocol**: Full support for Codec 8 Extended and IMEI 2-stage handshake.
- **Database**: Efficient PostgreSQL storage with SSH Tunneling support.
- **Integration**: Webhook notifications to external APIs (Nauticoncept) and Microsoft Teams for critical errors.

## Prerequisites

- Rust (install via `rustup`)
- PostgreSQL database
- SSH client (if tunneling is used)

## Configuration

Copy `.env` from the project root or create one with the following variables:

```bash
# Server Configuration
HTTP_SERVER_PORT=6000
MONITOR_PORT=9090
FILE_DESCRIPTOR_LIMIT=10000

# Logging
RUST_LOG_FORMAT=text # Set to 'json' for structured logging
DEBUG=1

# Database
DB_HOST=localhost
DB_PORT=5432
DB_USER=postgres
DB_PASSWORD=yourpassword
DB_NAME=nauticoncept

# SSH Tunnel (Optional)
SSH_USER=user
SSH_HOST=host.com
SSH_PRIVATE_KEY_PATH=/path/to/key

# Webhooks
NAUTICONCEPT_API_URL=https://api.nauticoncept.com
```

## Running

### Development
1. **Build and Run**:
   ```bash
   cargo run
   ```

### Production Build
1. **Build Release**:
   ```bash
   cargo build --release
   ```
   The binary will be in `target/release/nc-teltonika-server`.

## Observability & Monitoring

The service exposes a dedicated HTTP server (default port `9090`) for monitoring:

-   **Health Check**: `GET /health`
    -   Returns `200 OK` if the service and database connection are healthy.
    -   Returns `503 Service Unavailable` if the database is unreachable.
-   **Metrics**: `GET /metrics`
    -   Returns Prometheus-formatted metrics:
        -   `tcp_connections_active`: Current number of TCP clients.
        -   `packets_received_total`: Total number of data packets processed.
        -   `db_query_duration_seconds`: Histogram of database insert times.

### Logging Recommendations
For production, set `RUST_LOG_FORMAT=json` in your `.env`. This outputs logs in a structured JSON format, making them easy to ingest into centralized logging systems like ELK or Grafana Loki.

## Service Installation (Debian/Ubuntu)

Since this is a Rust application, you need to compile it for release and install it manually as a systemd service.

### 1. Build release
```bash
cargo build --release
```

### 2. Prepare Directories
```bash
sudo mkdir -p /opt/nc-teltonika-server
sudo mkdir -p /var/log/nc-teltonika-server
# Set permissions (adjust user/group if not running as root)
sudo chown -R root:root /opt/nc-teltonika-server
sudo chmod 755 /var/log/nc-teltonika-server
```

### 3. Install Files
Copy the binary and environment file to the installation directory:
```bash
sudo cp target/release/nc-teltonika-server /opt/nc-teltonika-server/
sudo cp .env /opt/nc-teltonika-server/
sudo chmod +x /opt/nc-teltonika-server/nc-teltonika-server
```

### 4. Setup Systemd Service
Copy the provided service file to systemd:
```bash
sudo cp nc-teltonika-server.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable nc-teltonika-server
sudo systemctl start nc-teltonika-server
```

Check status and logs:
```bash
sudo systemctl status nc-teltonika-server
sudo journalctl -u nc-teltonika-server -f
```

### 5. Setup Log Rotation
To prevent logs from filling up the disk:
```bash
sudo cp nc-teltonika-server.logrotate /etc/logrotate.d/nc-teltonika-server
sudo chmod 644 /etc/logrotate.d/nc-teltonika-server
sudo chown root:root /etc/logrotate.d/nc-teltonika-server
# Test
sudo logrotate -d /etc/logrotate.d/nc-teltonika-server
```
