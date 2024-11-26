{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
    rustup
    trunk
    cargo-binutils
    cargo-leptos
    cargo-generate
    tailwindcss
    binaryen
    postgresql
    sqlx-cli
  ];
}
