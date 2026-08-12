{ rustPlatform, ... }:
rustPlatform.buildRustPackage {
  pname = "mypass-cli";
  version = "0.1.0";
  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;
  buildAndTestSubdir = "mypass-cli";
}
