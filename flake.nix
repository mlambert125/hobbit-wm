{
  description = "Burn Bar development environment flake";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    fenix,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
      rust = fenix.packages.${system}.complete.toolchain;
      rust-analyzer = fenix.packages.${system}.complete.rust-analyzer;
      clippy = fenix.packages.${system}.complete.clippy;
      rustfmt = fenix.packages.${system}.complete.rustfmt;
      libPackages = with pkgs; [
        udev
        seatd
        libxkbcommon
        wayland
        libGL
        libdisplay-info
        libinput
        pixman
        libgbm
        xwayland
      ];
    in {
      devShells.default = pkgs.mkShell {
        packages = with pkgs;
          [
            # languages / tooling
            rust
            rust-analyzer
            rustfmt
            clippy
            nixd
            alejandra
            pkg-config
          ]
          ++ libPackages;
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libPackages;

        env = {
          WINIT_UNIX_BACKEND = "wayland";
        };
      };
    });
}
