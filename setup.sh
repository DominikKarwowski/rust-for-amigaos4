#!/bin/sh
# setup.sh — One-time setup for Rust-for-AmigaOS4
#
# Installs Rust nightly, rust-src component, and pulls the Docker image.
# Run once before your first build.

set -e

echo "=== Rust for AmigaOS 4 — Setup ==="
echo

# 1. Check rustup
if ! command -v rustup >/dev/null 2>&1; then
    echo "ERROR: rustup not found. Install from https://rustup.rs/"
    exit 1
fi

# 2. Initialise submodules (clib4 source pinned in clib4-src/)
echo "[1/4] Initialising git submodules (clib4 source)..."
git submodule update --init --recursive
echo "  Done."

# 3. Install Rust toolchain (rust-toolchain.toml pins the exact nightly;
#    rustup will auto-install it on first cargo invocation, but we do it
#    up-front here so setup is an atomic one-shot).
echo "[2/4] Installing Rust toolchain pinned by rust-toolchain.toml..."
rustup show active-toolchain >/dev/null
echo "  Done."

# 4. Install rust-src (needed for build-std). rust-toolchain.toml lists it
#    as a component, so rustup installs it automatically, but we run this
#    defensively in case the user has an older rustup that ignores it.
echo "[3/4] Ensuring rust-src component is present..."
rustup component add rust-src
echo "  Done."

# 5. Pull Docker image
if ! command -v docker >/dev/null 2>&1; then
    echo "WARNING: Docker not found. Install Docker to link AmigaOS binaries."
    echo "  https://docs.docker.com/get-docker/"
else
    echo "[4/4] Pulling AmigaOS cross-compiler Docker image..."
    docker pull walkero/amigagccondocker:os4-gcc11
    echo "  Done."
fi

echo
echo "=== Setup complete ==="
echo
echo "Next steps:"
echo "  cd examples/hello"
echo "  ./build.sh        # or build.bat on Windows"
