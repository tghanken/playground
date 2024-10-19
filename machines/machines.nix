{inputs, ...}:
with inputs; let
  home = [
    home-manager.nixosModules.home-manager
    {
      home-manager = {
        useGlobalPkgs = true;
        useUserPackages = true;
        users.tghanken = import ./modules/users/tghanken/home-manager.nix;
      };
    }
  ];
  users = [./modules/users/users.nix];

  secrets = [agenix.nixosModules.default ../secrets/mod.nix];

  # Apply to all hosts, including bootstrap images
  bootstrap_mods = [./modules/bootstrap/bootstrap.nix];

  # Apply to all hosts, including hosts being adopted
  install_mods = [disko.nixosModules.disko ./modules/install/install.nix] ++ bootstrap_mods ++ secrets ++ users;

  # Apply to all activated hosts
  common_mods = [nix-serve-ng.nixosModules.default ./modules/common/common.nix] ++ install_mods ++ home;

  # Apply to only servers
  server_mods = [./modules/server/server.nix] ++ common_mods;

  # Apply to only desktops
  desktop_mods = [./modules/desktop/desktop.nix] ++ common_mods;
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
          ++ install_mods;
      };
    };
  };
  perSystem = {
    packages = let
      bootstrap_modules = [./hosts/utils/nixos-bootstrap/configuration.nix] ++ bootstrap_mods;
    in {
      nixos-vm-bootstrap-image = inputs.nixos-generators.nixosGenerate {
        system = "x86_64-linux";
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
