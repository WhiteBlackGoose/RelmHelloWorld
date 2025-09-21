{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  outputs = { nixpkgs, ... }:
    let
      systems = [ "aarch64-darwin" "x86_64-darwin" "aarch64-linux" "x86_64-linux" ];
    in
    {
      formatter.x86_64-linux = nixpkgs.legacyPackages.x86_64-linux.nixpkgs-fmt;
      devShells = nixpkgs.lib.genAttrs systems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default =
            pkgs.mkShell rec {
              nativeBuildInputs = with pkgs; [
                pkgs.rustup
                pkgs.vscode-extensions.vadimcn.vscode-lldb
                pkgs.cmake
                pkgs.pkg-config
                pkgs.fontconfig
                pkgs.ninja
                pkgs.gcc
                pkgs.freetype
                pkgs.graphene
                pkgs.glib
                pkgs.openssl
                bzip2
                alsa-lib

                pkgs.pango
                pkgs.cairo
                pkgs.doxygen
                pkgs.heaptrack

                pkgs.redis
                pkgs.gtk4
                pkgs.gdk-pixbuf
                pkgs.librsvg
                pkgs.powershell
                pkgs.azure-cli
                pkgs.alsa-lib.dev

                rustPlatform.bindgenHook
                libGL
                xorg.libX11
                xorg.libXi
                xorg.libXcursor
                xorg.libXrandr
                libz libz.dev
                ffmpeg.dev
                ffmpeg
                libxkbcommon
                wayland

                libv4l libv4l.dev
                opencv4

                stdenv.cc.cc

                xz
                libadwaita
              ] ++ (with pkgs.xorg; [
                libX11
                libX11.dev
                libXi
                libXcursor
                libXrandr
                libXft
                libXft.dev
                libXinerama
              ]);
              VSCODE_CODELLDB = "${pkgs.vscode-extensions.vadimcn.vscode-lldb}";
              LD_LIBRARY_PATH = "/run/opengl-driver/lib:${ with pkgs; lib.makeLibraryPath
                nativeBuildInputs
              }";
              PKG_CONFIG_PATH = "${pkgs.gtk4.dev}/lib/pkgconfig";
              RUSTFLAGS = "--cfg nightly";
              FLTK_SCALING_FACTOR = 2;
              OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib";
              OPENSSL_DIR="${pkgs.openssl.dev}";
              shellHook = ''
                export XDG_DATA_DIRS=$XDG_DATA_DIRS:${pkgs.gtk4}/share/gsettings-schemas/gtk4-4.16.3/
                export PATH=$PATH:${pkgs.vscode-extensions.vadimcn.vscode-lldb}/share/vscode/extensions/vadimcn.vscode-lldb/adapter/
                export PATH=$HOME/.cargo/bin:$PATH
                export CARGO_HOME="$HOME/.cargo"
              '';
            };
        });
    };
}
