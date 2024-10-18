# Edit this configuration file to define what should be installed on
# your system.  Help is available in the configuration.nix(5) man page
# and in the NixOS manual (accessible by running ‘nixos-help’).
{
  config,
  pkgs,
  lib,
  modulesPath,
  ...
}:
with config; {
  networking.hostName = "nixos-bootstrap"; # Define your hostname.

  services.tailscale_user.auth_key = "tskey-auth-kiYBxaz5rN11CNTRL-PXqYkPTojtGP5iNEkR3DxGLviJYB9e7A6";

  networking.firewall = {
    # enable the firewall
    enable = true;

    # always allow traffic from your Tailscale network
    trustedInterfaces = ["tailscale0"];

    # allow the Tailscale UDP port through the firewall
    allowedUDPPorts = [config.services.tailscale.port];
  };
  services.openssh = {
    enable = true;
    openFirewall = false;
  };

  # Prebuild disko
  environment.systemPackages = with pkgs; [
    disko
    nh
    nix-output-monitor
  ];

  # This value determines the NixOS release from which the default
  # settings for stateful data, like file locations and database versions
  # on your system were taken. It‘s perfectly fine and recommended to leave
  # this value at the release version of the first install of this system.
  # Before changing this value read the documentation for this option
  # (e.g. man configuration.nix or on https://nixos.org/nixos/options.html).
  system.stateVersion = "24.05"; # Did you read the comment?
}
