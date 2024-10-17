{
  description = "Monorepo for everything homelab and/or experiment related that hasn't made it's way into a dedicated repo yet";

  inputs = {
    # Core Inputs
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";
    nixpkgs-unstable.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.systems.follows = "nix-systems";
    };
    nix-systems = {
      url = "github:nix-systems/default";
    };

    # NixOs Inputs
    agenix = {
      url = "github:ryantm/agenix";
      # TODO: Enable once darwin is setup
      inputs.darwin.follows = "";
      inputs.home-manager.follows = "home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.systems.follows = "nix-systems";
    };
    disko = {
      url = "github:nix-community/disko";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    home-manager = {
      url = "github:nix-community/home-manager/release-24.05";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nix-serve-ng = {
      url = github:aristanetworks/nix-serve-ng;
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.utils.follows = "flake-utils";
      inputs.flake-compat.follows = "flake-compat";
    };
    flake-compat = {
      url = "github:edolstra/flake-compat";
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
      inputs.purescript-overlay.follows = "purescript-overlay";
    };
    purescript-overlay = {
      url = "github:thomashoneyman/purescript-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-compat.follows = "flake-compat";
      inputs.slimlock.follows = "slimlock";
    };
    slimlock = {
      url = "github:thomashoneyman/slimlock";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {
    # Core Inputs
    nixpkgs,
    nixpkgs-unstable,
    flake-parts,
    # NixOs Inputs
    agenix,
    disko,
    home-manager,
    nix-serve-ng,
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
      flakeModules.clusters = importApply ./clusters/clusters.nix {inherit withSystem;};
      flakeModules.machines = import ./machines/machines.nix;
      flakeModules.projects = import ./projects/projects.nix;
    in {
      imports = [
        flakeModules.clusters
        flakeModules.machines
        flakeModules.projects
      ];

      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin"];

      perSystem = {
        pkgs,
        system,
        ...
      }: {
        formatter = pkgs.alejandra;
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            inputs.agenix.packages.${system}.default
            nixos-generators
          ];
        };
      };

      flake = {
      };
    });
}
