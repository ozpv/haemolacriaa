{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
    rustup
    cargo-binutils
    cargo-leptos
    cargo-generate
    tailwindcss
    binaryen
    postgresql
    sqlx-cli
	wasm-bindgen-cli
  ];
}
