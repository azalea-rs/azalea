//! Automatically eat when we get hungry.

use async_trait::async_trait;
use azalea::{Client, Event};
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct Plugin {
    pub state: Arc<Mutex<State>>,
}

#[derive(Default)]
pub struct State {}

#[async_trait]
impl azalea::Plugin for Plugin {
    async fn handle(self: Arc<Self>, bot: Client, event: Arc<Event>) {
        match event {
            Event::UpdateHunger => {
                if !bot.using_held_item() && bot.food_level() <= 17 {
                    if bot.hold(azalea::ItemGroup::Food).await {
                        bot.use_held_item().await;
                    }
                }
            }
            _ => {}
        }
    }
}
