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
    version = "0.2.128";
    src = fetchCrate {
      inherit pname version;
      hash = "sha256-a7lcXJnnZkYReja+iUO7NqqrWyv3toxnUgQb8s4IS5s=";
    };
    cargoHash = "sha256-R1Tas33Ursy8kqsxguAkG0ZhNed2n5uFTAhw1l2qlLY=";
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
