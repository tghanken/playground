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
      syno-vm = inputs.nixpkgs.lib.nixosSystem {
        modules =
          [
            ./hosts/servers/syno-vm/configuration.nix
          ]
          ++ common_mods
          ++ server_mods;
      };

    };
  };
  perSystem = {
    packages = let
      bootstrap_modules = [./hosts/utils/nixos-bootstrap/configuration.nix] ++ core_mods;
    in {
      nixos-vm-bootstrap-image = inputs.nixos-generators.nixosGenerate {
        modules = bootstrap_modules;
        format = "iso";
      };
      nixos-rpi-bootstrap-image = inputs.nixos-generators.nixosGenerate {
        system = "aarch64-linux";
        modules = bootstrap_modules;
        format = "sd-aarch64";
      };
    };
  };
}
