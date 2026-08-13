{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };
  outputs =
    { nixpkgs, ... }:
    let
      lib = nixpkgs.lib;
      eachSystem = lib.genAttrs [
        "aarch64-darwin"
        "aarch64-linux"
        "i686-linux"
        "x86_64-linux"
      ];
    in
    {
      packages = eachSystem (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        {
          mypass-cli = pkgs.callPackage ./mypass-cli { };
          mypass-wasm = pkgs.callPackage ./mypass-wasm { };
          mypass-addon = pkgs.callPackage ./mypass-addon { };
        }
      );
      devShells = eachSystem (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        {
          default = pkgs.mkShell {
            name = "rust environment";
            packages = with pkgs; [
              cargo
              rustc
              rust-analyzer
              rustfmt

              wasm-pack
              wasm-bindgen-cli
              llvmPackages.bintools

              bun
              typescript-language-server
              zip

              nixd
              nixfmt
            ];
          };
        }
      );
    };
}
