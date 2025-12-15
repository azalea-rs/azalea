use azalea::{prelude::*, swarm::prelude::*};

#[tokio::main]
async fn main() -> AppExit {
    let mut accounts = Vec::new();
    let mut states = Vec::new();

    for i in 0..10 {
        accounts.push(Account::offline(&format!("bot{i}")));
        states.push(State::default());
    }

    SwarmBuilder::new()
        .add_accounts(accounts.clone())
        .set_handler(handle)
        .set_swarm_handler(swarm_handle)
        .start("localhost")
        .await
}

#[derive(Clone, Component, Default)]
struct State {}

#[derive(Clone, Default, Resource)]
struct SwarmState {}

async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    Ok(())
}

async fn swarm_handle(swarm: Swarm, event: SwarmEvent, state: SwarmState) -> anyhow::Result<()> {
    match &event {
        SwarmEvent::Login => {
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
