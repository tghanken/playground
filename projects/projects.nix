{inputs, ...}: {
  imports = [
    ./rust/fullstack-rustapp-template/workspace.nix
  ];

  perSystem = {
    pkgs,
    system,
    lib,
    ...
  }: {
    devShells.default = pkgs.mkShell {
      packages = with pkgs; [
        alejandra

        pnpm
        nodejs
        just
        dive
        cargo-generate
        cargo-leptos
        worker-build
        wasm-pack
        binaryen
      ];
    };
  };
}
