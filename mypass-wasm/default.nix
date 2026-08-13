{
  rustPlatform,
  fetchCrate,
  llvmPackages,
  wasm-pack,
  pkg-config,
  ...
}:
let
  wasm-bindgen-cli = rustPlatform.buildRustPackage rec {
    pname = "wasm-bindgen-cli";
    version = "0.2.127";
    src = fetchCrate {
      inherit pname version;
      hash = "sha256-di+qBAdd7pENLiIB9CoZoab+W5xeDoByMREcCGTSzWo=";
    };
    cargoHash = "sha256-FTv2GZIAQs0ePdIZXIXil7JbZ6kIT05VG6vqC1qNFxQ=";
    nativeBuildInputs = [ pkg-config ];
  };
in
rustPlatform.buildRustPackage {
  pname = "mypass-wasm";
  version = "0.1.0";
  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;
  buildAndTestSubdir = "mypass-wasm";

  nativeBuildInputs = [
    wasm-pack
    wasm-bindgen-cli
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
