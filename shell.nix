with import <nixpkgs> { };

let
  nightly = (rustChannels.nightly.rust.override { extensions = [ "rls" "rust-analysis" ]; });
  rls = stdenv.mkDerivation {
    name = "rls";
    buildInputs = [ nightly ];
    buildCommand = ''
      mkdir -p $out/bin
      cd $out/bin
      ln -s "${nightly}/bin/rls" rls
    '';
  };
  stable = (rustChannels.stable.rust.override { extensions = [ "rust-src" ]; });
in
stdenv.mkDerivation {
  name = "blacklung";
  buildInputs = [ rls stable rustfmt gcc ];

  RUST_SRC_PATH= "${stable}/lib/rustlib/src/rust/src";
}
