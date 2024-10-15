{
  config,
  lib,
  pkgs,
  ...
}: {
  nix.buildMachines = [
    {
      hostName = "nixos-thinkpad";
      sshUser = "nixbuilder";
      systems = ["x86_64-linux" "aarch64-linux"];
      protocol = "ssh";
      maxJobs = 8;
      speedFactor = 0;
      supportedFeatures = ["nixos-test" "benchmark" "big-parallel" "kvm"];
      mandatoryFeatures = [];
    }
    {
      hostName = "inwin-tower";
      sshUser = "nixbuilder";
      systems = ["x86_64-linux" "aarch64-linux"];
      protocol = "ssh";
      maxJobs = 8;
      speedFactor = 0;
      supportedFeatures = ["nixos-test" "benchmark" "big-parallel" "kvm"];
      mandatoryFeatures = [];
    }
  ];
  nix.settings.substituters = [
    (
      if (config.networking.hostName != "nixos-thinkpad")
      then "http://nixos-thinkpad.myth-chameleon.ts.net:16893"
      else ""
    )
    (
      if (config.networking.hostName != "inwin-tower")
      then "http://inwin-tower.myth-chameleon.ts.net:16893"
      else ""
    )
  ];
  programs.ssh.extraConfig = ''
    Host nixos-thinkpad
      StrictHostKeyChecking no
      UpdateHostkeys yes
      ConnectTimeout=1
      ConnectionAttempts=1

    Host inwin-tower
        StrictHostKeyChecking no
        UpdateHostkeys yes
        ConnectTimeout=1
        ConnectionAttempts=1
  '';

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

  nix.distributedBuilds = true;
  nix.settings = {
    builders-use-substitutes = true;
    trusted-public-keys = [home-builders:U3PWObVWROdM+8rlJqk70qE9aeffp9RsKmoCojx7XQ0=];
  };
}
