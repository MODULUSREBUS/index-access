{ pkgs, rust }:

with pkgs;

mkShell {
  buildInputs = [
    git
    hub
    gnumake
    pkg-config
    openssl

    rust
  ];
}
