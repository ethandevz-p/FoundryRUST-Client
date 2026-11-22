use crate::game::Game;
use foundry_client::text::Text;
use foundry_gameplay::commands::version::VersionHost;

impl VersionHost for Game<'_> {
    fn source_silent(&self) -> bool {
        self.command_source.is_some_and(|s| s.silent)
    }
    fn chat_system_message(&mut self, text: Text) {
        self.system_message(text);
    }
    fn version_lines(&self) -> Vec<Text> {
        Game::version_lines(self)
    }
}
