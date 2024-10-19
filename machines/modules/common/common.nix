{
  imports = [
    ./bootloader/bootloader.nix
    ./builders/build_server.nix
    ./locale/locale.nix
    ./networking/networking.nix
    ./sound/sound.nix
    ./users/users.nix
    ./utils/utils.nix
  ];
}
