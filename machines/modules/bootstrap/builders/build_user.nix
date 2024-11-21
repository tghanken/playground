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
    openssh.authorizedKeys.keys = (import ../../../../secrets/secrets.nix).ssh_keys;
  };
  users.groups.nixbuilder = {};
  nix.settings.trusted-users = ["nixbuilder"];
}