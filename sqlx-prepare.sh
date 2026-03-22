#!/bin/bash
# Script to run sqlx prepare correctly via cargo.
# This generates the .sqlx directory needed for offline compilation.
echo "Running: cargo sqlx prepare"
cargo sqlx prepare
