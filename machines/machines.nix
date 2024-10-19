{inputs, ...}:
with inputs; let
  home = [
    home-manager.nixosModules.home-manager
    {
      home-manager = {
        useGlobalPkgs = true;
        useUserPackages = true;
        users.tghanken = import ./modules/home-manager/home.nix;
      };
    }
  ];
  builder = [nix-serve-ng.nixosModules.default];
  secrets = [agenix.nixosModules.default ../secrets/mod.nix];
  core_mods = [./modules/core/core.nix] ++ builder;
  server_mods = [] ++ core_mods;
  desktop_mods = [] ++ server_mods;
  common_mods = [disko.nixosModules.disko ./modules/common/common.nix] ++ home ++ secrets;
in {
  flake = {
    nixosConfigurations = {
      inwin-tower = inputs.nixpkgs.lib.nixosSystem {
        modules =
          [
            ./hosts/desktops/inwin-tower/configuration.nix
          ]
          ++ common_mods
          ++ desktop_mods;
      };
      nixos-thinkpad = inputs.nixpkgs.lib.nixosSystem {
        modules =
          [
            ./hosts/desktops/nixos-thinkpad/configuration.nix
          ]
          ++ common_mods
          ++ desktop_mods;
      };
      nixos-bootstrap = inputs.nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        modules =
          [
            ./hosts/utils/nixos-bootstrap/configuration.nix
          ]
          ++ core_mods;
      };
    };
  };
}
