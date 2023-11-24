{
  outputs = { self, nixpkgs }: 
  with nixpkgs; 
  {

    devShells.x86_64-linux.default = 
      let
        pkgs = nixpkgs.legacyPackages.x86_64-linux; 
      in pkgs.mkShell rec {
        buildInputs = with pkgs; [
          gcc
          cmake
          pkg-config
          fontconfig

          wayland
          libxkbcommon
        ];

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
      };
  };
}
