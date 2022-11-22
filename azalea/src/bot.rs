use crate::{Client, Event};
use async_trait::async_trait;
use azalea_core::Vec3;
use parking_lot::Mutex;
use std::{f64::consts::PI, sync::Arc};

#[derive(Clone, Default)]
pub struct Plugin;
impl crate::Plugin for Plugin {
    type State = State;

    fn build(&self) -> State {
        State::default()
    }
}

#[derive(Default, Clone)]
pub struct State {
    jumping_once: Arc<Mutex<bool>>,
}

#[async_trait]
impl crate::PluginState for State {
    async fn handle(self: Box<Self>, event: Event, mut bot: Client) {
        if let Event::Tick = event {
            if *self.jumping_once.lock() && bot.jumping() {
                *self.jumping_once.lock() = false;
                bot.set_jumping(false);
            }
        }
    }
}

pub trait BotTrait {
    fn jump(&mut self);
    fn look_at(&mut self, pos: &Vec3);
}

impl BotTrait for azalea_client::Client {
    /// Queue a jump for the next tick.
    fn jump(&mut self) {
        self.set_jumping(true);
        let state = self.plugins.get::<State>().unwrap().clone();
        *state.jumping_once.lock() = true;
    }

    /// Turn the bot's head to look at the coordinate in the world.
    fn look_at(&mut self, pos: &Vec3) {
        let (y_rot, x_rot) = direction_looking_at(self.entity().pos(), pos);
        self.set_rotation(y_rot, x_rot);
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
