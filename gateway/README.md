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
    - Adds `X-Content-Type-Options: nosniff` for security.

2. **TLS Setup (Client to APISIX)**
    - HTTPS between external clients and APISIX.
    - Supports TLSv1.2 and TLSv1.3.

3. **mTLS Setup (Sidecar to APISIX)**
    - Mutual TLS between the **NGINX sidecar** and APISIX.
    - Rust REST API talks HTTP locally; sidecar enforces TLS.

4. **Consumers and Basic Authentication**
    - Defines `note` and `health` users with basic auth.
    - Protects endpoints like `/health` and `/note`.

5. **Request ID Plugin**
    - Adds a unique `X-Request-ID` header to all requests.

6. **Routes and Services**
    - Maps paths to upstream Rust API.
    - Mocking can simulate endpoints like `/posts`.

7. **Hot Reloadable Plugins**
    - Plugins like `request-id`, `basic-auth`, `consumer-restriction`, `mocking`, `response-rewrite` reload automatically.

## Notes for Local Development

- **Static IPs** help troubleshooting.
- **Sidecar Flexibility**: Rust API can be exposed directly for debugging.
- **Mocking**: Simulate responses without hitting real services.

## Quick Start (Local Development)

1. **Build and start services**:

```bash
docker-compose up --build
```

2. **Verify APISIX is running**:

```bash
curl -k https://gateway.svc.docker.local:9443/health --user health:health
```

3. **Access Rust API directly**:

```bash
curl http://127.0.0.1:8443/health
```

4. **Edit configuration**
    - Modify `apisix.yaml` or certificate files; reload happens automatically.

5. **Stop services**:

```bash
docker-compose down
```

---

This setup allows you to run APISIX, the NGINX sidecar, and the Rust API locally with mTLS, mocking, and basic authentication for fast iteration and troubleshooting.
