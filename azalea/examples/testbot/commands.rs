pub mod combat;
pub mod debug;
pub mod movement;

use azalea::brigadier::prelude::*;
use azalea::chat::ChatPacket;
use azalea::ecs::prelude::*;
use azalea::entity::metadata::Player;
use azalea::Client;
use azalea::GameProfileComponent;
use parking_lot::Mutex;

use crate::State;

pub type Ctx = CommandContext<Mutex<CommandSource>>;

pub struct CommandSource {
    pub bot: Client,
    pub state: State,
    pub chat: ChatPacket,
}

impl CommandSource {
    pub fn reply(&self, message: &str) {
        if self.chat.is_whisper() {
            self.bot
                .chat(&format!("/w {} {}", self.chat.username().unwrap(), message));
        } else {
            self.bot.chat(message);
        }
    }

    pub fn entity(&mut self) -> Option<Entity> {
        let username = self.chat.username()?;
        self.bot.entity_by::<With<Player>, &GameProfileComponent>(
            |profile: &&GameProfileComponent| profile.name == username,
        )
    }
}

pub fn register_commands(commands: &mut CommandDispatcher<Mutex<CommandSource>>) {
    combat::register(commands);
    debug::register(commands);
    movement::register(commands);
}
