use azalea::{prelude::*, SwarmEvent};
use azalea::{Account, Client, Event, Swarm};

#[tokio::main]
async fn main() {
    let mut accounts = Vec::new();
    let mut states = Vec::new();

    for i in 0..10 {
        accounts.push(Account::offline(&format!("bot{}", i)));
        states.push(State::default());
    }

    azalea::start_swarm(azalea::SwarmOptions {
        accounts,
        address: "localhost",

        swarm_state: SwarmState::default(),
        states,

        swarm_plugins: plugins![],
        plugins: plugins![],

        handle,
        swarm_handle,

        join_delay: None,
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

async fn swarm_handle(
    swarm: Swarm<State>,
    event: SwarmEvent,
    state: SwarmState,
) -> anyhow::Result<()> {
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
