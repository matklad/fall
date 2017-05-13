with import <nixpkgs> {}; {
  cargoEnv = stdenv.mkDerivation {
    name = "cargo";
    buildInputs = [ pkgconfig openssl ];
  };
}
