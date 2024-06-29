{ pkgs ? import <nixpkgs> { }, lib }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "yunodo";
  version = "0.7.0";
  cargoLock.lockFile = ../Cargo.lock;
  src = pkgs.lib.cleanSource ../.;
  buildInputs = [ ];
  nativeBuildInputs = [ pkgs.pkg-config ];
  doCheck = false;
}
