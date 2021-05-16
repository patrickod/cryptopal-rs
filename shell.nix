let
  rust-version = "1.51.0";

  nixpkgs = fetchGit {
    url = "https://github.com/patrickod/nixpkgs.git";
    rev = "52e9607e1702ef469ec5cf6d11173c0ebaf1c8a7";
    ref = "personal";
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
