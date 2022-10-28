use crate::{Client, Event};
use async_trait::async_trait;
use parking_lot::Mutex;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct Plugin {
    pub state: State,
}

#[derive(Default, Clone)]
pub struct State {
    jumping_once: Arc<Mutex<bool>>,
}

pub trait BotTrait {
    fn jump(&self);
}

impl BotTrait for azalea_client::Client {
    /// Queue a jump for the next tick.
    fn jump(&self) {
        let player_lock = self.player.lock();
        let mut dimension_lock = self.dimension.lock();

        let mut player_entity = player_lock
            .entity_mut(&mut dimension_lock)
            .expect("Player must exist");

        player_entity.jumping = true;
    }
}

#[async_trait]
impl crate::Plugin for Plugin {
    async fn handle(self: Box<Self>, event: Event, mut bot: Client) {
        if let Event::Tick = event {
            if *self.state.jumping_once.lock() {
                if bot.jumping() {
                    *self.state.jumping_once.lock() = false;
                } else {
                    bot.set_jumping(true);
                }
            }
        }
    }
}
