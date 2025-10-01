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
      rust-overlay,
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
          pkg-config
          libudev-zero
          alsa-lib
          vulkan-loader
          libudev-zero
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          libxkbcommon
          wayland
        ];

        nativeBuildInputs = with pkgs; [
          libxkbcommon

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

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
            vulkan-loader
            xorg.libX11
            xorg.libXi
            xorg.libXcursor
            libxkbcommon
          ]);

          nativeBuildInputs =
            with pkgs;
            [
              vulkan-tools
              nixfmt
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
