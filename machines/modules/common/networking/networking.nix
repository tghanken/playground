{config, ...}:
with config; {
  # Enable networking
  networking.networkmanager.enable = true;
  services.tailscale_user.auth_key_path = age.secrets."tailscale_key".path;

  services.openssh = {
    enable = true;
    openFirewall = false;
  };
}
