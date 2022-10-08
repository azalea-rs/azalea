use crate::{Client, Event};
use async_trait::async_trait;
use parking_lot::Mutex;
use std::sync::Arc;

#[derive(Default)]
pub struct Plugin {
    pub state: Arc<Mutex<State>>,
}

#[derive(Default)]
pub struct State {
    jumping_once: bool,
}

pub trait BotTrait {
    fn jump(&self);
}

impl BotTrait for azalea_client::Client {
    /// Try to jump next tick.
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
    async fn handle(self: Arc<Self>, mut bot: Client, event: Arc<Event>) {
        if let Event::Tick = *event {
            let mut state = self.state.lock();
            if bot.jumping() {
                state.jumping_once = false;
            } else if state.jumping_once {
                bot.set_jumping(true);
            }
        }
    }
}
