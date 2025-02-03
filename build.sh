#!/bin/bash
apt-get update
apt-get install -y pkg-config libssl-dev
cargo build --release