//! The platform's public API: what the Foundry Client layer, mods and ports may use of the engine
//! (docs/layers.md#platform). The Foundry Client layer depends on the platform only, never on an engine
//! crate, so it can build from another repository; when it needs something the engine has, it
//! comes through here.
//!
//! Today this re-exports engine modules whole. Each one gets its own stable types as the split
//! goes on (docs/layers.md#plan, M6). The parts that pull in large engine crates sit behind
//! features (`world`, `protocol`, `prepare`, `server-files`), so a crate builds only what it uses.

/// The machine the game runs on (`foundry-system`): OS, CPU, memory, drive, power, GPU maker and
/// memory.
pub use foundry_system as system;

/// The benchmark report format and its hardware probes (`foundry-bench`).
pub use foundry_bench as bench;

/// The `.hzperf` capture contract and its files (`foundry-perf-format`).
pub use foundry_perf_format as perf_format;

/// The pinned Minecraft version: its id, protocol and data version.
pub use foundry_core::version;

/// The player's "no AI" switch, which the launcher, the game and their agent tools share.
pub use foundry_core::no_ai;

/// The prepared folder's file of files (`prepared_files::FILE_NAME`).
pub use foundry_core::prepared_files;

/// Vanilla's world files, as tools outside the game read and write them.
#[cfg(feature = "world")]
pub mod world {
    pub use foundry_core::nbt;
    pub use foundry_core::position::ChunkPos;
    pub use foundry_world::{compression, region};
}

/// What a server list and a profile head need of the network protocol.
#[cfg(feature = "protocol")]
pub mod protocol {
    pub use foundry_protocol::crypto::offline_uuid;
    pub use foundry_protocol::server_redirect;
    /// What a server that logs players in on behalf of another (`foundry-server`'s `[proxy]`) speaks: the
    /// codecs, packets, ids, connection framing, encryption and the account-service hash.
    pub use foundry_protocol::{
        Error, PROTOCOL_VERSION, buf, connection, crypto, ids, packets, rsa,
    };
}

/// The preparer: the game's files from the player's own Minecraft install.
#[cfg(feature = "prepare")]
pub use foundry_prepare as prepare;

/// A server's own files: `server.properties`, the op, white and ban lists, the user cache.
#[cfg(feature = "server-files")]
pub use foundry_server_files as server_files;

/// `java.util.Properties` and the JDK's date text, for the files a server keeps in that format.
#[cfg(feature = "server-files")]
pub use foundry_java_properties as java_properties;

/// Plugging into the game (`foundry-hooks`): implement [`hooks::Hooks`] and
/// [`hooks::register`] it before the game starts (docs/layers.md, "How a Foundry Client crate plugs
/// in"). The engine's dispatch side is not part of the API.
pub mod hooks {
    /// The players the game shows (`foundry_hooks::players`): the views [`Hooks::frame`] and
    /// [`Hooks::tick`] get, the pose a crate lays over a player's body, what goes by their names,
    /// the crate's sprites, the camera, and what a crate draws on them.
    pub use foundry_hooks::players::{
        BODY_PARTS, Badge, BodyPose, Bone, CameraOverride, CameraView, Frame, Gear, Kind, Light,
        Look, Mark, Mesh, Motion, NameDecoration, PartPoses, Particle, Pass, PlayerDraw,
        PlayerModel, PlayerModels, Quad, SpriteStore, Sprites, View,
    };
    pub use foundry_hooks::{
        Activity, Client, Dimension, GameMode, GetTheGame, Hooks, Play, Started, TooLate, perf,
        register, seat, tunnel,
    };
}

/// The settings store (`foundry_hooks::settings`): a crate's own `key:value` lines in the game
/// folder, loaded and saved with the player's settings (docs/layers.md, "Platform registries").
pub mod settings {
    pub use foundry_hooks::settings::{Store, register};
}

/// Settings pages a crate adds to the options screens (docs/layers.md, "Platform registries").
pub mod pages {
    pub use foundry_hooks::pages::{Act, Label, Page, Place, Row, connect, register};
}

/// The lang files with a crate's text, loaded with the game's own (docs/layers.md, "Platform
/// registries").
pub mod lang {
    pub use foundry_hooks::lang::{Files, register};
}

/// Drawing on the HUD (docs/layers.md, "Platform registries").
pub mod hud {
    pub use foundry_hooks::hud::{Canvas, Corner, Layer, register};
}

/// The multiplayer server list as vanilla stores it (`servers.dat`): read, write, add, remove,
/// and the lock held while it is read and written back.
#[cfg(feature = "protocol")]
pub mod servers {
    pub use foundry_protocol::servers::{
        ServerEntry, load, lock, parse_address, prepend, remove_where, save,
    };
}

/// Build-time mod source generation.
#[cfg(feature = "mod-build")]
pub mod mod_build {
    /// The shared mod API and source generator.
    pub use foundry_mod_api_gen as api_gen;
}

/// Mod package formats, installation, SDK metadata and registry validation.
/// Enable `mod-index`, `mod-prebuilt` or `mod-runtime-process` for the corresponding services.
#[cfg(feature = "mod-package")]
pub use foundry_mod_package as mod_package;

/// Links supplied by installed products for their built-in mod entries.
pub mod mod_links {
    pub use foundry_hooks::mod_links::{Links, register};
}

/// Establish the executable stack policy before starting the game or supervisor.
pub use foundry_guest_stack::prepare_main as prepare_main_stack;
/// Run an executable's whole `main` on a main thread stack the guest fits on.
pub use foundry_guest_stack::run_main;
