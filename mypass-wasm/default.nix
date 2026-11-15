{
  rustPlatform,
  llvmPackages,
  wasm-pack,
  wasm-bindgen-cli_0_2_130,
  ...
}:
rustPlatform.buildRustPackage {
  pname = "mypass-wasm";
  version = "0.1.0";
  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;
  buildAndTestSubdir = "mypass-wasm";

  nativeBuildInputs = [
    wasm-pack
    wasm-bindgen-cli_0_2_130
    llvmPackages.bintools
  ];

  buildPhase = ''
    export HOME=$(mktemp -d)
    wasm-pack build -t web -m no-install --release mypass-wasm
  '';

  installPhase = ''
    mkdir -p $out
    cp -r mypass-wasm/pkg/* $out
  '';
}
