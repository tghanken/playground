{
  imports = [
    ./bootloader/bootloader.nix
    ./builders/build_server.nix
    ./desktop/desktop.nix
    ./locale/locale.nix
    ./networking/networking.nix
    ./sound/sound.nix
    ./users/users.nix
    ./utils/utils.nix
  ];
}
