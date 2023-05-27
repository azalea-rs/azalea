//! Steal all the diamonds from all the nearby chests.

use azalea::{prelude::*, BlockPos};
use azalea_inventory::operations::QuickMoveClick;
use azalea_inventory::ItemSlot;
use parking_lot::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let account = Account::offline("bot");
    // or let bot = Account::microsoft("email").await.unwrap();

    ClientBuilder::new()
        .set_handler(handle)
        .start(account, "localhost")
        .await
        .unwrap();
}

#[derive(Default, Clone, Component)]
struct State {
    pub checked_chests: Arc<Mutex<Vec<BlockPos>>>,
}

async fn handle(mut bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Chat(m) => {
            if m.username() == Some(bot.profile.name.clone()) {
                return Ok(());
            };
            if m.content() != "go" {
                return Ok(());
            }
            {
                state.checked_chests.lock().clear();
            }

            let chest_block = bot
                .world()
                .read()
                .find_block(bot.position(), &azalea::Block::Chest.into());
            // TODO: update this when find_blocks is implemented
            let Some(chest_block) = chest_block else {
                bot.chat("No chest found");
                return Ok(());
            };
            // bot.goto(BlockPosGoal::from(chest_block));
            let Some(chest) = bot.open_container(chest_block).await else {
                println!("Couldn't open chest");
                return Ok(());
            };

            println!("Getting contents");
            for (index, slot) in chest
                .contents()
                .expect("we just opened the chest")
                .iter()
                .enumerate()
            {
                println!("Checking slot {index}: {slot:?}");
                if let ItemSlot::Present(item) = slot {
                    if item.kind == azalea::Item::Diamond {
                        println!("clicking slot ^");
                        chest.click(QuickMoveClick::Left { slot: index as u16 });
                    }
                }
            }

            println!("Done");
        }
        _ => {}
    }

    Ok(())
}
