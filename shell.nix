# Define nix function, use nixpkgs by default
{ pkgs ? import <nixpkgs> { } }:

# Avoid typing `pkgs.` before each package name
with pkgs;

# Define the shell
mkShell {
  nativeBuildInputs = [
    git
    rustup # Rust toolchain installation (mainly for Wasm target)
    wasm-pack # For compiling rust to Wasm
    nodePackages.npm # For iris-hub JS modules
    nixpkgs-fmt # Autoformatting for shell.nix
  ];
  buildInputs = [
    glibc # Required by some rust packages
  ];
}
