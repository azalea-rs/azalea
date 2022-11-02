use crate::{Client, Event};
use async_trait::async_trait;
use azalea_core::Vec3;
use parking_lot::Mutex;
use std::{f64::consts::PI, sync::Arc};

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
        {
            let player_entity_id = self.player.read().entity_id;
            let mut dimension_lock = self.dimension.write();
            let mut player_entity = dimension_lock
                .entity_mut(player_entity_id)
                .expect("Player must exist");
            player_entity.jumping = true;
        }
        let state = self.plugins.get::<Plugin>().unwrap().state.clone();
        *state.jumping_once.lock() = true;
    }

    /// Turn the bot's head to look at the coordinate in the world.
    fn look_at(&mut self, pos: &Vec3) {
        let (y_rot, x_rot) = direction_looking_at(self.entity().pos(), pos);
        self.set_rotation(y_rot, x_rot);
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

fn direction_looking_at(current: &Vec3, target: &Vec3) -> (f32, f32) {
    // borrowed from mineflayer's Bot.lookAt because i didn't want to do math
    let delta = target - current;
    let y_rot = (PI - f64::atan2(-delta.x, -delta.z)) * (180.0 / PI);
    let ground_distance = f64::sqrt(delta.x * delta.x + delta.z * delta.z);
    let x_rot = f64::atan2(delta.y, ground_distance) * -(180.0 / PI);
    (y_rot as f32, x_rot as f32)
}
