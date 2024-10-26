{pkgs, ...}: {
  hardware = {
    raspberry-pi."4".apply-overlays-dtmerge.enable = true;
    raspberry-pi."4".poe-plus-hat.enable = true;
    deviceTree = {
      enable = true;
      filter = "*rpi-4-*.dtb";
    };
  };

  environment.systemPackages = with pkgs; [
    libraspberrypi
    raspberrypi-eeprom
  ];
}
