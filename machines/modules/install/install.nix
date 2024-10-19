{
  imports = [
    ./bootloader/bootloader.nix
    ./locale/locale.nix
    ./networking/networking.nix
  ];

  disko.extraRootModules = ["zfs"];
}
