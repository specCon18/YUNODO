{
  description = "SunServer";

  inputs={
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
  };
  outputs = { self, nixpkgs }@inputs:
    let
      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      #v I improved how you call in packages
      pkgs = forAllSystems (system:
        import nixpkgs {
          inherit system;
          #v this can be adjusted to read args passed without impure later
          config = { allowUnfreePredicate = pkg: builtins.elem (nixpkgs.lib.getName pkg) [
              "unrar"
            ];
          };
        }
      );
    in {
      packages = forAllSystems (system: {
        default = pkgs.${system}.callPackage ./nix/default.nix { };
      });
      devShells = forAllSystems (system: {
        default = pkgs.${system}.callPackage ./nix/devshell.nix { };
      });
      nixConfig = {
      };
    };
}
