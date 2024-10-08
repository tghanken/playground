# The importApply argument. Use this to reference things defined locally,
# as opposed to the flake where this is imported.
localFlake:
# Regular module arguments; self, inputs, etc all reference the final user flake,
# where this module was imported.
{inputs, ...}: {
  flake = {
    nixosConfigurations = {
      inwin-tower = inputs.nixpkgs.lib.nixosSystem {
        modules = with inputs; [
          ./hosts/inwin-tower/configuration.nix
          ./common/common.nix
          ../secrets/mod.nix

          nix-serve-ng.nixosModules.default
          agenix.nixosModules.default
          {
            # TODO: Split this into a flake-part module
            environment.systemPackages = [agenix.packages."x86_64-linux".default];
          }
          home-manager.nixosModules.home-manager
          {
            home-manager.useGlobalPkgs = true;
            home-manager.useUserPackages = true;

            home-manager.users.tghanken = import ./home-manager/home.nix;
          }
          disko.nixosModules.disko
          {
            disko.devices = import ./hosts/inwin-tower/devices.nix;
          }
        ];
      };
      nixos-thinkpad = inputs.nixpkgs.lib.nixosSystem {
        modules = with inputs; [
          ./hosts/nixos-thinkpad/configuration.nix
          ./common/common.nix
          ../secrets/mod.nix

          nix-serve-ng.nixosModules.default
          agenix.nixosModules.default
          {
            # TODO: Split this into a flake-part module
            environment.systemPackages = [agenix.packages."x86_64-linux".default];
          }
          home-manager.nixosModules.home-manager
          {
            home-manager.useGlobalPkgs = true;
            home-manager.useUserPackages = true;

            home-manager.users.tghanken = import ./home-manager/home.nix;
          }
          disko.nixosModules.disko
          {
            disko.devices = import ./hosts/nixos-thinkpad/devices.nix;
          }
        ];
      };
      nixos-usb = inputs.nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        modules = [
          ./hosts/nixos-usb/configuration.nix
        ];
      };
    };
  };
}
