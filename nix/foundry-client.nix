# The game (foundry-client, with sound, controllers and Foundry Client's extras) and foundry-ctl, the tool
# that drives it (docs/agent-control.md). Start it with `foundry-client --resources DIR`, DIR being
# what foundry-prepare wrote.
{
  lib,
  stdenv,
  rustPlatform,
  pkg-config,
  callPackage,
  alsa-lib,
  udev,
  vulkan-loader,
  wayland,
  libxkbcommon,
  libx11,
  libxcursor,
  libxi,
  libxrandr,
  libxcb,
  version,
}:
let
  # wgpu and winit open these with dlopen, so the linker never records them.
  dlopened = [
    vulkan-loader
    wayland
    libxkbcommon
    libx11
    libxcursor
    libxi
    libxrandr
    libxcb
  ];
in
rustPlatform.buildRustPackage {
  pname = "foundry-client";
  inherit version;
  src = callPackage ./source.nix { };
  cargoLock.lockFile = ../Cargo.lock;

  cargoBuildFlags = [
    "--package=foundry-client"
    "--bin=foundry-client"
    "--package=foundry-client-lib"
    "--bin=foundry-ctl"
  ];

  nativeBuildInputs = [ pkg-config ];
  buildInputs = lib.optionals stdenv.hostPlatform.isLinux [
    alsa-lib
    udev
  ];

  doCheck = false;

  postFixup = lib.optionalString stdenv.hostPlatform.isLinux ''
    patchelf --add-rpath ${lib.makeLibraryPath dlopened} $out/bin/foundry-client
  '';

  meta = {
    description = "Foundry Client, Minecraft: Java Edition 26.3 rewritten in Rust, and foundry-ctl";
    homepage = "https://playfoundry.dev";
    license = lib.licenses.lgpl3Plus;
    mainProgram = "foundry-client";
    platforms = lib.platforms.linux ++ lib.platforms.darwin;
  };
}
