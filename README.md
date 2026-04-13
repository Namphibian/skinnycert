<img src="/assets/skinnycert.png" alt="Skinnycert Logo" width="200">

# Skinnycert

Industrial-strength TLS certificate API built in Rust.

Skinnycert provides a robust and efficient way to manage the lifecycle of TLS certificates. It handles private key generation, Certificate Signing Requests (CSR), and secure storage of certificates and their chains. Designed for high-performance and scalability, it leverages the power of Rust and OpenSSL.

## 🚀 Features

- **Multi-Algorithm Support**: Manage RSA and ECDSA key types (RSA key size registration and automated ECDSA curve discovery).
- **Certificate Lifecycle**:
    - Automatic private key and CSR generation.
    - Secure storage for CSRs, private keys, public keys, and signed certificates.
    - Support for Subject Alternative Names (SANs).
    - Tracking of certificate validity, expiry, and fingerprints.
- **Advanced Networking & Security**:
    - **mTLS Sidecar**: NGINX-based sidecar for mutual TLS termination.
    - **API Gateway**: Apache APISIX for routing, authentication, and header sanitization.
- **Observability**: Built-in health check endpoint with system memory monitoring (Linux-specific).
- **Industrial Strength**:
    - Built with **Actix-web** for high-performance asynchronous networking.
    - **SQLx** for safe and efficient PostgreSQL interactions.
    - **OpenSSL** for industry-standard cryptography.
    - **Tracing** with Bunyan formatting for structured logging.
- **Performance Testing**: Integrated Taurus (BZT) benchmarks for load testing.

## 🛠 Technology Stack

- **Language**: [Rust](https://www.rust-lang.org/) (Edition 2024)
- **Web Framework**: [Actix-web](https://actix.rs/)
- **Database**: [PostgreSQL](https://www.postgresql.org/) (16-alpine) with [SQLx](https://github.com/launchbadge/sqlx)
- **Cryptography**: [OpenSSL](https://www.openssl.org/)
- **API Gateway**: [Apache APISIX](https://apisix.apache.org/) (Standalone mode)
- **Sidecar Proxy**: [NGINX](https://www.nginx.com/)
- **Benchmarking**: [Taurus (BZT)](https://gettaurus.org/)
- **Logging**: [Tracing](https://tracing.rs/) with Bunyan formatting

## 📋 Requirements

- **Docker & Docker Compose**: Recommended for running the full stack.
- **Rust**: Version 1.80+ (required for Edition 2024) if building locally.
- **PostgreSQL**: Version 16+ (if running outside Docker).
- **OpenSSL Development Headers**: Required for local builds.
- **SQLx CLI**: `cargo install sqlx-cli` (optional, for migrations).

## 🏁 Getting Started

### Running with Docker Compose

The easiest way to start the complete Skinnycert stack is using the provided `compose.yaml`:

```bash
docker compose up -d
```

This will spin up:
- **Skinnycert API**: The core service (port `3000` internal).
- **NGINX Sidecar**: Handles mTLS termination (ports `80`, `8443`).
- **PostgreSQL**: Database for persistent storage.
- **APISIX Gateway**: Entry point for the stack (port `9443`).
- **Taurus**: Benchmarking container.

### Local Development

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/skinnycert.git
   cd skinnycert
   ```

2. **Set up the database**:
   You can use the PostgreSQL container from the Docker Compose stack:
   ```bash
   docker compose up -d postgres
   ```

3. **Configure environment variables**:
   Create a `.env` file or export variables (see [Configuration](#-configuration)).

4. **Run the API**:
   Use the watch script for hot-reloading:
   ```bash
   ./cargo-watch-run.sh
   ```

## 📜 Scripts

- `cargo-watch-run.sh`: Runs the application with `cargo-watch`, automatically re-running on file changes.
- `sqlx-prepare.sh`: Runs `cargo sqlx prepare` to generate the `.sqlx` metadata needed for offline SQLx compilation.
- `compile-release.sh`: Compiles the application in release mode with optimizations for `x86_64-unknown-linux-gnu`.

## ⚙️ Configuration (Environment Variables)

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | *Required* |
| `DB_MAX_CONNECTIONS` | Max database connections | `5` |
| `SERVER_ADDRESS` | IP address to bind to | `127.0.0.1` |
| `SERVER_PORT` | Port to listen on | `8080` |
| `WORKER_THREADS` | Number of worker threads | CPU Count |
| `RSA_KEY_MIN_SUPPORTED_SIZE` | Minimum RSA key size | `2048` |
| `RSA_KEY_MAX_SUPPORTED_SIZE` | Maximum RSA key size | `4096` |
| `RUST_LOG` | Logging level (info, debug, etc.) | `info` |

## 🧪 Testing

### Integration Tests
Run the Rust integration tests:
```bash
cargo test
```

### Manual Testing
Use [Slumber](https://slumber.lucaspickering.me/), a TUI HTTP client. The configuration is located in the `slumber/` directory.
```bash
slumber -f slumber/slumber.yml
```

### Benchmarks
Performance tests use Taurus (BZT). Run a test scenario using the container:
```bash
docker exec -it taurus bzt /bzt-configs/dev-test.yaml
```

## 📁 Project Structure

- `src/bin`: Application entry point (`skinnycert.rs`).
- `src/server`: Core server logic, routes, models, and configuration.
- `database`: SQL initialization scripts and migrations.
- `sidecar`: NGINX configuration for mTLS termination.
- `gateway`: Apache APISIX configuration (standalone mode).
- `benchmarks`: Taurus configuration and test artifacts.
- `slumber`: Slumber HTTP client collection for API testing.
- `tests`: Integration tests for the API.
- `certificates`: Directory for CA and mTLS certificates.
- `Dockerfile`: Multi-stage build for a distroless runtime image.
- `compose.yaml`: Full-stack Docker Compose configuration.
- `k8s_skinnycert.yaml`: Kubernetes deployment manifest.

## 📚 API Endpoints Summary

### Certificates
- `GET /certificates`: List all certificates (supports paging).
- `POST /certificates`: Create a new certificate (generates key & CSR).
- `GET /certificates/{id}`: Retrieve a specific certificate.
- `PUT /certificates/{id}`: Update/Upload signed certificate.
- `DELETE /certificates/{id}`: Delete a certificate.

### Key Algorithms
- `GET /keys`: List all supported key algorithms.
- `GET /keys/{id}`: Get algorithm details.
- `GET /keys/{id}/keypair`: Generate a key pair for the specified algorithm.

### Health Check
- `GET /health`: System health and memory status.

## TODOs
- [ ] Implement Swagger/OpenAPI documentation (dependencies present in `Cargo.toml`).
- [ ] Add more comprehensive ECDSA support tests.
- [ ] Automate certificate rotation logic.
- [ ] Implement robust pagination for all list endpoints.

## ⚖️ License

Distributed under the MIT License.
