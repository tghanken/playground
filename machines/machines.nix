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
  secrets = [agenix.nixosModules.default ../secrets/mod.nix];

  # Apply to all hosts, including bootstrap images
  core_mods = [disko.nixosModules.disko ./modules/core/core.nix];

  # Apply to all hosts, except bootstrap images
  common_mods = [nix-serve-ng.nixosModules.default ./modules/common/common.nix] ++ core_mods ++ home ++ secrets;

  # Apply to only servers
  server_mods = [] ++ common_mods;

  # Apply to only desktops
  desktop_mods = [] ++ common_mods;
in {
  flake = {
    nixosConfigurations = {
      # Desktops
      inwin-tower = inputs.nixpkgs.lib.nixosSystem {
        modules =
          [
            ./hosts/desktops/inwin-tower/configuration.nix
          ]
          ++ desktop_mods;
      };
      nixos-thinkpad = inputs.nixpkgs.lib.nixosSystem {
        modules =
          [
            ./hosts/desktops/nixos-thinkpad/configuration.nix
          ]
          ++ desktop_mods;
      };

      # Servers
      syno-vm = inputs.nixpkgs.lib.nixosSystem {
        modules =
          [
            ./hosts/servers/syno-vm/configuration.nix
          ]
          ++ core_mods;
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
