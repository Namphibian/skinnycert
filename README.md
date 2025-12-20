# Skinnycert

Industrial-strength TLS certificate API built in Rust.

Skinnycert provides a robust and efficient way to manage the lifecycle of TLS certificates. It handles private key generation, Certificate Signing Requests (CSR), and secure storage of certificates and their chains. Designed for high-performance and scalability, it leverages the power of Rust and OpenSSL.

## 🚀 Features

- **RSA Key Management**: Define and manage RSA key sizes (algorithms) and generate secure RSA key pairs.
- **Certificate Lifecycle**:
    - Automatic private key and CSR generation.
    - Secure storage for CSRs, private keys, public keys, and signed certificates.
    - Support for Subject Alternative Names (SANs).
    - Tracking of certificate validity, expiry, and fingerprints.
- **ECDSA Support**: Foundation for Elliptic Curve Digital Signature Algorithm (ECDSA) support, including automatic curve discovery from OpenSSL.
- **Observability**: Built-in health check endpoint with system memory monitoring (Linux-specific).
- **Industrial Strength**:
    - Built with **Actix-web** for high-performance asynchronous networking.
    - **SQLx** for safe and efficient PostgreSQL interactions.
    - **OpenSSL** for industry-standard cryptography.
    - **Tracing** with Bunyan formatting for structured logging.

## 🛠 Technology Stack

- **Language**: [Rust](https://www.rust-lang.org/)
- **Web Framework**: [Actix-web](https://actix.rs/)
- **Database**: [PostgreSQL](https://www.postgresql.org/) with [SQLx](https://github.com/launchbadge/sqlx)
- **Cryptography**: [OpenSSL](https://www.openssl.org/)
- **Serialization**: [Serde](https://serde.rs/)
- **Logging**: [Tracing](https://tracing.rs/)

## 🏗 Architecture Overview

Skinnycert is designed to be deployed in a secure environment, often behind a sidecar proxy for mTLS termination.

- **API Server**: The core Rust application handling certificate logic.
- **Database**: PostgreSQL for persistent storage of keys and certificates.
- **Sidecar**: Nginx-based sidecar for mTLS/TLS termination (optional but recommended).
- **Gateway**: APISIX as an entry point for the stack.

## 🏁 Getting Started

### Prerequisites

- [Docker](https://www.docker.com/) and [Docker Compose](https://docs.docker.com/compose/)
- [Rust](https://www.rust-lang.org/tools/install) (for local development)
- [PostgreSQL](https://www.postgresql.org/) (if running outside Docker)

### Running with Docker Compose

The easiest way to start Skinnycert is using the provided `compose.yaml`:

```bash
docker-compose up -d
```

This will spin up:
- The Skinnycert API on port `3000` (internal) and `8443` (mTLS).
- A PostgreSQL database.
- An Nginx sidecar for TLS termination.
- An APISIX gateway on port `9443`.

### Configuration

Configuration is managed via environment variables. You can use a `.env` file for local development.

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | *Required* |
| `SERVER_ADDRESS` | IP address to bind to | `127.0.0.1` |
| `SERVER_PORT` | Port to listen on | `8080` |
| `WORKER_THREADS` | Number of worker threads | Number of CPUs |
| `DB_MAX_CONNECTIONS`| Max database connections | `5` |
| `RUST_LOG` | Logging level (info, debug, etc.) | `info` |

## 📚 API Endpoints Summary

### Certificates
- `GET /certificates`: List all active certificates.
- `POST /certificates`: Create a new certificate (generates key & CSR).
- `GET /certificates/{id}`: Retrieve a specific certificate.
- `PATCH /certificates/{id}`: Upload a signed certificate and chain.
- `DELETE /certificates/{id}`: Soft delete a certificate.

### RSA Keys
- `GET /keys/rsa`: List all RSA key algorithms.
- `POST /keys/rsa`: Register a new RSA key size.
- `GET /keys/rsa/{id}`: Get RSA key algorithm details.
- `POST /keys/rsa/{id}/keypair`: Generate an RSA key pair of the specified size.

### Health Check
- `GET /health`: System health and memory status.

## 📁 Project Structure

- `src/bin`: Application entry point.
- `src/server`: Core server logic, routes, and models.
- `database`: SQL initialization and migration scripts.
- `sidecar`: Nginx configuration for mTLS.
- `gateway`: APISIX configuration.
- `benchmarks`: Performance testing configuration (Taurus).
- `slumber`: HTTP client configuration for testing the API.
- `k8s_skinnycert.yaml`: Kubernetes deployment manifest.

## ⚖️ License

Distributed under the MIT License.
