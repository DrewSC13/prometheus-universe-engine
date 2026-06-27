#!/usr/bin/env bash
set -euo pipefail

echo "[1/4] Formato Rust"
cargo fmt --all -- --check

echo "[2/4] Cargo check"
cargo check --workspace

echo "[3/4] Tests"
cargo test --workspace

echo "[4/4] Firma del último commit"
git log --show-signature -1

echo "Repositorio verificado localmente."
