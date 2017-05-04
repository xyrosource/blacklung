with import ./rustOverlay.nix (import <nixpkgs>);
let
build-ci = stdenv.mkDerivation {
  name = "build-ci";
  src = ./.;
  buildInputs = [ rustChannels.stable.rust ];
  buildCommand = ''
    export CARGO_HOME=$PWD
    export CARGO_TARGET_DIR=$PWD
    unset SSL_CERT_FILE
    cd $src
    cargo build
    cargo test
    mkdir -p $out
'';
};

lint-ci = stdenv.mkDerivation {
  name = "lint-ci";
  src = ./.;
  buildInputs = [ rustChannels.nightly.rust ];
  buildCommand = ''
    export CARGO_HOME=$PWD
    unset SSL_CERT_FILE
    cargo install clippy
    export PATH=$CARGO_HOME/.cargo/bin:$PATH
    export CARGO_TARGET_DIR=$PWD
    cd $src
    cargo clippy -- -D warnings
    mkdir -p $out
'';
};
in
stdenv.mkDerivation {
  name = "blacklung-ci";
  src = ./.;
  buildInputs = [ build-ci lint-ci ];
  buildCommand = ''
    mkdir -p $out
'';
}
