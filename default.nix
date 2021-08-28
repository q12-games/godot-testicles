let
  nixpkgs = import <nixpkgs> {};
  rust_channel = nixpkgs.rust.packages.stable;
in
  with nixpkgs;
  mkShell rec {
    buildInputs = [
      # Build
      pkgconfig
      patchelf
      rustup

      # Dev
      rust-analyzer

      # Lib
      libclang
      xorg.libX11
      xorg.libXi
      libGL
      xorg.libXcursor
      xorg.libXinerama
      xorg.libXext
      xorg.libXrandr
      xorg.libXrender
    ];
    nativeBuildInputs = [ clang ];

    #RUST_BACKTRACE = 1;
    LIBCLANG_PATH = "${libclang.lib}/lib";
    RUST_SRC_PATH = rust_channel.rustPlatform.rustLibSrc;
    LD_LIBRARY_PATH = lib.makeLibraryPath (buildInputs ++ nativeBuildInputs);
  }
