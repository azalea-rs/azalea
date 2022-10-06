//! Automatically eat when we get hungry.

use azalea::{Client, Event};
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct State {}

pub async fn handle(bot: &mut Client, event: Event, state: Arc<Mutex<State>>) {
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
