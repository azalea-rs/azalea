use azalea::prelude::*;
use azalea::{Account, Accounts, Client, Event, Swarm};
use parking_lot::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let mut accounts = Vec::new();
    let mut states = Vec::new();

    for i in 0..10 {
        accounts.push(Account::offline(&format!("bot{}", i)));
        states.push(Arc::new(Mutex::new(State::default())));
    }

    azalea::start_swarm(azalea::SwarmOptions {
        accounts,
        address: "localhost",

        swarm_state: State::default(),
        states,

        swarm_plugins: plugins![azalea_pathfinder::Plugin::default()],
        plugins: plugins![],

        handle,
        swarm_handle,
    })
    .await
    .unwrap();
}

#[derive(Default, Clone)]
struct State {}

#[derive(Default, Clone)]
struct SwarmState {}

async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    Ok(())
}

async fn swarm_handle(swarm: Swarm, event: Event, state: SwarmState) -> anyhow::Result<()> {
    match event {
        Event::Login => {
            swarm.goto(azalea::BlockPos::new(0, 70, 0)).await;
            // or bots.goto_goal(pathfinder::Goals::Goto(azalea::BlockPos(0, 70, 0))).await;

            // destroy the blocks in this area and then leave

            swarm
                .fill(
                    azalea::Selection::Range(
                        azalea::BlockPos::new(0, 0, 0),
                        azalea::BlockPos::new(16, 255, 16),
                    ),
                    azalea::block::Air,
                )
                .await;
        }
        _ => {}
    }

    Ok(())
}
