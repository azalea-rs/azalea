use std::{
    fmt::Debug,
    mem,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use azalea_core::tick::GameTick;
use azalea_entity::{
    EntityUpdateSystems, PlayerAbilities, indexing::EntityIdIndex, inventory::Inventory,
};
use azalea_physics::local_player::PhysicsState;
use azalea_world::InstanceContainer;
use bevy_app::{App, AppExit, Plugin, PluginsState, SubApp, Update};
use bevy_ecs::{
    message::MessageCursor,
    prelude::*,
    schedule::{InternedScheduleLabel, LogLevel, ScheduleBuildSettings},
};
use parking_lot::RwLock;
use tokio::{sync::oneshot, time};
use tracing::{info, warn};

use crate::{
    attack,
    block_update::QueuedServerBlockUpdates,
    chunks::ChunkBatchInfo,
    connection::RawConnection,
    cookies::ServerCookies,
    interact::BlockStatePredictionHandler,
    local_player::{Hunger, InstanceHolder, PermissionLevel, TabList},
    mining,
    movement::LastSentLookDirection,
    player::retroactively_add_game_profile_component,
};

/// A bundle of components that's inserted right when we switch to the `login`
/// state and stay present on our clients until we disconnect.
///
/// For the components that are only present in the `game` state, see
/// [`JoinedClientBundle`].
#[derive(Bundle)]
pub struct LocalPlayerBundle {
    pub raw_connection: RawConnection,
    pub instance_holder: InstanceHolder,

    pub metadata: azalea_entity::metadata::PlayerMetadataBundle,
}

/// A bundle for the components that are present on a local player that is
/// currently in the `game` protocol state.
///
/// All of these components are also removed when the client disconnects.
///
/// If you want to filter for this, use [`InGameState`].
#[derive(Bundle, Default)]
pub struct JoinedClientBundle {
    // note that InstanceHolder isn't here because it's set slightly before we fully join the world
    pub physics_state: PhysicsState,
    pub inventory: Inventory,
    pub tab_list: TabList,
    pub block_state_prediction_handler: BlockStatePredictionHandler,
    pub queued_server_block_updates: QueuedServerBlockUpdates,
    pub last_sent_direction: LastSentLookDirection,
    pub abilities: PlayerAbilities,
    pub permission_level: PermissionLevel,
    pub chunk_batch_info: ChunkBatchInfo,
    pub hunger: Hunger,
    pub cookies: ServerCookies,

    pub entity_id_index: EntityIdIndex,

    pub mining: mining::MineBundle,
    pub attack: attack::AttackBundle,

    pub in_game_state: InGameState,
}

/// A marker component for local players that are currently in the
/// `game` state.
#[derive(Clone, Component, Debug, Default)]
pub struct InGameState;
/// A marker component for local players that are currently in the
/// `configuration` state.
#[derive(Clone, Component, Debug, Default)]
pub struct InConfigState;

pub struct AzaleaPlugin;
impl Plugin for AzaleaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                // add GameProfileComponent when we get an AddPlayerEvent
                retroactively_add_game_profile_component
                    .after(EntityUpdateSystems::Index)
                    .after(crate::join::handle_start_join_server_event),
            ),
        )
        .init_resource::<InstanceContainer>()
        .init_resource::<TabList>();
    }
}

/// Create the ECS world, and return a function that begins running systems.
/// This exists to allow you to make last-millisecond updates to the world
/// before any systems start running.
///
/// You can create your app with `App::new()`, but don't forget to add
/// [`DefaultPlugins`].
///
/// # Panics
///
/// This function panics if it's called outside of a Tokio `LocalSet` (or
/// `LocalRuntime`). This exists so Azalea doesn't unexpectedly run game ticks
/// in the middle of blocking user code.
#[doc(hidden)]
pub fn start_ecs_runner(
    app: &mut SubApp,
) -> (
    Arc<RwLock<World>>,
    impl FnOnce(),
    oneshot::Receiver<AppExit>,
) {
    // this block is based on Bevy's default runner:
    // https://github.com/bevyengine/bevy/blob/390877cdae7a17095a75c8f9f1b4241fe5047e83/crates/bevy_app/src/schedule_runner.rs#L77-L85
    if app.plugins_state() != PluginsState::Cleaned {
        // Wait for plugins to load
        if app.plugins_state() == PluginsState::Adding {
            info!("Waiting for plugins to load ...");
            while app.plugins_state() == PluginsState::Adding {
                thread::yield_now();
            }
        }
        // Finish adding plugins and cleanup
        app.finish();
        app.cleanup();
    }

    // all resources should have been added by now so we can take the ecs from the
    // app
    let ecs = Arc::new(RwLock::new(mem::take(app.world_mut())));

    let ecs_clone = ecs.clone();
    let outer_schedule_label = *app.update_schedule.as_ref().unwrap();

    let (appexit_tx, appexit_rx) = oneshot::channel();
    let start_running_systems = move || {
        tokio::task::spawn_local(async move {
            let appexit = run_schedule_loop(ecs_clone, outer_schedule_label).await;
            appexit_tx.send(appexit)
        });
    };

    (ecs, start_running_systems, appexit_rx)
}

/// Runs the `Update` schedule 60 times per second and the `GameTick` schedule
/// 20 times per second.
///
/// Exits when we receive an `AppExit` event.
async fn run_schedule_loop(
    ecs: Arc<RwLock<World>>,
    outer_schedule_label: InternedScheduleLabel,
) -> AppExit {
    let mut last_update: Option<Instant> = None;
    let mut last_tick: Option<Instant> = None;

    // azalea runs the Update schedule at most 60 times per second to simulate
    // framerate. unlike vanilla though, we also only handle packets during Updates
    // due to everything running in ecs systems.
    const UPDATE_DURATION_TARGET: Duration = Duration::from_micros(1_000_000 / 60);
    // minecraft runs at 20 tps
    const GAME_TICK_DURATION_TARGET: Duration = Duration::from_micros(1_000_000 / 20);

    loop {
        // sleep until the next update if necessary
        let now = Instant::now();
        if let Some(last_update) = last_update {
            let elapsed = now.duration_since(last_update);
            if elapsed < UPDATE_DURATION_TARGET {
                time::sleep(UPDATE_DURATION_TARGET - elapsed).await;
            }
        }
        last_update = Some(now);

        let mut ecs = ecs.write();

        // if last tick is None or more than 50ms ago, run the GameTick schedule
        ecs.run_schedule(outer_schedule_label);
        if last_tick
            .map(|last_tick| last_tick.elapsed() > GAME_TICK_DURATION_TARGET)
            .unwrap_or(true)
        {
            if let Some(last_tick) = &mut last_tick {
                *last_tick += GAME_TICK_DURATION_TARGET;

                // if we're more than 10 ticks behind, set last_tick to now.
                // vanilla doesn't do it in exactly the same way but it shouldn't really matter
                if (now - *last_tick) > GAME_TICK_DURATION_TARGET * 10 {
                    warn!(
                        "GameTick is more than 10 ticks behind, skipping ticks so we don't have to burst too much"
                    );
                    *last_tick = now;
                }
            } else {
                last_tick = Some(now);
            }
            ecs.run_schedule(GameTick);
        }

        ecs.clear_trackers();
        if let Some(exit) = should_exit(&mut ecs) {
            // it's possible for references to the World to stay around, so we clear the ecs
            ecs.clear_all();
            // ^ note that this also forcefully disconnects all of our bots without sending
            // a disconnect packet (which is fine because we want to disconnect immediately)

            return exit;
        }
    }
}

/// Checks whether the [`AppExit`] event was sent, and if so returns it.
///
/// This is based on Bevy's `should_exit` function: https://github.com/bevyengine/bevy/blob/b9fd7680e78c4073dfc90fcfdc0867534d92abe0/crates/bevy_app/src/app.rs#L1292
fn should_exit(ecs: &mut World) -> Option<AppExit> {
    let mut reader = MessageCursor::default();

    let events = ecs.get_resource::<Messages<AppExit>>()?;
    let mut events = reader.read(events);

    if events.len() != 0 {
        return Some(
            events
                .find(|exit| exit.is_error())
                .cloned()
                .unwrap_or(AppExit::Success),
        );
    }

    None
}

pub struct AmbiguityLoggerPlugin;
impl Plugin for AmbiguityLoggerPlugin {
    fn build(&self, app: &mut App) {
        app.edit_schedule(Update, |schedule| {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Warn,
                ..Default::default()
            });
        });
        app.edit_schedule(GameTick, |schedule| {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Warn,
                ..Default::default()
            });
        });
    }
}
