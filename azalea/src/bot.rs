use std::f64::consts::PI;

use azalea_client::{
    mining::Mining,
    tick_broadcast::{TickBroadcast, UpdateBroadcast},
};
use azalea_core::{
    position::{BlockPos, Vec3},
    tick::GameTick,
};
use azalea_entity::{
    Jumping, LocalEntity, LookDirection, Position, clamp_look_direction,
    dimensions::EntityDimensions, metadata::Player, update_dimensions,
};
use azalea_physics::PhysicsSet;
use bevy_app::Update;
use bevy_ecs::prelude::*;
use futures_lite::Future;
use tracing::trace;

use crate::{
    accept_resource_packs::AcceptResourcePacksPlugin,
    app::{App, Plugin, PluginGroup, PluginGroupBuilder},
    auto_respawn::AutoRespawnPlugin,
    container::ContainerPlugin,
    ecs::{
        component::Component,
        entity::Entity,
        query::{With, Without},
        system::{Commands, Query},
    },
    pathfinder::PathfinderPlugin,
};

#[derive(Clone, Default)]
pub struct BotPlugin;
impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<LookAtEvent>()
            .add_message::<JumpEvent>()
            .add_systems(
                Update,
                (
                    insert_bot,
                    look_at_listener
                        .before(clamp_look_direction)
                        .after(update_dimensions),
                    jump_listener,
                ),
            )
            .add_systems(
                GameTick,
                stop_jumping
                    .after(PhysicsSet)
                    .after(azalea_client::movement::send_player_input_packet),
            );
    }
}

/// A component that clients with [`BotPlugin`] will have. If you just want to
/// check if an entity is one of our bots, you should use [`LocalEntity`].
#[derive(Default, Component)]
pub struct Bot {
    jumping_once: bool,
}

/// Insert the [`Bot`] component for any local players that don't have it.
#[allow(clippy::type_complexity)]
fn insert_bot(
    mut commands: Commands,
    mut query: Query<Entity, (Without<Bot>, With<LocalEntity>, With<Player>)>,
) {
    for entity in &mut query {
        commands.entity(entity).insert(Bot::default());
    }
}

fn stop_jumping(mut query: Query<(&mut Jumping, &mut Bot)>) {
    for (mut jumping, mut bot) in &mut query {
        if bot.jumping_once && **jumping {
            bot.jumping_once = false;
            **jumping = false;
        }
    }
}

pub trait BotClientExt {
    /// Queue a jump for the next tick.
    fn jump(&self);
    /// Turn the bot's head to look at the coordinate in the world.
    fn look_at(&self, pos: Vec3);
    /// Get a receiver that will receive a message every tick.
    fn get_tick_broadcaster(&self) -> tokio::sync::broadcast::Receiver<()>;
    /// Get a receiver that will receive a message every ECS Update.
    fn get_update_broadcaster(&self) -> tokio::sync::broadcast::Receiver<()>;
    /// Wait for the specified number of game ticks.
    fn wait_ticks(&self, n: usize) -> impl Future<Output = ()> + Send;
    /// Wait for the specified number of ECS `Update`s.
    fn wait_updates(&self, n: usize) -> impl Future<Output = ()> + Send;
    /// Mine a block. This won't turn the bot's head towards the block, so if
    /// that's necessary you'll have to do that yourself with [`look_at`].
    ///
    /// [`look_at`]: crate::prelude::BotClientExt::look_at
    fn mine(&self, position: BlockPos) -> impl Future<Output = ()> + Send;
}

impl BotClientExt for azalea_client::Client {
    fn jump(&self) {
        let mut ecs = self.ecs.lock();
        ecs.write_message(JumpEvent {
            entity: self.entity,
        });
    }

    fn look_at(&self, position: Vec3) {
        let mut ecs = self.ecs.lock();
        ecs.write_message(LookAtEvent {
            entity: self.entity,
            position,
        });
    }

    /// Returns a Receiver that receives a message every game tick.
    ///
    /// This is useful if you want to efficiently loop until a certain condition
    /// is met.
    ///
    /// ```
    /// # use azalea::prelude::*;
    /// # use azalea::container::WaitingForInventoryOpen;
    /// # async fn example(bot: &mut azalea::Client) {
    /// let mut ticks = bot.get_tick_broadcaster();
    /// while ticks.recv().await.is_ok() {
    ///     let ecs = bot.ecs.lock();
    ///     if ecs.get::<WaitingForInventoryOpen>(bot.entity).is_none() {
    ///         break;
    ///     }
    /// }
    /// # }
    /// ```
    fn get_tick_broadcaster(&self) -> tokio::sync::broadcast::Receiver<()> {
        let ecs = self.ecs.lock();
        let tick_broadcast = ecs.resource::<TickBroadcast>();
        tick_broadcast.subscribe()
    }

    /// Returns a Receiver that receives a message every ECS Update.
    ///
    /// ECS Updates happen at least at the frequency of game ticks, usually
    /// faster.
    ///
    /// This is useful if you're sending an ECS event and want to make sure it's
    /// been handled before continuing.
    fn get_update_broadcaster(&self) -> tokio::sync::broadcast::Receiver<()> {
        let ecs = self.ecs.lock();
        let update_broadcast = ecs.resource::<UpdateBroadcast>();
        update_broadcast.subscribe()
    }

    /// Wait for the specified number of ticks using
    /// [`Self::get_tick_broadcaster`].
    ///
    /// If you're going to run this in a loop, you may want to use that function
    /// instead and use the `Receiver` from it to avoid accidentally skipping
    /// ticks and having to wait longer.
    async fn wait_ticks(&self, n: usize) {
        let mut receiver = self.get_tick_broadcaster();
        for _ in 0..n {
            let _ = receiver.recv().await;
        }
    }
    /// Waits for the specified number of ECS `Update`s using
    /// [`Self::get_update_broadcaster`].
    ///
    /// These are basically equivalent to frames because even though we have no
    /// rendering, some game mechanics depend on frames.
    ///
    /// If you're going to run this in a loop, you may want to use that function
    /// instead and use the `Receiver` from it to avoid accidentally skipping
    /// ticks and having to wait longer.
    async fn wait_updates(&self, n: usize) {
        let mut receiver = self.get_update_broadcaster();
        for _ in 0..n {
            let _ = receiver.recv().await;
        }
    }

    async fn mine(&self, position: BlockPos) {
        self.start_mining(position);

        let mut receiver = self.get_tick_broadcaster();
        while receiver.recv().await.is_ok() {
            let ecs = self.ecs.lock();
            if ecs.get::<Mining>(self.entity).is_none() {
                break;
            }
        }
    }
}

/// Event to jump once.
#[derive(Message)]
pub struct JumpEvent {
    pub entity: Entity,
}

pub fn jump_listener(
    mut query: Query<(&mut Jumping, &mut Bot)>,
    mut events: MessageReader<JumpEvent>,
) {
    for event in events.read() {
        if let Ok((mut jumping, mut bot)) = query.get_mut(event.entity) {
            **jumping = true;
            bot.jumping_once = true;
        }
    }
}

/// Make an entity look towards a certain position in the world.
#[derive(Message)]
pub struct LookAtEvent {
    pub entity: Entity,
    /// The position we want the entity to be looking at.
    pub position: Vec3,
}
fn look_at_listener(
    mut events: MessageReader<LookAtEvent>,
    mut query: Query<(&Position, &EntityDimensions, &mut LookDirection)>,
) {
    for event in events.read() {
        if let Ok((position, dimensions, mut look_direction)) = query.get_mut(event.entity) {
            let new_look_direction =
                direction_looking_at(position.up(dimensions.eye_height.into()), event.position);

            trace!("look at {} (currently at {})", event.position, **position);
            look_direction.update(new_look_direction);
        }
    }
}

/// Return the look direction that would make a client at `current` be
/// looking at `target`.
pub fn direction_looking_at(current: Vec3, target: Vec3) -> LookDirection {
    // borrowed from mineflayer's Bot.lookAt because i didn't want to do math
    let delta = target - current;
    let y_rot = (PI - f64::atan2(-delta.x, -delta.z)) * (180.0 / PI);
    let ground_distance = f64::sqrt(delta.x * delta.x + delta.z * delta.z);
    let x_rot = f64::atan2(delta.y, ground_distance) * -(180.0 / PI);

    LookDirection::new(y_rot as f32, x_rot as f32)
}

/// A [`PluginGroup`] for the plugins that add extra bot functionality to the
/// client.
pub struct DefaultBotPlugins;

impl PluginGroup for DefaultBotPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BotPlugin)
            .add(PathfinderPlugin)
            .add(ContainerPlugin)
            .add(AutoRespawnPlugin)
            .add(AcceptResourcePacksPlugin)
    }
}
