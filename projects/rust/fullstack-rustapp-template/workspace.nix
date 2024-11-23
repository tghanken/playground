{inputs, ...}: {
  perSystem = {
    pkgs,
    lib,
    system,
    ...
  }:
    with inputs; let
      inherit pkgs lib;

      pkgs-unstable = import inputs.nixpkgs-unstable {
        inherit system;
      };

      pname = "fullstackRustappTemplate";

      rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
      craneLibNightly = (crane.mkLib pkgs).overrideToolchain (p: p.rust-bin.nightly.latest.minimal.override {});

      sqlFilter = path: _type: null != builtins.match ".*sql$" path;
      htmlFilter = path: _type: null != builtins.match ".*html$" path;
      jsonFilter = path: _type: null != builtins.match ".*json$" path;

      usedFileFilter = path: type:
        (sqlFilter path type)
        || (htmlFilter path type)
        || (jsonFilter path type)
        || (craneLib.filterCargoSources path type);

      src = lib.cleanSourceWith {
        src = ./.; # The original, unfiltered source
        filter = usedFileFilter;
        name = "source"; # Be reproducible, regardless of the directory name
      };

      baseArgs = {
        inherit src pname;
        strictDeps = true;

        # NB: we disable tests since we'll run them all via cargo-nextest
        doCheck = false;

        nativeBuildInputs = with pkgs; [clang mold];
        buildInputs =
          [
          ]
          ++ lib.optionals pkgs.stdenv.isDarwin [
            pkgs.libiconv
          ];
      };

      # Common arguments can be set here to avoid repeating them later
      commonArgs = baseArgs;

      workspaceHackSource = lib.fileset.toSource {
        root = ./.;
        fileset = lib.fileset.unions [
          ./.cargo
          ./Cargo.toml
          ./Cargo.lock
          ./packages/my-workspace-hack/.
        ];
      };

      # Build *just* the cargo dependencies (of the entire workspace),
      # so we can reuse all of that work (e.g. via cachix) when running in CI
      # It is *highly* recommended to use something like cargo-hakari to avoid
      # cache misses when building individual top-level-crates
      cargoArtifacts = craneLib.buildDepsOnly (baseArgs
        // {
          pname = "release";
          src = workspaceHackSource;
        });

      cargoDevArtifacts = craneLib.buildDepsOnly (baseArgs
        // {
          pname = "dev";
          src = workspaceHackSource;
          cargoBuildCommand = "cargo build";
        });

      individualCrateArgs =
        commonArgs
        // {
          inherit cargoArtifacts;
          inherit (craneLib.crateNameFromCargoToml {inherit src;}) version;
        };

      fileSetForCrate = crate:
        lib.fileset.toSource {
          root = ./.;
          fileset = lib.fileset.unions [
            ./.cargo
            ./Cargo.toml
            ./Cargo.lock
            ./packages
            crate
          ];
        };

      frontend-ui-vite-node-build = dream2nix.lib.evalModules {
        packageSets.nixpkgs = nixpkgs.legacyPackages.${system};
        modules = [
          # Import our actual package definiton as a dream2nix module from ./default.nix
          ./packages/libs/frontend-ui/vite.nix
          {
            # Aid dream2nix to find the project root. This setup should also works for mono
            # repos. If you only have a single project, the defaults should be good enough.
            paths.projectRoot = ./.;
            # can be changed to ".git" or "flake.nix" to get rid of .project-root
            paths.projectRootFile = "flake.nix";
            paths.package = ./.;
          }
        ];
      };

      vite-runtime-files = pkgs.stdenv.mkDerivation {
        name = "vite-runtime-files";
        src = frontend-ui-vite-node-build.assets;
        installPhase = ''
          mkdir -p $out/assets
          cp -r ./assets/* $out/assets
          ${pkgs.brotli}/bin/brotli -s $out/assets/*
        '';
      };
      vite-manifests = frontend-ui-vite-node-build.manifest;

      # Build the top-level crates of the workspace as individual derivations.
      # This allows consumers to only depend on (and build) only what they need.
      # Though it is possible to build the entire workspace as a single derivation,
      # so this is left up to you on how to organize things
      server = craneLib.buildPackage (individualCrateArgs
        // {
          pname = "fullstackRustappTemplate-server-prod";
          cargoExtraArgs = "-p server";
          src = fileSetForCrate ./packages/bins/server;
          meta.mainProgram = "server";
        });

      devServer = craneLib.buildPackage (individualCrateArgs
        // {
          pname = "fullstackRustappTemplate-server-dev";
          cargoBuildCommand = "cargo build";
          cargoArtifacts = cargoDevArtifacts;
          cargoExtraArgs = "-p server -F dev";
          src = fileSetForCrate ./packages/bins/server;
          meta.mainProgram = "server";
        });

      dockerImage = pkgs.dockerTools.streamLayeredImage {
        name = "fullstack-rustapp-server";
        tag = "latest";
        contents = [
          vite-runtime-files
        ];
        config = {
          Cmd = ["${server}/bin/server"];
          Env = ["VITE_MANIFEST_PATH=${vite-manifests}/manifest/manifest.json"];
        };
      };

      dockerDevImage = pkgs.dockerTools.streamLayeredImage {
        name = "fullstack-rustapp-server-dev";
        tag = "latest";
        contents = [
          vite-runtime-files
        ];
        config = {
          Cmd = ["${devServer}/bin/server"];
          Env = ["VITE_MANIFEST_PATH=${vite-manifests}/manifest/manifest.json"];
        };
      };
    in {
      _module.args = {
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
          ];
          config = {};
        };
      };

      checks = {
        # Run clippy (and deny all warnings) on the workspace source,
        # again, reusing the dependency artifacts from above.
        #
        # Note that this is done as a separate derivation so that
        # we can block the CI if there are issues here, but not
        # prevent downstream consumers from building our crate by itself.
        fullstackRustappTemplate-clippy = craneLib.cargoClippy (commonArgs
          // {
            cargoArtifacts = cargoDevArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

        fullstackRustappTemplate-doc = craneLib.cargoDoc (commonArgs
          // {
            cargoArtifacts = cargoDevArtifacts;
          });

        # Check formatting
        fullstackRustappTemplate-fmt = craneLib.cargoFmt {
          inherit src pname;
        };

        fullstackRustappTemplate-toml-fmt = craneLib.taploFmt {
          inherit pname;
          src = pkgs.lib.sources.sourceFilesBySuffices src [".toml"];
          # taplo arguments can be further customized below as needed
          taploExtraArgs = "--config ./.config/taplo.toml";
        };

        # Audit dependencies
        fullstackRustappTemplate-audit = craneLib.cargoAudit {
          inherit src pname advisory-db;
        };

        # Audit licenses
        fullstackRustappTemplate-deny = craneLib.cargoDeny {
          inherit src pname;
        };

        # Run tests with cargo-nextest
        # Consider setting `doCheck = false` on other crate derivations
        # if you do not want the tests to run twice
        fullstackRustappTemplate-nextest = craneLib.cargoNextest (commonArgs
          // {
            cargoArtifacts = cargoDevArtifacts;
            partitions = 1;
            partitionType = "count";
          });

        # Ensure that cargo-hakari is up to date
        fullstackRustappTemplate-hakari =
          baseArgs
          // craneLib.mkCargoDerivation {
            inherit src;
            pname = pname + "-hakari";
            cargoArtifacts = null;
            doInstallCargoArtifacts = false;

            buildPhaseCargoCommand = ''
              cargo hakari generate --diff  # workspace-hack Cargo.toml is up-to-date
              cargo hakari manage-deps --dry-run  # all workspace crates depend on workspace-hack
              cargo hakari verify
            '';

            nativeBuildInputs = [
              pkgs-unstable.cargo-hakari
            ];
          };

        # Check for unused dependencies with cargo-udeps
        fullstackRustappTemplate-udeps = craneLibNightly.mkCargoDerivation {
          inherit src;
          pname = pname + "-udeps";
          cargoArtifacts = null;

          buildPhaseCargoCommand = ''
            cargo udeps --workspace --exclude my-workspace-hack
          '';
          nativeBuildInputs = with pkgs; [
            cargo-udeps
            clang
            mold
          ];
        };
      };

      packages = {
        fullstackRustappTemplate-server-prod = server;
        fullstackRustappTemplate-server-prod-dockerImage = dockerImage;
        fullstackRustappTemplate-server-dev = devServer;
        fullstackRustappTemplate-server-dev-dockerImage = dockerDevImage;
      };

      apps = {
        fullstackRustappTemplate-server-prod.program = server;
        fullstackRustappTemplate-server-dev.program = devServer;
      };

      devShells.fullstackRustappTemplate-shell = craneLib.devShell {
        # Inherit inputs from checks.
        checks = self.checks.${system};

        # Additional dev-shell environment variables can be set directly
        # MY_CUSTOM_DEVELOPMENT_VAR = "something else";
        VITE_ASSETS_PATH = "${vite-runtime-files}/assets";
        VITE_MANIFEST_PATH = "${vite-manifests}/manifest/manifest.json";

        shellHooks = ''
          git lfs install
        '';

        # Extra inputs can be added here; cargo and rustc are provided by default.
        packages = with pkgs; [
          nix-output-monitor
          dive
          nodejs_22
          just
          flyctl
          watchexec
          bacon
          git-lfs
        ];
      };
    };
}
