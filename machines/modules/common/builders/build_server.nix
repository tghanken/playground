{
  config,
  lib,
  pkgs,
  ...
}: {
  users.users.nixbuilder = {
    isNormalUser = true;
    description = "nixbuilder";
    group = "nixbuilder";
  };
  users.groups.nixbuilder = {};
  nix.settings.trusted-users = ["nixbuilder"];

  services.nix-serve = {
    enable = true;
    secretKeyFile = config.age.secrets.nix_store_signing_key.path;
    port = 16893;
    openFirewall = true;
  };
}
