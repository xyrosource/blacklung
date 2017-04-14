with import <nixpkgs> { };

let
  rust = (rustChannels.stable.rust.override { extensions = [ "rust-src" ]; });
in
stdenv.mkDerivation {
  name = "blacklung";
  buildInputs = [ rust rustracer rustfmt gcc ];

  RUST_SRC_PATH= "${rust}/lib/rustlib/src/rust/src";
}
