//! The game as the platform runs it, for an executable to start (docs/layers.md, "How a Foundry Client
//! crate plugs in"). The official executable (`foundry-official`, Foundry Client layer) registers its
//! hooks (`foundry_platform::hooks`) and then calls [`main`]; it never names an engine crate.
//!
//! An executable built on it:
//!
//! ```ignore
//! static GAME_IDENTITY: [u8; foundry_game::GAME_IDENTITY.len()] = foundry_game::GAME_IDENTITY;
//!
//! fn main() {
//!     foundry_game::run_main(|| {
//!         std::hint::black_box(&GAME_IDENTITY);
//!         my_crate::register();
//!         foundry_game::main();
//!     });
//! }
//! ```
//!
//! The platform's own parts register here, in [`main`], as they leave the engine (layer plan M6).
#![forbid(unsafe_code)]

/// Identifies an executable as the game for launcher imports, without an API version or build id.
pub const GAME_IDENTITY: [u8; foundry_mod_package::game_identity::MARK.len()] =
    foundry_mod_package::game_identity::MARK;

/// Runs the game with the hooks registered so far, and returns when it ends. Register hooks
/// (`foundry_platform::hooks::register`) before calling it.
pub fn main() {
    foundry_guest_stack::prepare_main();
    foundry_mod_host::exit_if_compile_child();
    let startup = foundry_client::entry::prepare();
    std::hint::black_box(&foundry_mod_package::holder::CAPABILITY);
    foundry_mod_host::main_with(Vec::new(), &[], &[], startup);
}

/// Establish the executable main thread stack before initialization.
pub use foundry_guest_stack::prepare_main as prepare_main_stack;
/// Run an executable's whole `main` on a main thread stack the guest fits on.
pub use foundry_guest_stack::run_main;
