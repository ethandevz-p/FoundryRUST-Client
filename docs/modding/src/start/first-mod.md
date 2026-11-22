# Your first native mod

A native mod is a Rust crate that implements hooks from the mod API. The launcher compiles it on your machine
against the exact Foundry Client build an instance plays, then runs that instance with your mod in it. In this page you
make a mod that greets every player who joins.

You need the Foundry Client launcher (`foundry-launcher`) with an instance, and the `cargo foundry-mod` tool, which you
install with Rust's `cargo` ([as in the content mod tutorial](first-content-mod.md#1-install-the-mod-tool)). The
native build itself uses the launcher's own pinned Rust toolchain, not yours.

## 1. Make the project

```sh
cargo foundry-mod new greeter --template native
cd greeter
```

This writes a flat project, pinned to the build of the current Nightly (or to `--foundry <build>`), and fetches
that build's [SDK](../reference/sdk.md):

```text
greeter/
├── foundry-mod.toml     # the manifest; [native] pins the exact Foundry Client build
├── Cargo.toml           # depends on foundry-mod-api in .foundry/sdk/, never on the game
├── src/lib.rs           # the hooks
├── assets/
├── data/
└── .foundry/sdk/        # the pinned SDK: fetched by the tool, never committed or packed
```

After a fresh clone, run any `cargo foundry-mod` command once before opening the project in an editor: it fetches
`.foundry/sdk/` again.

`cargo foundry-mod update` moves the pin to a newer Foundry Client later.

## 2. Read the hook

`src/lib.rs` implements one method of `mod_api::Hooks`:

```rust,ignore
{{#include ../../examples/src/greeter.rs:mod}}
```

- Each method of `Hooks` is one moment in the game. You implement only the ones you need; the game calls
  your mod only for those.
- `#[mod_api::foundry_mod]` marks the one `impl Hooks` at the crate root and subscribes the mod to exactly the
  methods it implements.
- The `host` is your handle on the game. A server hook gets a `ServerHost`, a client hook a `ClientHost`, so a call
  on the wrong side does not compile.
- The `event` carries the moment's data. Values you may change are writable fields of the event.

[Writing hooks](writing-hooks.md) explains these rules in full.

## 3. Build and play

```sh
foundry-launcher native-build . --instance "My Instance" --trust-native-code
```

Install the instance's Foundry Client build first, then choose **Prepare & play** in the launcher or run
`foundry-launcher prepare --version <version>`. The launcher fetches any missing SDK slices for the pinned build
and installs its managed Rust and linker tools as needed; with an empty cache, that can be several hundred MB of
downloads. It compiles your crate against the SDK and makes the resulting build the one the instance plays. `--trust-native-code` records that you allow native code on this
install: a native mod runs with your rights, outside any sandbox. Start the instance and join a world; the
greeting appears in chat.

After an edit, run `native-build` again. The [build log](../how-to/native-mods/05-build-log.md) shows the compiler's
output when a build fails.

## Next

- [Writing hooks](writing-hooks.md): the crate root, the hook factory, events and hosts.
- [Hooks and the Host](../concepts/hooks-and-the-host.md) and [Parts](../concepts/parts.md): how a mod reaches
  the game.
- [Your first WebAssembly mod](first-wasm-mod.md): the same code, sandboxed.
- [Native mods: building and shipping](../how-to/native-mods.md): what the launcher does, trust, multiplayer and
  servers.
