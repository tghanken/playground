{
  description = "Monorepo for everything homelab and/or experiment related that hasn't made it's way into a dedicated repo yet";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";
    home-manager = {
      url = "github:nix-community/home-manager/release-24.05";
      inputs.nixpkgs.follows = "nixpkgs";
    };
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
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    nix-serve-ng = {
      url = github:aristanetworks/nix-serve-ng;
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.utils.follows = "flake-utils";
      inputs.flake-compat.follows = "flake-compat";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.systems.follows = "nix-systems";
    };
    flake-compat = {
      url = "github:edolstra/flake-compat";
    };
    nix-systems = {
      url = "github:nix-systems/default";
    };
  };

  outputs = inputs @ {
    nixpkgs,
    home-manager,
    agenix,
    disko,
    flake-parts,
    nix-serve-ng,
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
      flakeModules.projects = importApply ./projects/projects.nix {inherit withSystem;};
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

        packages.default = pkgs.hello;

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [agenix.packages.${system}.default];
        };
      };

      flake = {
      };
    });
}
