{ nixpkgs, system}:
let
  pkgs = nixpkgs.legacyPackages.${system};
  manifest = (nixpkgs.lib.importTOML ./Cargo.toml).package;
in
pkgs.rustPlatform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;

  nativeBuildInputs = with pkgs; [ openssl pkg-config bacon rust-analyzer];
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;
}
