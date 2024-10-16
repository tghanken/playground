{config, ...}:
with config; {
  imports = [
    ./firewall.nix
  ];

  # Enable networking
  networking.networkmanager.enable = true;
  services.tailscale_user.auth_key_path = age.secrets."tailscale_key".path;

  services.openssh = {
    enable = true;
  };
}
