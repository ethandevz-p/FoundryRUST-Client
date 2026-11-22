//! Gameplay and startup, above the reusable client library.
// No `#![recursion_limit]` or other inner attribute foundry-mixins doesn't know here: the
// launcher's foundry-mixins then rejects every native mod targeting this crate (#8169).
#![forbid(unsafe_code)]
extern crate self as foundry_client;
pub use foundry_client_lib::carrier_store;
pub use foundry_client_lib::*;
#[doc(hidden)]
pub mod entry;

// The game's allocator lives in the library, not the executable: the prebuilt game (arismoko/foundry#589) is this crate as a
// dylib, and the allocator must be the one its code and the mods' executable share. The Windows and macOS archives
// build with `system-allocator`: their loaders bind each library to std's own allocator, which everything then
// shares (arismoko/foundry#2394, arismoko/foundry#3877).
#[cfg(not(feature = "system-allocator"))]
#[global_allocator]
static ALLOCATOR: foundry_threads::GameAllocator = foundry_threads::GAME_ALLOCATOR;
pub mod game;
pub mod orb_limits;
