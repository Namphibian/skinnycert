#!/bin/bash
set -e

# Create directories
mkdir -p mtls/ca mtls/gateway mtls/sidecar

echo "📂 Creating directory structure..."
echo

echo "🛠️  Generating CA..."
openssl req -x509 -nodes -new -keyout mtls/ca/ca.key \
  -out mtls/ca/ca.crt -days 3650 \
  -subj "/C=/ST=/L=/O=/OU=web/CN=private_ca"

echo "🔑 Generating Gateway key and CSR..."
openssl req -newkey rsa:2048 -nodes -keyout mtls/gateway/gateway.key \
  -out mtls/gateway/gateway.req -subj "/C=/ST=/L=/O=/OU=security/CN=gateway"

echo "🧾 Signing Gateway certificate..."
openssl x509 -req -days 3650 -set_serial 01 \
  -in mtls/gateway/gateway.req -out mtls/gateway/gateway.crt \
  -CA mtls/ca/ca.crt -CAkey mtls/ca/ca.key

echo "🔐 Generating Sidecar key and CSR..."
openssl req -newkey rsa:2048 -nodes -keyout mtls/sidecar/sidecar.key \
  -out mtls/sidecar/sidecar.req -subj "/C=/ST=/L=/O=/OU=security/CN=api"

echo "🧾 Signing Sidecar certificate..."
openssl x509 -req -days 3650 -set_serial 02 \
  -in mtls/sidecar/sidecar.req -out mtls/sidecar/sidecar.crt \
  -CA mtls/ca/ca.crt -CAkey mtls/ca/ca.key

echo
echo "✅ Certificates generated successfully!"
echo "📁 Output directories: mtls/ca, mtls/gateway, mtls/sidecar"
