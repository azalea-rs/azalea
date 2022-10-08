use azalea::{pathfinder, Account, Accounts, Client, Event};
use parking_lot::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let accounts = Accounts::new();

    for i in 0..10 {
        accounts.add(Account::offline(format!("bot{}", i)));
    }

    azalea::start_group(azalea::GroupOptions {
        accounts,
        address: "localhost",

        group_state: Arc::new(Mutex::new(State::default())),
        state: State::default(),

        group_plugins: vec![Arc::new(pathfinder::Plugin::default())],
        plugins: vec![],

        handle: Box::new(handle),
        group_handle: Box::new(handle),
    })
    .await
    .unwrap();
}

#[derive(Default)]
struct State {}

#[derive(Default)]
struct GroupState {}

async fn handle(bot: Client, event: Arc<Event>, state: Arc<Mutex<State>>) -> anyhow::Result<()> {
    match event {
        _ => {}
    }

    Ok(())
}

async fn group_handle(
    bots: Swarm,
    event: Arc<Event>,
    state: Arc<Mutex<GroupState>>,
) -> anyhow::Result<()> {
    match *event {
        Event::Login => {
            bots.goto(azalea::BlockPos::new(0, 70, 0)).await;
            // or bots.goto_goal(pathfinder::Goals::Goto(azalea::BlockPos(0, 70, 0))).await;

            // destroy the blocks in this area and then leave

            bots.fill(
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
