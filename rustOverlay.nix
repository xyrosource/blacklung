pkgs:
let
  rustOverlay = (pkgs { }).fetchFromGitHub {
    repo = "nixpkgs-mozilla";
    owner = "mozilla";
    rev = "918b538f52cd9fea4bb1cf56d7f5abba5a4af634";
    sha256 = "14g3g6g18qsncwpl940krywwsp8frbbmgy4pdmgxqffdg85nkx88";
  };
in
pkgs { overlays = [ (import "${rustOverlay}/rust-overlay.nix") ]; }
