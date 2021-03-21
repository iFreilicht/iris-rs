# Define nix function, use nixpkgs with oxalica rust-bin overlay
let
  rust_overlay = import (builtins.fetchTarball
    "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  nixpkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  rust_channel = nixpkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain;
in
# Avoid typing `nixpkgs.` before each package name
with nixpkgs;

# Define the shell
pkgs.mkShell {
  nativeBuildInputs = [
    rust_channel # Full rust from overlay
    git
    nodePackages.npm # For iris-hub JS modules
    nixpkgs-fmt # Autoformatting for shell.nix
    wasm-pack # Compiling to WASM and packing with web-stuff
  ];
}
