#!/usr/bin/env bash
set -e

PATH="$HOME"/.cargo/bin:$PATH

if ! [[ -f "$HOME/.config/nixpkgs/overlays/rust-overlay.nix" ]]
then
    git clone https://github.com/mozilla/nixpkgs-mozilla
    pushd nixpkgs-mozilla
    ./rust-overlay-install.sh
    popd
fi

nix-shell --pure -p rustChannels.nightly.rust gcc --run 'cargo install --force clippy; cargo clean; cargo clippy'
