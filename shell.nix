{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShellNoCC {

  packages = with pkgs; [ 
    gcc
    rust-analyzer
    rustup
    hyperfine
    mdbook 
  ];

}
