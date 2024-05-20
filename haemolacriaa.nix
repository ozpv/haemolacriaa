{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
    rustup
    rustfmt
    trunk
    rustc-wasm32
    cargo-leptos
    cargo-generate
    tailwindcss
    binaryen
    sqlite
    sqlx-cli
  ];
}
