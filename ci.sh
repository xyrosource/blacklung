#!/usr/bin/env bash
set -e

# Install overlay
if ! [[ -f "$HOME/.config/nixpkgs/overlays/rust-overlay.nix" ]]
then
    git clone https://github.com/mozilla/nixpkgs-mozilla
    pushd nixpkgs-mozilla
    ./rust-overlay-install.sh
    popd
fi

# Build and test on stable
nix-shell --pure -p rustChannels.stable.rust gcc --run "cargo clean && cargo build && cargo test"

# Lint with clippy
if [[ $# -eq 1 ]] && [[ "$1" = "-n" ]]
then
  INSTALL=""
else
  INSTALL="cargo install --force clippy && "
fi

PATH="$HOME"/.cargo/bin:$PATH
nix-shell --pure -p rustChannels.nightly.rust gcc --run "$INSTALL cargo clean && cargo clippy -- -D warnings"
