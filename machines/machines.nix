# The importApply argument. Use this to reference things defined locally,
# as opposed to the flake where this is imported.
localFlake:
# Regular module arguments; self, inputs, etc all reference the final user flake,
# where this module was imported.
{inputs, ...}: {
  flake = {
    nixosConfigurations = {
      nixos-usb = inputs.nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        modules = [./hosts/nixos-usb/configuration.nix];
      };
    };
  };
}
