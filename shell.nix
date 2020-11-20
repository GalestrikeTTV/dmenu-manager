with import <nixpkgs> {};

mkShell {
  buildInputs = [
    cargo
    clippy
    pkg-config
    dbus
    x11
  ];
}
