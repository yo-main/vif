with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "zeus-env";
  nativeBuildInputs = [ stdenv rust-analyzer hyperfine rustup ];
}
