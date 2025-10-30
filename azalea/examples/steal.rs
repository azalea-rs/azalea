//! Steal all the diamonds from all the nearby chests.

use std::sync::Arc;

use azalea::{BlockPos, pathfinder::goals::RadiusGoal, prelude::*};
use azalea_inventory::{ItemStack, operations::QuickMoveClick};
use parking_lot::Mutex;

#[tokio::main(flavor = "current_thread")]
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
    pub is_stealing: Arc<Mutex<bool>>,
    pub checked_chests: Arc<Mutex<Vec<BlockPos>>>,
}

async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    if let Event::Chat(m) = event {
        if m.sender() == Some(bot.username()) {
            return Ok(());
        };
        if m.content() != "go" {
            return Ok(());
        }

        steal(bot, state).await?;
    }

    Ok(())
}

async fn steal(bot: Client, state: State) -> anyhow::Result<()> {
    {
        let mut is_stealing = state.is_stealing.lock();
        if *is_stealing {
            bot.chat("Already stealing");
            return Ok(());
        }
        *is_stealing = true;
    }

    state.checked_chests.lock().clear();

    loop {
        let chest_block = bot
            .world()
            .read()
            .find_blocks(bot.position(), &azalea::registry::Block::Chest.into())
            .find(
                // find the closest chest that hasn't been checked
                |block_pos| !state.checked_chests.lock().contains(block_pos),
            );
        let Some(chest_block) = chest_block else {
            break;
        };

        state.checked_chests.lock().push(chest_block);

        bot.goto(RadiusGoal::new(chest_block.center(), 3.)).await;

        let Some(chest) = bot.open_container_at(chest_block, None).await else {
            println!("Couldn't open chest at {chest_block:?}");
            continue;
        };

        println!("Getting contents of chest at {chest_block:?}");
        for (index, slot) in chest.contents().unwrap_or_default().iter().enumerate() {
            println!("Checking slot {index}: {slot:?}");
            let ItemStack::Present(item) = slot else {
                continue;
            };
            if item.kind == azalea::registry::Item::Diamond {
                println!("clicking slot ^");
                chest.click(QuickMoveClick::Left { slot: index as u16 });
            }
        }
    }

    bot.chat("Done");

    *state.is_stealing.lock() = false;

    Ok(())
}
