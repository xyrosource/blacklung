with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "blacklung";

  buildInputs = [ rustc cargo ];
}
