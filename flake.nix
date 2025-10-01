{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      flake-utils,
      naersk,
      nixpkgs,
      rust-overlay
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [
            (import rust-overlay)
          ];
        };

        naersk' = pkgs.callPackage naersk { };

        buildInputs = with pkgs; [
          udev
          alsa-lib
          pkg-config
        ];

        nativeBuildInputs = with pkgs; [
          (pkgs.rust-bin.stable.latest.default.override {
            extensions = [
              "rust-src"
              "cargo"
              "rustc"
            ];
          })
        ];
      in
      rec {
        defaultPackage = packages.toybox-client;
        packages = {
          toybox-client = naersk'.buildPackage {
            src = ./.;
            nativeBuildInputs = nativeBuildInputs;
            buildInputs = buildInputs;
          };
          toybox-server = naersk'.buildPackage {
            src = ./.;
            nativeBuildInputs = nativeBuildInputs;
            buildInputs = buildInputs;
          };
          container = pkgs.dockerTools.buildImage {
            name = "toybox-client";
            config = {
              entrypoint = [ "${packages.toybox-client}/bin/toybox-client" ];
            };
          };
        };

        devShell = pkgs.mkShell {
          RUST_SRC_PATH = "${
            pkgs.rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
            }
          }/lib/rustlib/src/rust/library";

          nativeBuildInputs =
            with pkgs;
            [
              nixfmt
              cmake
              rustc
              rustfmt
              cargo
              clippy
              rust-analyzer
            ]
            ++ buildInputs
            ++ nativeBuildInputs;
        };
      }
    );
}