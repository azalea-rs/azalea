{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    fenix.url   = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, fenix }:
  let
    system = "x86_64-linux"; # change if needed (e.g., "aarch64-linux")
    pkgs   = import nixpkgs { inherit system; };

    # Libraries we want available from nixpkgs (NO wayland here)
    libPath = pkgs.lib.makeLibraryPath [
      pkgs.stdenv.cc.cc               # libstdc++.so.6
      pkgs.libxkbcommon               # keyboard support for winit
      pkgs.vulkan-loader              # libvulkan.so
      pkgs.vulkan-validation-layers   # validation layer runtime
      pkgs.xorg.libX11
      pkgs.xorg.libXcursor
      pkgs.xorg.libXi
      pkgs.xorg.libXrandr
    ];

    # pkg-config paths (exclude wayland to avoid ABI clashes)
    pcPath = pkgs.lib.makeSearchPathOutput "dev" "lib/pkgconfig" [
      pkgs.libxkbcommon
      pkgs.xorg.libX11
      pkgs.xorg.libXcursor
      pkgs.xorg.libXi
      pkgs.xorg.libXrandr
    ];
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = [
        # Nightly toolchain (needed for crates using feature gates)
        fenix.packages.${system}.default.toolchain

        # Build tools
        pkgs.gcc pkgs.binutils pkgs.pkg-config
        pkgs.cmake pkgs.ninja pkgs.python3

        # X11 + Vulkan runtime helpers
        pkgs.libxkbcommon
        pkgs.xorg.libX11 pkgs.xorg.libXcursor pkgs.xorg.libXi pkgs.xorg.libXrandr
        pkgs.vulkan-loader
        pkgs.vulkan-validation-layers
        pkgs.vulkan-tools   # optional: vkcube, vulkaninfo
      ];

      shellHook = ''
        export LD_LIBRARY_PATH=${libPath}''${LD_LIBRARY_PATH:+:}''$LD_LIBRARY_PATH

        # Prefer X11, even on Wayland sessions
        export WINIT_UNIX_BACKEND=x11
        unset WAYLAND_DISPLAY
      '';
    };
  };
}
