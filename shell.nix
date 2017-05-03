let
  _pkgs = import <nixpkgs>;
  rustOverlay = (_pkgs { }).fetchFromGitHub {
    repo = "nixpkgs-mozilla";
    owner = "mozilla";
    rev = "918b538f52cd9fea4bb1cf56d7f5abba5a4af634";
    sha256 = "14g3g6g18qsncwpl940krywwsp8frbbmgy4pdmgxqffdg85nkx88";
  };
  pkgs = _pkgs { overlays = [ (import "${rustOverlay}/rust-overlay.nix") ]; };
  nightly = (pkgs.rustChannels.nightly.rust.override { extensions = [ "rls" "rust-analysis" ]; });
  rls = pkgs.stdenv.mkDerivation {
    name = "rls";
    buildInputs = [ nightly ];
    buildCommand = ''
      mkdir -p $out/bin
      cd $out/bin
      ln -s "${nightly}/bin/rls" rls
    '';
  };
  stable = (pkgs.rustChannels.stable.rust.override { extensions = [ "rust-src" ]; });
in
pkgs.stdenv.mkDerivation {
  name = "blacklung";
  buildInputs = with pkgs; [ rls stable rustfmt gcc ];

  RUST_SRC_PATH= "${stable}/lib/rustlib/src/rust/src";
}
