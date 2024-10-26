{pkgs, ...}: {
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
}
