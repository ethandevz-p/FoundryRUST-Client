# The server build of the game (`foundry-client --no-default-features`): the game without sound,
# controllers or Foundry Client's online extras, so it links neither ALSA nor udev. foundry-server runs it
# headless as `foundry-engine` (docs/hosting.md, packaging/server/Dockerfile).
{
  lib,
  rustPlatform,
  pkg-config,
  callPackage,
  version,
}:
rustPlatform.buildRustPackage {
  pname = "foundry-engine";
  inherit version;
  src = callPackage ./source.nix { };
  cargoLock.lockFile = ../Cargo.lock;

  cargoBuildFlags = [
    "--package=foundry-client"
    "--bin=foundry-client"
  ];
  buildNoDefaultFeatures = true;

  nativeBuildInputs = [ pkg-config ];

  # The game's tests read the prepared vanilla files in .local/, which a build never has.
  doCheck = false;

  postInstall = ''
    mv $out/bin/foundry-client $out/bin/foundry-engine
  '';

  meta = {
    description = "Foundry Client's headless game engine, run by foundry-server";
    homepage = "https://playfoundry.dev";
    license = lib.licenses.lgpl3Plus;
    mainProgram = "foundry-engine";
    platforms = lib.platforms.linux ++ lib.platforms.darwin;
  };
}
