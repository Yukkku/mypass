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
    version = "0.2.129";
    src = fetchCrate {
      inherit pname version;
      hash = "sha256-pcecKQd7E8Opw6bkFoE569epUi7gh5qpQF1e5PJY6V8=";
    };
    cargoHash = "sha256-vmUrWVU7kPJJxO5qIVeAkwQyWDELO1Z4Z5gitz2kco8=";
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
