{inputs, ...}: {
  imports = [
    ./rust/fullstack-rustapp-template/workspace.nix
  ];

  perSystem = {
    pkgs,
    system,
    lib,
    ...
  }:
    with inputs; let
      inherit pkgs lib;
      craneLib = crane.mkLib pkgs;
    in {
      devShells.default = craneLib.devShell {
        # Inherit inputs from checks.
        checks = self.checks.${system};

        shellHooks = ''
          git lfs install
        '';

        packages = with pkgs; [
          # Core
          alejandra
          just
          dive
          nix-output-monitor
          bacon
          watchexec
          git-lfs

          # Build tools
          bazel_7
          bazel-buildtools

          # JS
          pnpm
          nodejs

          # Rust
          mold
          clang
          llvmPackages.bintools

          # CF Workers
          cargo-generate
          worker-build
          wasm-pack
          binaryen
        ];
      };
    };
}
