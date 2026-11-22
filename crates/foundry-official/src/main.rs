//! The official game executable (docs/layers.md, "How a Foundry Client crate plugs in").
#![forbid(unsafe_code)]

/// The game identity marker for launcher imports, retained by `main`.
static GAME_IDENTITY: [u8; foundry_game::GAME_IDENTITY.len()] = foundry_game::GAME_IDENTITY;

fn main() {
    foundry_game::run_main(|| {
        std::hint::black_box(&GAME_IDENTITY);
        foundry_official::register();
        foundry_game::main();
    });
}
