{
  description = "Smithay Compositor Development Environment";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
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
