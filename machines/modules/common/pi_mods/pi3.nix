{
  lib,
  config,
  ...
}: let
  cfg = config.pi3Mods;
in {
  options = {
    pi3Mods.enable = lib.mkEnableOption "Enable Pi3 Mods";
  };

  config = lib.mkIf cfg.enable {
    hardware = {
      raspberry-pi."3".apply-overlays-dtmerge.enable = true;
      deviceTree = {
        enable = true;
        filter = "*rpi-3-*.dtb";
      };
    };

    environment.systemPackages = with pkgs; [
      libraspberrypi
      raspberrypi-eeprom
    ];
  };
}
