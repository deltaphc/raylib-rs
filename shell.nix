{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  packages = with pkgs; [
    rustup
    glfw
    cmake
    clang
    wayland
    # Web support (uncomment to enable) -- Untested - @JamesKEbert
    # emscripten
  ];
  
  LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
    libGL
    xorg.libXrandr
    xorg.libXinerama
    xorg.libXcursor
    xorg.libXi
  ];
  LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
}