let
  nixpkgs = import <nixpkgs> {};
  rust_pkgs = nixpkgs.rust.packages.stable;
in
  with nixpkgs;
  mkShell rec {
    buildInputs = [
      rust_pkgs.cargo
      rust_pkgs.rustc
      rust_pkgs.rustfmt
      rust_pkgs.clippy
      rust-analyzer
      libclang
    ];

    nativeBuildInputs = [ clang pkgconfig ];

    #RUST_BACKTRACE = 1;
    LIBCLANG_PATH = "${libclang.lib}/lib";
    RUST_SRC_PATH = rust_pkgs.rustPlatform.rustLibSrc;
    LD_LIBRARY_PATH = lib.makeLibraryPath (buildInputs ++ nativeBuildInputs);
  }
