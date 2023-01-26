{
  outputs = { self, nixpkgs }: 
  with nixpkgs; 
  {

    devShells.x86_64-linux.default = 
      let
        pkgs = nixpkgs.legacyPackages.x86_64-linux; 
      in pkgs.mkShell {
        buildInputs = [
          pkgs.gcc
          pkgs.cmake
          pkgs.pkg-config
          pkgs.fontconfig
        ];
      };
  };
}
