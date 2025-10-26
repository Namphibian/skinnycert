# NGINX Sidecar (development)

This folder contains everything required to run NGINX as an mTLS **sidecar** in front of the Skinnycert Rust REST API.

**Dev goals**
- Keep an unauthenticated HTTP port for quick connectivity checks/debugging.
- Provide an mTLS-protected HTTPS listener for integration tests and client cert checks.
- Practical and simple to run with `docker compose` locally.

---

## What this config does

- HTTP  : `80`   → plain HTTP, proxies to the API (bypass mTLS for quick checks).
- HTTPS : `8443` → TLS with mutual TLS (mTLS). NGINX requires a client cert signed by the local `ca.crt`.
- Both proxy to the REST API backend at `localhost:3000` (inside the sidecar container).
- Certificates are mounted into `/etc/nginx/conf.d/` (see `docker-compose` snippet).

---


