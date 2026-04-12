# API Six Gateway Configuration (Local Development)

The `apisix.yaml` file configures **APISIX** in **standalone mode**, primarily for local development and testing. This setup enables quick iteration and debugging while using mTLS and HTTP authentication.

## Standalone Mode

To run APISIX in standalone mode, set the environment variable:

```bash
APISIX_STAND_ALONE=true
```

In this setup, the `apisix.yaml` file is mounted into the Docker container at:

```
/usr/local/apisix/conf/apisix.yaml
```

Changes made to this file **while the container is running** are automatically detected, and APISIX reloads the configuration dynamically—no container restart is required.

For full reference, see the [APISIX Documentation](https://apisix.apache.org/docs/apisix/getting-started/README/).

## Key Configuration Features

1. **Global Rules (Header Sanitization)**
    - Removes headers like `Server` from responses.
    - Adds `X-Content-Type-Options: nosniff` and `Content-Type: application/json` for security.

2. **TLS Setup (Client to APISIX)**
    - HTTPS between external clients and APISIX.
    - Configured for TLSv1.3 for enhanced security.

3. **mTLS Setup (APISIX to Sidecar)**
    - Mutual TLS between APISIX and the **NGINX sidecar**.
    - APISIX acts as an mTLS client using the certificate defined in `apisix.yaml`.
    - Rust REST API talks HTTP locally; sidecar handles TLS termination.

4. **Consumers and Basic Authentication**
    - Defines a `public` user with basic authentication.
    - Protects the `/keys` endpoint using the `basic-auth` and `consumer-restriction` plugins.

5. **Request ID Plugin**
    - Adds a unique `X-Request-ID` header to all requests.

6. **Routes and Services**
    - Maps paths to the upstream Rust API.
    - Includes routes for certificates, keys, health checks, and Swagger documentation.

7. **Hot Reloadable Plugins**
    - Plugins like `request-id`, `basic-auth`, `consumer-restriction`, `mocking`, `response-rewrite` reload automatically.

## Notes for Local Development

- **Configuration files**: Both `apisix.yaml` (standalone routes/consumers) and `config.yaml` (deployment role and provider) are required.
- **Static IPs** help troubleshooting.
- **Sidecar Flexibility**: Rust API can be exposed directly for debugging (port 9090).
- **Mocking**: Simulate responses without hitting real services.

## Quick Start (Local Development)

1. **Build and start services**:

```bash
docker-compose up --build
```

2. **Verify APISIX is running (Public Endpoint)**:

```bash
curl -k https://gateway.svc.docker.local:9443/health
```

3. **Verify Authentication (Protected Endpoint)**:

```bash
curl -k https://gateway.svc.docker.local:9443/keys --user public:public
```

4. **Access Rust API directly**:

```bash
# Bypassing the gateway and sidecar (Directly to Rust API)
curl http://127.0.0.1:9090/health
```

5. **Edit configuration**
    - Modify `apisix.yaml` or certificate files; reload happens automatically.

6. **Stop services**:

```bash
docker-compose down
```

---

This setup allows you to run APISIX, the NGINX sidecar, and the Rust API locally with mTLS, mocking, and basic authentication for fast iteration and troubleshooting.
