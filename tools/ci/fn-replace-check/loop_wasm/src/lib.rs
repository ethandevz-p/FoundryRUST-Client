//! Loop rows the build refuses by name: nothing here is compiled into the game.
#[derive(Default)]
struct Mod;

#[mod_api::foundry_mod]
impl mod_api::Hooks for Mod {}
