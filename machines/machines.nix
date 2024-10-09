# The importApply argument. Use this to reference things defined locally,
# as opposed to the flake where this is imported.
localFlake:
# Regular module arguments; self, inputs, etc all reference the final user flake,
# where this module was imported.
{inputs, ...}:
with inputs; let
  home = [
    home-manager.nixosModules.home-manager
    {
      home-manager = {
        useGlobalPkgs = true;
        useUserPackages = true;
        users.tghanken = import ./home-manager/home.nix;
      };
    }
  ];
  builder = [nix-serve-ng.nixosModules.default];
  secrets = [agenix.nixosModules.default ../secrets/mod.nix];
  server_mods = [] ++ builder;
  desktop_mods = [] ++ server_mods;
  common_mods = [disko.nixosModules.disko ./common/common.nix] ++ home ++ secrets;
in {
  flake = {
    nixosConfigurations = {
      inwin-tower = inputs.nixpkgs.lib.nixosSystem {
        modules =
          [
            ./hosts/inwin-tower/configuration.nix
          ]
          ++ common_mods
          ++ desktop_mods;
      };
      nixos-thinkpad = inputs.nixpkgs.lib.nixosSystem {
        modules =
          [
            ./hosts/nixos-thinkpad/configuration.nix
          ]
          ++ common_mods
          ++ desktop_mods;
      };
      nixos-usb = inputs.nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        modules =
          [
            ./hosts/nixos-usb/configuration.nix
          ]
          ++ common_mods;
      };
    };
  };
}
