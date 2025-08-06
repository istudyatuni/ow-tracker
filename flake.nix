{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = {
    self,
    flake-utils,
    nixpkgs,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
        lib = pkgs.lib;
        libPath = with pkgs;
          lib.makeLibraryPath [
            libGL
            libxkbcommon
            wayland
            fontconfig
          ];
      in {
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            # rustc
            # cargo
            cargo-xwin
            yarn
            nodePackages.nodejs
          ];
          LD_LIBRARY_PATH = libPath;
        };
        formatter = with pkgs; writeShellScriptBin "alejandra" "exec ${lib.getExe alejandra} .";
      }
    );
}
