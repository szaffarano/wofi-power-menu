{
  description = "Highly configurable power menu using the wofi launcher ";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      with nixpkgs.legacyPackages.${system}; {
        packages = {
          wofi-power-menu = rustPlatform.buildRustPackage {
            name = "wofi-power-menu";
            version = "0.1.0";
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
            src = lib.cleanSource ./.;
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
            nixpkgs-fmt
          ];

          env = {
            RUST_BACKTRACE = "1";
          };
        };
      });
}
