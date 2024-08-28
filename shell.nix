{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
    nativeBuildInputs = [
        pkgs.pkg-config
    ];

    buildInputs = [
        pkgs.rustup
        pkgs.openssl
    ];
}