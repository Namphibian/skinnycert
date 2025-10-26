#!/bin/bash
set -e

mkdir -p tls/ca tls/gateway

echo "📂 Creating directory structure..."
echo

echo "🛠️  Generating TLS CA..."
openssl genrsa -out tls/ca/tls_ca.key 2048
openssl req -new -sha256 -key tls/ca/tls_ca.key -out tls/ca/tls_ca.csr -subj "/CN=ROOTCA"
openssl x509 -req -days 36500 -sha256 -signkey tls/ca/tls_ca.key -in tls/ca/tls_ca.csr -out tls/ca/tls_ca.crt

echo "🔑 Generating Gateway certificate..."
openssl genrsa -out tls/gateway/gateway.key 2048
openssl req -new -sha256 -key tls/gateway/gateway.key -out tls/gateway/gateway.csr -subj "/CN=gateway.svc.docker.local"
openssl x509 -req -days 36500 -sha256 \
  -CA tls/ca/tls_ca.crt -CAkey tls/ca/tls_ca.key -CAserial tls/ca/tls_ca.srl -CAcreateserial \
  -in tls/gateway/gateway.csr -out tls/gateway/gateway.crt

echo
echo "✅ Certificates generated successfully!"
echo "📁 Output directories: tls/ca, tls/gateway"
