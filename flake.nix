{
  inputs = {
    nix.url = github:MODULUSREBUS/nix;
  };

  outputs = { self, nix }:
    with nix.lib;
    eachSystem [ system.x86_64-linux ] (system: let
      pkgs = nix.packages.${system};
      custom-rust = pkgs.rust-bin.stable.latest.default.override {
        extensions = [
          "rust-src"
        ];
        targets = [
          "x86_64-unknown-linux-gnu"
        ];
      };
    in {
      devShell = pkgs.devshell.mkShell {
        name = "index-access";
        packages = with pkgs; [
          git
          hub

          custom-rust
          rust-analyzer
          cargo-edit

          pkg-config
          gcc
          openssl.dev
        ];
        commands = [
          {
            name = "clippy";
            category = "rust";
            help = "rust linter";
            command = ''
              cargo clippy -- \
                -W clippy::pedantic \
                -A clippy::doc_markdown \
                -A clippy::missing_errors_doc
            '';
          }
        ];
        env = [
          {
            name = "PKG_CONFIG_PATH";
            prefix = "$DEVSHELL_DIR/lib/pkgconfig";
          }
        ];
      };
    }
  );
}
