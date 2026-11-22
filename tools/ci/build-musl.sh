#!/bin/sh
# Called inside rust:alpine3.22 by game-build.yml, without release credentials.
set -eu

apk add --no-cache build-base pkgconf linux-headers alsa-lib-dev eudev-dev python3 git bash
git config --global --add safe.directory /work
# Match the shared release/SDK pin, not the image's bundled Rust or the moving stable channel.
bash tools/ci/rustup_install.sh --profile minimal --component llvm-tools
export RUSTUP_TOOLCHAIN="$(cat tools/ci/rust-version)"

# Static musl cannot dlopen the graphics/window libraries. Keep audio and controllers,
# and link against Alpine's native libraries rather than Ubuntu's glibc libraries.
export RUSTFLAGS='-C target-feature=-crt-static'
target=x86_64-unknown-linux-musl
bin="target/$target/release"
cargo build --release --locked --target "$target" \
  -p foundry-official --bin foundry-official -p foundry-client-lib --bin foundry-ctl \
  -p foundry-prepare --bin foundry-prepare -p foundry-server --bin foundry-server
# The official game (foundry-client's with Foundry Client's own parts, docs/layers.md) is what players
# get, under the game's name.
cp -f "$bin/foundry-official" "$bin/foundry-client"
cp wasm-std/foundry-wasm-std.wasm "$bin/foundry-wasm-std.wasm"

set -- "$bin/foundry-client" "$bin/foundry-ctl" "$bin/foundry-prepare" "$bin/foundry-server"
python3 tools/check_imports.py --musl "$@"
python3 tools/check_cpu_baseline.py "$@"

# Even --help must never touch real player data.
export FOUNDRY_HOME="$(mktemp -d)"
export FOUNDRY_LAUNCHER_HOME="$(mktemp -d)"
export XDG_DATA_HOME="$(mktemp -d)"
export XDG_CONFIG_HOME="$(mktemp -d)"
export FOUNDRY_LAUNCHER_NO_SELF_UPDATE=1 FOUNDRY_NO_AUDIO=1
"$bin/foundry-client" --help > "$FOUNDRY_HOME/client-help"
grep -q -- '--resources' "$FOUNDRY_HOME/client-help"
"$bin/foundry-ctl" --help > /dev/null
"$bin/foundry-prepare" --help > "$FOUNDRY_HOME/prepare-help"
grep -q -- '--minecraft-dir' "$FOUNDRY_HOME/prepare-help"
for binary in foundry-prepare foundry-server; do
  test "$("$bin/$binary" --version)" = "$binary $FOUNDRY_VERSION"
done

cargo run --locked --target "$target" -p xtask -- package \
  "$bin/foundry-client" dist/foundry-linux-musl-x86_64
tar -C dist -czf dist/foundry-linux-musl-x86_64.tar.gz foundry-linux-musl-x86_64
