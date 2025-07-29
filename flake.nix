{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem(
    system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
      };

      inherit (pkgs) lib;

      craneLib = (crane.mkLib pkgs).overrideToolchain (p: p.rust-bin.stable.latest.default);
      
      unfilteredRoot = ./.;
      src = lib.fileset.toSource {
        root = unfilteredRoot;
        fileset = lib.fileset.unions [
          (craneLib.fileset.commonCargoSources unfilteredRoot)
          (lib.fileset.maybeMissing ./templates)
        ];
      };

      commonArgs = {
        inherit src;
        strictDeps = true;
      };
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      fixit-crate = craneLib.buildPackage(
        commonArgs
        // {
          inherit cargoArtifacts;
          nativeCheckInputs = with pkgs; [ bash fish zsh tmux ];
        }
      );
    in
    {
      checks = {
        cargo-nextest = craneLib.cargoNextest(
          commonArgs
          // {
            inherit cargoArtifacts;
            nativeBuildInputs = with pkgs; [ bash fish zsh tmux ];
            CARGO_PROFILE = "dev";
          }
        );
      };
    }
    );
}
