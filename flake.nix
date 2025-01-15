{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system:
      with nixpkgs.legacyPackages.${system}; {
        packages = {
          wofi-power-menu = rustPlatform.buildRustPackage {
            pname = "wofi-power-menu";
            version = "0.2.6";

            src = lib.cleanSource ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            meta = with lib; {
              description = "Highly configurable power menu using the wofi launcher ";
              homepage = "https://github.com/szaffarano/wofi-power-menu";
              license = licenses.mit;
              maintainers = [];
            };
          };
        };

        formatter = nixpkgs-fmt;

        defaultPackage = self.packages.${system}.wofi-power-menu;

        devShell = mkShell {
          inputsFrom = builtins.attrValues self.packages.${system};
          packages = [
            cargo-bloat
            cargo-edit
            cargo-outdated
            cargo-udeps
            cargo-watch
            clippy
            rust-analyzer
            curl
            git
            jq
          ];

          env = {
            RUST_BACKTRACE = "1";
          };
        };
      });
}
