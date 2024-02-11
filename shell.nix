with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "vif-env";
  nativeBuildInputs = [ stdenv rust-analyzer hyperfine mdbook rustup ];
}
