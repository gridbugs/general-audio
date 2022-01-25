{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell rec {
  packages = with pkgs; [
    rustc
    cargo
    rustPlatform.rustLibSrc
    rust-analyzer
    cargo-watch
    rustfmt
    pkg-config
    alsaLib
    libao
    openal
    libpulseaudio
    udev
  ];

  # Allows rust-analyzer to find the rust source
  RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
}
