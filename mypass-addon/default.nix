{
  stdenvNoCC,
  callPackage,
  bun,
  zip,
  ...
}:
let
  mypass-wasm = callPackage ../mypass-wasm { };
  guid = "{0f7e3393-e120-4b1f-9d5c-46ffdfde4195}";
in
stdenvNoCC.mkDerivation {
  name = "mypass-addon";
  version = "3.0";
  src = ./.;

  nativeBuildInputs = [
    bun
    zip
  ];

  patchPhase = ''
    mkdir -p node_modules
    cp -r ${mypass-wasm} node_modules/mypass-wasm
  '';
  buildPhase = ''
    runHook preBuild
    bun run build
    runHook postBuild
  '';

  installPhase = ''
    dst="$out/share/mozilla/extensions/{ec8030f7-c20a-464f-9b0e-13a3a9e97384}"
    mkdir -p $dst
    ls -la
    cp mypass.xpi "$dst/${guid}.xpi"
  '';
}
