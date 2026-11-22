//! Foundry Client: native Rust client for Minecraft Java Edition 26.3 gameplay.
#![forbid(unsafe_code)]

/// Identifies this executable as the game for launcher imports. `main` reads it so the
/// linker keeps it; it carries neither an API version nor a per-build id.
static GAME_IDENTITY: [u8; foundry_mod_package::game_identity::MARK.len()] =
    foundry_mod_package::game_identity::MARK;

fn main() {
    foundry_guest_stack::run_main(|| {
        std::hint::black_box(&GAME_IDENTITY);
        foundry_game::main();
    });
}
