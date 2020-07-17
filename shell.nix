let
  rust-version = "1.45.0";

  nixpkgs = fetchGit {
    url = "https://github.com/NixOS/nixpkgs.git";
    rev = "5272327b81ed355bbed5659b8d303cf2979b6953";
    ref = "release-20.03";
  };

  mozilla-overlay =
    import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);

  pkgs = import nixpkgs {
    overlays = [ mozilla-overlay ];
  };

  rust-channel = pkgs.rustChannelOf {
    channel = rust-version;
  };

  rust = rust-channel.rust.override {
    extensions = [ "rust-src" ];
  };

  cargo = rust-channel.cargo;
in
  pkgs.mkShell {
    name = "rust-dev";
    buildInputs = [ rust cargo pkgs.openssl pkgs.pkgconfig ];
  }
