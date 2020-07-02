# let
#   pkgs = import <nixpkgs> {};
# in
# pkgs.mkShell {
#   buildInputs = [
#     pkgs.hello
#   ];
# }


let
  # pkgs = import <nixpkgs> {};
  rust-version = "1.40.0";

  nixpkgs = fetchGit {
    url = "https://github.com/NixOS/nixpkgs.git";
    rev = "a3070689aef665ba1f5cc7903a205d3eff082ce9";
    ref = "release-19.09";
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
    buildInputs = [ rust cargo pkgs.rls ];
  }
