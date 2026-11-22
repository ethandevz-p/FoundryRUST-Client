# foundry-prepare: builds the game files Foundry Client reads from the player's own Minecraft 26.3 and the
# server jar they downloaded themselves. Nothing of Mojang's is fetched or packaged here.
#
# It works in a "root": research-lock.json (the pinned jars' hashes), the exporters' Java sources it
# compiles against those jars, and a writable .local/ for its scratch files. The store is read-only, so
# the package keeps that root in share/foundry/prepare and bin/foundry-prepare (foundry-prepare.sh)
# copies it somewhere writable unless --root is given. The exporters need a JDK of the pinned Java
# version (25): the default JAVA_HOME.
{
  lib,
  stdenv,
  rustPlatform,
  callPackage,
  coreutils,
  jdk25_headless,
  version,
}:
rustPlatform.buildRustPackage {
  pname = "foundry-prepare";
  inherit version;
  src = callPackage ./source.nix { };
  cargoLock.lockFile = ../Cargo.lock;

  cargoBuildFlags = [
    "--package=foundry-prepare"
    "--bin=foundry-prepare"
  ];

  doCheck = false;

  postInstall = ''
    root=$out/share/foundry/prepare
    install -Dm444 research-lock.json -t $root
    install -Dm444 crates/foundry-prepare/src/exporters.rs -t $root/crates/foundry-prepare/src
    mkdir -p $root/mods/foundry/src/test
    cp -r mods/foundry/src/test/java $root/mods/foundry/src/test/

    mkdir -p $out/libexec
    mv $out/bin/foundry-prepare $out/libexec/foundry-prepare
    substitute ${./foundry-prepare.sh} $out/bin/foundry-prepare \
      --subst-var out \
      --subst-var-by shell ${stdenv.shell} \
      --subst-var-by coreutils ${coreutils} \
      --subst-var-by java ${jdk25_headless}
    chmod +x $out/bin/foundry-prepare
  '';

  passthru.java = jdk25_headless;

  meta = {
    description = "Prepare Foundry Client's game files from your own copy of Minecraft: Java Edition 26.3";
    homepage = "https://playfoundry.dev";
    license = lib.licenses.lgpl3Plus;
    mainProgram = "foundry-prepare";
    platforms = lib.platforms.linux ++ lib.platforms.darwin;
  };
}
