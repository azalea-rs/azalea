use crate::{Client, Event};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct Plugin {
    pub state: Arc<Mutex<State>>,
}

#[derive(Default)]
pub struct State {
    jumping_once: bool,
}

pub trait BotTrait {
    fn jump(&mut self);
}

impl BotTrait for azalea_client::Client {
    fn jump(&mut self) {
        let player_lock = self.player.lock().unwrap();
        let mut dimension_lock = self.dimension.lock().unwrap();

        let mut player_entity = player_lock
            .entity_mut(&mut dimension_lock)
            .expect("Player must exist");

        player_entity.jumping = true;
    }
}

#[async_trait]
impl crate::Plugin for Plugin {
    async fn handle(&self, bot: Client, event: Arc<Event>) {
        match *event {
            Event::GameTick => {
                let state = self.state.lock().unwrap();
                let player_entity = bot.player_entity().unwrap();
                if state.jumping_once {
                    bot.set_jumping(true);
                    state.jumping_once = false;
                }
            }
            _ => {}
        }
    }
}
