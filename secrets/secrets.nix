let
  # Add user keys from ~/.ssh for desktop machines
  inwin-tower-tghanken = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAICh921bOnrGEySjw/eRrUAj1UbV2sf1YIcm5X74r6gTh";
  nixos-thinkpad-tghanken = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOHrxGPx3dgap4sUwWyHbQsMJiv9tSNG05BEMNkNLDZF";
  tghanken = [inwin-tower-tghanken nixos-thinkpad-tghanken];

  users = tghanken;

  # Add machine keys from /etc/ssh
  inwin-tower = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAII/iE8w8saXDau1F/BQ5IktJPQO3MhRT1+1e5UsQt/n0";
  nixos-thinkpad = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIEiccufbIo8bYbn5n7PpR1IAFmup53P6nn8IyYfkJfd0";
  syno-vm = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOnjOGfqp5KqJ7CAIS8TQNtLkxPB5ObPbosSgXAyN8FS";

  machines = [inwin-tower nixos-thinkpad syno-vm];

  all = users ++ machines;
in {
  "keys/nix_store_signing_key.age".publicKeys = all;
  "keys/github_pat.age".publicKeys = all;
  "keys/tailscale_key.age".publicKeys = all;
}
