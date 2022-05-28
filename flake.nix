{
  description = "rust cryptopals development";
  inputs = {
    nixpkgs.url = "github:patrickod/nixpkgs/personal";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      with nixpkgs;
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions =
            [ "clippy-preview" "rust-src" "rustfmt-preview" "rust-analysis" ];
        };
      in {
        devShell = pkgs.mkShell rec {
          buildInputs =
            [ rust pkgs.rust-analyzer pkgs.gcc pkgs.openssl pkgs.pkg-config ];

          # important environment variables
          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
          # OPENSSL_DIR = "${lib.getDev pkgs.openssl}";
        };
      });
}
