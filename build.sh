#!/bin/bash
set -e  # Exit on error

echo "Updating package lists..."
apt-get update

echo "Installing dependencies..."
apt-get install -y pkg-config libssl-dev

echo "Setting OpenSSL environment variables..."
export OPENSSL_DIR=/usr
export OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu
export OPENSSL_INCLUDE_DIR=/usr/include/openssl

echo "Verifying OpenSSL installation..."
ls -l $OPENSSL_DIR
ls -l $OPENSSL_LIB_DIR
ls -l $OPENSSL_INCLUDE_DIR

echo "Building project..."
cargo build --release

echo "Build completed!"