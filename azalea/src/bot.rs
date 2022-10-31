use crate::{Client, Event};
use async_trait::async_trait;
use azalea_core::Vec3;
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
    fn look_at(&mut self, pos: &Vec3);
}

impl BotTrait for azalea_client::Client {
    /// Queue a jump for the next tick.
    fn jump(&self) {
        let player_lock = self.player.lock();
        let mut world_lock = self.world.write();

        let mut player_entity = player_lock
            .entity_mut(&mut world_lock)
            .expect("Player must exist");

        player_entity.jumping = true;
    }

    /// Turn the bot's head to look at the coordinate in the world.
    fn look_at(&mut self, pos: &Vec3) {
        // borrowed from mineflayer's Bot.lookAt because i didn't want to do math
        let delta = self.entity().pos() - pos;
        let x_rot = f64::atan2(-delta.x, -delta.z);
        let ground_distance = f64::sqrt(delta.x * delta.x + delta.z * delta.z);
        let y_rot = f64::atan2(delta.y, ground_distance);
        self.set_rotation(y_rot as f32, x_rot as f32);
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
