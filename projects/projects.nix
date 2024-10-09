{inputs, ...}: {
  imports = [
    ./rust/fullstack-rustapp-template/workspace.nix
  ];
  perSystem = {
    pkgs,
    system,
    ...
  }: {
    devShells.default = pkgs.mkShell {
      packages = with pkgs; [inputs.agenix.packages.${system}.default];
    };
  };
}
