{
  description = "Monorepo for everything homelab and/or experiment related that hasn't made it's way into a dedicated repo yet";

  inputs = {
    # Core Inputs
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    # Rust Inputs
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };

    # JS Inputs
    dream2nix = {
      url = "github:nix-community/dream2nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {
    # Core Inputs
    nixpkgs,
    flake-parts,
    # Rust Inputs
    rust-overlay,
    crane,
    advisory-db,
    # JS Inputs
    dream2nix,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} ({
      withSystem,
      flake-parts-lib,
      ...
    }: let
      inherit (flake-parts-lib) importApply;
      flakeModules.projects = import ./projects/projects.nix;
    in {
      imports = [
        flakeModules.projects
      ];

      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin"];

      perSystem = {
        pkgs,
        system,
        lib,
        ...
      }: {
        checks = let
          fs = lib.fileset;
          sourceFiles = fs.unions [
            (fs.fileFilter (file: file.hasExt "nix") ./.)
          ];
        in {
          nix-fmt-check = pkgs.stdenv.mkDerivation {
            name = "nix-fmt-check";
            src = fs.toSource {
              root = ./.;
              fileset = sourceFiles;
            };
            installPhase = ''
              ${pkgs.alejandra}/bin/alejandra -c . || exit 1
              touch $out
            '';
          };
        };
        formatter = pkgs.alejandra;
      };
    });
}
