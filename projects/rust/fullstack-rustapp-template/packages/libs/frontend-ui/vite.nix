{
  lib,
  config,
  dream2nix,
  nixpkgs,
  ...
}: {
  imports = [
    dream2nix.modules.dream2nix.nodejs-package-lock-v3
    dream2nix.modules.dream2nix.nodejs-granular-v3
  ];

  mkDerivation = {
    src = ./.;
    installPhase = ''
      mkdir -p $manifest/manifest
      cp ./dist/.vite/manifest.json $manifest/manifest/manifest.json

      mkdir -p $assets/assets
      cp -r ./dist/assets $assets

      mkdir -p $sourcemaps/sourcemaps
      mv $assets/assets/*.js.map $sourcemaps/sourcemaps

      mkdir -p $node_modules/node_modules
      cp -r ./node_modules $node_modules
    '';
    outputs = ["out" "manifest" "assets" "sourcemaps" "node_modules"];
  };

  deps = {nixpkgs, ...}: {
    inherit
      (nixpkgs)
      fetchFromGitHub
      stdenv
      ;
  };

  nodejs-package-lock-v3 = {
    packageLockFile = "${config.mkDerivation.src}/package-lock.json";
  };

  name = "frontend-ui-vite-manifests";
  version = "0.1.0";
}
