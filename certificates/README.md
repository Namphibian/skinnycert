# Certificate Generation

This folder contains scripts to generate the certificates required for running **TLS** and **mutual TLS (mTLS)** in your local development environment.

---

## Scripts

### 1. `gen_mtls_certs.sh`

- Generates certificates for **mutual TLS (mTLS)**.
- Used to secure communication between:
    - The **ApiSix gateway**.
    - The **NGINX sidecar** and the **Rust REST API**.
- Creates:
    - A **private CA**.
    - A **gateway certificate** signed by the CA.
    - A **client certificate** for the Rust API sidecar signed by the CA.
- Validity: 10 years by default (can be adjusted in the script).

**Purpose:** Ensures that both client and server authenticate each other, preventing unauthorized access.

---

### 2. `gen_tls_cert.sh`

- Generates certificates to secure traffic to the **ApiSix gateway** using standard TLS.
- Creates:
    - A **TLS CA**.
    - A **gateway certificate** signed by the CA.
- Validity: 100 years in the current configuration (adjustable).
- Optional: Add SANs for local hostnames or container hostnames to avoid browser/client warnings.

**Purpose:** Encrypts traffic between clients and the API gateway, providing confidentiality and integrity.

---

## Notes

- Two separate CAs are used in these scripts for demonstration purposes:
    - **mTLS CA** for mutual authentication between ApiSix and sidecar services.
    - **TLS CA** for securing external traffic to ApiSix.
- Using separate CAs is not strictly required; a single CA could be used for both purposes if desired.
- These scripts are intended for **local development** and testing only. For production deployments, consider using properly managed CA certificates.

---

## Recommended Workflow

1. Run `gen_mtls_certs.sh` to generate certificates for local services.
2. Run `gen_tls_cert.sh` to secure the gateway endpoint.
3. Mount the generated certificates in your Docker Compose or Kubernetes setup.

---

## Certificate Flow Diagram

```text
               +-------------------+
               |  External Client  |
               |    (HTTPS/TLS)   |
               +-------------------+
                        |
                        | TLS (secured by TLS CA)
                        v
               +-------------------+
               |   ApiSix Gateway  |
               |   (HTTPS/TLS)     |
               +-------------------+
                        |
                        | mTLS (mutual auth)
                        v
          +-------------+---------------+
          |                             |
+-------------------+          +-------------------+
| NGINX Sidecar     |          | Rust REST API     |
| (mTLS Client)     |          | (mTLS Server)     |
+-------------------+          +-------------------+
