{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

  outputs =
    { self, ... }@inputs:
    let
      systems = [ "x86_64-linux" ];
      eachSystem = inputs.nixpkgs.lib.genAttrs systems;
    in
    {
      packages = eachSystem (system: {
        default = inputs.nixpkgs.legacyPackages.${system}.callPackage ./package.nix { };
      });

      devShells = eachSystem (
        system:
        let
          overlays = [ (import inputs.rust-overlay) ];
          pkgs = import inputs.nixpkgs {
            inherit system overlays;
          };
        in
        {
          default = pkgs.mkShellNoCC {
            packages = with pkgs; [ rust-bin.stable.latest.default ];
          };
        }
      );
    };
}
