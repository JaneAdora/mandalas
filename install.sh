#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
cargo build --release
mkdir -p "$HOME/.cargo/bin" "$HOME/.local/bin"
install -m 0755 target/release/mandalas "$HOME/.cargo/bin/mandalas"
install -m 0755 target/release/mandalas "$HOME/.local/bin/mandalas"
echo "mandalas installed to ~/.cargo/bin and ~/.local/bin"
