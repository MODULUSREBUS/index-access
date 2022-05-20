{ pkgs, rust }:

with pkgs;

mkShell {
  buildInputs = [
    git
    hub
    gnumake

    rust
  ];
}
