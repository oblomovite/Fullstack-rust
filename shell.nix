# See this link for nightly builds of rust
# https://github.com/NixOS/nixpkgs/blob/0109d6587a587170469cb11afc18a3afe71859a3/doc/languages-frameworks/rust.section.md#using-the-rust-nightlies-overlay

# Run this with

# $ nix-shell --pure
# $ cargo build
# $ cargo test

# with import <nixpkgs> {};
# 
# stdenv.mkDerivation {
#   name = "rust-env";
#   nativeBuildInputs = [
#     rustc cargo
# 
#     # Example Build-time Additional Dependencies
#     pkgconfig
#   ];
#   buildInputs = [
#     # Example Run-time Additional Dependencies
#     openssl
#   ];
# 
#   # Set Environment Variables
#   RUST_BACKTRACE = 1;
# }

let
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
    buildInputs = [ rust cargo ];
  }
