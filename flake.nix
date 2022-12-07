{
  inputs = {
    nix.url = github:MODULUSREBUS/nix;
  };

  outputs = { self, nix }:
    with nix.lib;
    eachSystem [ system.x86_64-linux ] (system: {
      devShell = let
        pkgs = nix.packages.${system};
        custom-rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
          ];
          targets = [
            "x86_64-unknown-linux-gnu"
          ];
        };
      in pkgs.mkShell {
        buildInputs = with pkgs; [
          git
          hub

          custom-rust
          cargo-edit
        ];
      };
  });
}
