{
  description = "rust ARM M-0 development";
  inputs = {
    nixpkgs.url      = "github:patrickod/nixpkgs/personal";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.stable."latest".default.override {
          extensions = [
            "clippy-preview"
            "rust-src"
            "rustfmt-preview"
          ];
          targets = [
            "thumbv7em-none-eabihf"
          ];
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [
            rust
            pkgs.openssl
          ];
        };
      }
    );
}
