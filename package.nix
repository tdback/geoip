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

  cargoHash = "sha256-IjiSSVaz0XxVmgBaaLw841T+UefeCxQe80WUa51EOQw=";

  nativeBuildInputs = [ clippy ];
  preBuild = "cargo clippy -- -W clippy::pedantic -D warnings";
})
