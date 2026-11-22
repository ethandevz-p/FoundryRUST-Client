//! The official game: the platform's game (`foundry-game`) with Foundry Client's own in-game parts
//! plugged in through the platform's hooks (docs/layers.md, "How a Foundry Client crate plugs in").
//! The release builds ship its executable under the game's name, `foundry-client`
//! (`game-build.yml`); foundry-client keeps its own plain executable.
//!
//! Each Foundry Client crate that runs in the game gives a `&'static` value implementing
//! `foundry_platform::hooks::Hooks`, and its settings store and pages if it has any, and
//! [`register`] adds them. Nothing here depends on an engine crate.
#![forbid(unsafe_code)]

use foundry_platform::hooks::{self, Hooks};
use foundry_platform::{mod_links, pages, settings};

/// Every Foundry Client part in the game, in the order its hooks run.
const PARTS: [&dyn Hooks; 5] = [
    // The anonymous "I'm playing" ping.
    &foundry_heartbeat::HOOKS,
    // Play with Friends: `friend:` addresses join through its tunnel.
    &foundry_friends::game::HOOKS,
    // The official server in the player's server list.
    &foundry_official_servers::HOOKS,
    // "Playing Foundry Client" on the player's Discord profile.
    &foundry_discord::HOOKS,
    // Open and Share for saved performance captures.
    &foundry_perf_share::HOOKS,
];
/// Their settings, in the player's settings files.
const STORES: [&dyn settings::Store; 1] = [&foundry_discord::SETTING];
/// Their rows and pages in the options screens.
const PAGES: [&dyn pages::Page; 1] = [&foundry_discord::SECTION];

/// The product chooses the built-in Mods screen entries' outside links.
static MOD_LINKS: [mod_links::Links; 2] = [
    mod_links::Links {
        id: "minecraft",
        license: None,
        homepage: Some(foundry_addresses::MINECRAFT_WEBSITE),
        issues: Some(foundry_addresses::MINECRAFT_ISSUES),
    },
    mod_links::Links {
        id: "foundry",
        license: Some(foundry_addresses::LICENSE),
        homepage: Some(foundry_addresses::WEBSITE),
        issues: Some(foundry_addresses::ISSUES),
    },
];

/// Registers Foundry Client's in-game parts with the game. The executable calls it before it runs the
/// game, and so does the launcher's generated executable for native mods.
pub fn register() {
    let late = PARTS.into_iter().map(hooks::register);
    let late = late.chain(STORES.into_iter().map(settings::register));
    let late = late.chain(MOD_LINKS.iter().map(mod_links::register));
    for result in late.chain(PAGES.into_iter().map(pages::register)) {
        if let Err(e) = result {
            // The executable registered after starting the game: a bug, not a player's problem.
            log::warn!("Foundry Client: an in-game part did not register: {e}");
        }
    }
}
