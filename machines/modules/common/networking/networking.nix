{
  imports = [
    ./firewall.nix
  ];

  # Enable networking
  networking.networkmanager.enable = true;

  services.openssh = {
    enable = true;
  };
}
