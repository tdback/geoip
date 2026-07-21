{
  lib,
  rustPlatform,
  clippy,
}:
let
  pkgAttr = src: attr: (builtins.fromTOML (builtins.readFile "${src}/Cargo.toml")).package.${attr};
in
rustPlatform.buildRustPackage (finalAttrs: {
  pname = pkgAttr finalAttrs.src "name";
  version = pkgAttr finalAttrs.src "version";
  __structuredAttrs = true;

  src = lib.cleanSource ./.;

  cargoHash = "sha256-NGywcF+BwM+3II+uDqkcdCAB0Bryc5NjvqvFQGzoPYg=";

  nativeBuildInputs = [ clippy ];
  preBuild = "cargo clippy -- -W clippy::pedantic -D warnings";
})
