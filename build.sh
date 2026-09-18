#!/usr/bin/env bash
set -euo pipefail
cargo build --release
echo 'Built target/release/bullet'
