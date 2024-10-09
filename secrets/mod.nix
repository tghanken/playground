{
  # Secrets
  age.secrets = {
    github_pat.file = ./keys/github_pat.age;
    nix_store_signing_key.file = ./keys/nix_store_signing_key.age;
    tailscale_key.file = ./keys/tailscale_key.age;
  };
}
