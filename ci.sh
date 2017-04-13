#!/usr/bin/env bash
set -e

PATH="$HOME"/.cargo/bin:$PATH

git clone https://github.com/mozilla/nixpkgs-mozilla
pushd nixpkgs-mozilla
./rust-overlay-install.sh
popd

nix-shell -p rustChannels.nightly.rust --run 'cargo install --force clippy; cargo clean; cargo clippy'
