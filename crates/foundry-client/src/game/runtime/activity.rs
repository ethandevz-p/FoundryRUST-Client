//! What the player is doing, for the hooks above the engine (`Hooks::activity`: the Foundry Client
//! layer's Discord presence shows it).
use super::*;
use foundry_client::command::args::GameMode;
use foundry_hooks::{Activity, Client, Dimension, Play};

/// At startup, for a game window (never a dedicated server's): the player's first activity, then
/// `Hooks::client_started`.
pub(super) fn client_started(options: &Options) {
    if options.dedicated.is_some() {
        return;
    }
    let automated = options.benchmark.is_some()
        || options.offscreen
        || options.frames.is_some()
        || options.world_frames.is_some();
    foundry_hooks::engine::activity(Activity::Menus);
    foundry_hooks::engine::client_started(&Client::new(automated));
}

impl Game<'_> {
    /// Each frame: only a change reaches the hooks.
    pub(super) fn report_activity(&self) {
        // One status per computer: P1's, not a splitscreen player's.
        if self.options.split_guest {
            return;
        }
        foundry_hooks::engine::activity(self.activity());
    }
    fn activity(&self) -> Activity {
        if self.title.is_some() {
            return Activity::Menus;
        }
        let max = foundry_client::lan::MAX_PLAYERS;
        let play = if let Some(host) = &self.lan {
            Play::Hosting {
                players: 1 + host.hub.guests.len(),
                max,
            }
        } else if let Some(server) = &self.integrated {
            match (server.published(), &self.remote) {
                (Some(_), Some(remote)) => Play::Hosting {
                    players: remote.listed_player_count(),
                    max,
                },
                _ => Play::Singleplayer,
            }
        } else if self.remote.is_some() {
            Play::Multiplayer
        } else {
            Play::Singleplayer
        };
        Activity::World {
            play,
            dimension: match self.dim {
                Dim::Overworld => Dimension::Overworld,
                Dim::Nether => Dimension::Nether,
                Dim::End => Dimension::End,
                Dim::Extra(_) => Dimension::Other,
            },
            mode: match self.mode {
                GameMode::Survival => foundry_hooks::GameMode::Survival,
                GameMode::Creative => foundry_hooks::GameMode::Creative,
                GameMode::Adventure => foundry_hooks::GameMode::Adventure,
                GameMode::Spectator => foundry_hooks::GameMode::Spectator,
            },
        }
    }
}
