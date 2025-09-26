{
  pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-25.05.tar.gz") {
    overlays = [
      (import (fetchTarball "https://github.com/oxalica/rust-overlay/archive/stable.tar.gz"))
    ];
  },
}:

pkgs.mkShell rec {
  buildInputs = with pkgs; [
    bacon

    (rust-bin.stable.latest.default.override {
      extensions = [
        "rust-src"
        "rust-analyzer"
      ];
    })

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
    git submodule update --remote --force
    cargo update iced iced_test
    clear
    echo "Hello, world!"
  '';
}
