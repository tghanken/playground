{
  lib,
  config,
  dream2nix,
  nixpkgs,
  ...
}: let
  fs = lib.fileset;
  sourceFiles = fs.unions [
    ../input.css
    ../package.json
    ../package-lock.json
    ../postcss.config.cjs
    ../tailwind.config.ts
    ../vite.config.ts
    (fs.fileFilter (file: file.hasExt "ts") ../src_ts)
    (fs.fileFilter (file: file.hasExt "html") ../templates)
  ];
in
  #  fs.trace sourceFiles
  {
    imports = [
      dream2nix.modules.dream2nix.nodejs-package-lock-v3
      dream2nix.modules.dream2nix.nodejs-granular-v3
    ];

    mkDerivation = {
      src = fs.toSource {
        root = ../.;
        fileset = sourceFiles;
      };
      installPhase = ''
        mkdir -p $manifest/manifest
        cp ./dist/.vite/manifest.json $manifest/manifest/manifest.json

        mkdir -p $assets/assets
        cp -r ./dist/assets $assets

        mkdir -p $sourcemaps/sourcemaps
        mv $assets/assets/*.js.map $sourcemaps/sourcemaps
      '';
      outputs = ["out" "manifest" "assets" "sourcemaps"];
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
