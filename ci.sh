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

if [[ $# -eq 1 ]] && [[ "$1" = "-n" ]]
then
  INSTALL=""
else
  INSTALL="cargo install --force clippy && "
fi

nix-shell --pure -p rustChannels.nightly.rust gcc --run "$INSTALL cargo clean && cargo clippy -- -D warnings"
