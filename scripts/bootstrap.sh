#!/usr/bin/env bash
set -euo pipefail

cargo fmt
cargo check --workspace
cargo test --workspace
cargo run -p forgegate-cli -- init
cargo run -p forgegate-cli -- run "Fix the failing parser tests"
cargo run -p forgegate-cli -- inspect last
