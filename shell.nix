{
  pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-25.05.tar.gz") {
    overlays = [
      (import (fetchTarball "https://github.com/oxalica/rust-overlay/archive/stable.tar.gz"))
    ];
  },
}:

pkgs.mkShell rec {
  buildInputs = with pkgs; [

    (rust-bin.selectLatestNightlyWith (
      toolchain:
      toolchain.default.override {
        extensions = [
          "rust-src"
          "rust-analyzer"
        ];
      }
    ))

    expat
    fontconfig
    freetype
    freetype.dev
    libGL
    pkg-config
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
    wayland
    libxkbcommon
  ];

  LD_LIBRARY_PATH = builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" buildInputs;

  shellHook = ''
    clear
    echo "Hello, world!"
  '';
}
