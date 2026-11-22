# foundry-server: server.toml, the CLI and the supervisor. It finds the game beside itself, so the
# engine is linked into the same bin/ (crates/foundry-server/src/engine.rs `program_in`).
{
  lib,
  rustPlatform,
  pkg-config,
  callPackage,
  foundry-engine,
  version,
}:
rustPlatform.buildRustPackage {
  pname = "foundry-server";
  inherit version;
  src = callPackage ./source.nix { };
  cargoLock.lockFile = ../Cargo.lock;

  cargoBuildFlags = [
    "--package=foundry-server"
    "--bin=foundry-server"
  ];

  nativeBuildInputs = [ pkg-config ];

  doCheck = false;

  postInstall = ''
    ln -s ${lib.getExe foundry-engine} $out/bin/foundry-engine
  '';

  passthru.engine = foundry-engine;

  meta = {
    description = "Host a Minecraft: Java Edition 26.3 server with Foundry Client, the game rewritten in Rust";
    longDescription = ''
      Holds none of Mojang's files: the server reads the game files you prepare from your own copy of
      Minecraft 26.3 with foundry-prepare (docs/hosting.md).
    '';
    homepage = "https://playfoundry.dev";
    license = lib.licenses.lgpl3Plus;
    mainProgram = "foundry-server";
    platforms = lib.platforms.linux ++ lib.platforms.darwin;
  };
}
