{
  description = "Kaji - A logical puzzle solver";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        kaji = pkgs.rustPlatform.buildRustPackage {
          pname = "kaji";
          version = "git";
          src = ./.;
          cargoLock = { lockFile = ./Cargo.lock; };
        };
      in with pkgs; {
        packages = {
          inherit kaji;
          default = kaji;
        };
        devShells.default = mkShell { buildInputs = [ gnumake ]; };
      });
}
