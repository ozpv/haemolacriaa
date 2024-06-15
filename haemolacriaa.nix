{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
    clang
    rustup
    rustc
    rust-analyzer
    rustfmt
    trunk
    rustc-wasm32
    cargo
    cargo-binutils
    cargo-leptos
    cargo-generate
    tailwindcss
    binaryen
    postgresql
    sqlx-cli
  ];
}
