{
  config,
  lib,
  pkgs,
  ...
}:
with lib; let
  cfg = config.services.tailscale_user;
in {
  options.services.tailscale_user = {
    auth_key_path = mkOption {
      type = types.path;
      description = "Tailscale auth key file";
    };
    auth_key = mkOption {
      type = types.str;
      description = "Tailscale auth key";
    };
  };

  config = {
    # make the tailscale command usable to users
    environment.systemPackages = [pkgs.tailscale];

    # enable the tailscale service
    services.tailscale.enable = true;

    # create a oneshot job to authenticate to Tailscale
    systemd.services.tailscale-autoconnect = let
      # auth_key = if builtins.hasAttr "auth_key_path" cfg then "$(cat ${cfg.auth_key_path})" else cfg.auth_key;
    in {
      description = "Automatic connection to Tailscale";

      # make sure tailscale is running before trying to connect to tailscale
      after = ["network-pre.target" "tailscale.service"];
      wants = ["network-pre.target" "tailscale.service"];
      wantedBy = ["multi-user.target"];

      # set this service as a oneshot job
      serviceConfig.Type = "oneshot";

      # have the job run this shell script
      script = with pkgs; ''
        # wait for tailscaled to settle
        sleep 2

        # check if we are already authenticated to tailscale
        status="$(${tailscale}/bin/tailscale status -json | ${jq}/bin/jq -r .BackendState)"
        if [ $status = "Running" ]; then # if so, then do nothing
          exit 0
        fi

        # otherwise authenticate with tailscale
        ${tailscale}/bin/tailscale up -authkey $(cat ${cfg.auth_key_path}) --ssh
      '';
    };
  };
}
