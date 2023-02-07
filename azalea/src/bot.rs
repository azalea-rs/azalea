use azalea_core::Vec3;
use azalea_ecs::{
    app::{App, Plugin, PluginGroup, PluginGroupBuilder},
    component::Component,
    entity::Entity,
    event::EventReader,
    query::{With, Without},
    schedule::IntoSystemDescriptor,
    system::{Commands, Query},
    AppTickExt,
};
use azalea_world::entity::{metadata::Player, set_rotation, Jumping, Local, Physics, Position};
use std::f64::consts::PI;

use crate::pathfinder::PathfinderPlugin;

#[derive(Clone, Default)]
pub struct BotPlugin;
impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LookAtEvent>()
            .add_event::<JumpEvent>()
            .add_system(insert_bot.before("deduplicate_entities"))
            .add_system(look_at_listener)
            .add_system(jump_listener.label("jump_listener").before("ai_step"))
            .add_tick_system(stop_jumping.after("ai_step"));
    }
}

/// Component for all bots.
#[derive(Default, Component)]
pub struct Bot {
    jumping_once: bool,
}

/// Insert the [`Bot`] component for any local players that don't have it.
#[allow(clippy::type_complexity)]
fn insert_bot(
    mut commands: Commands,
    mut query: Query<Entity, (Without<Bot>, With<Local>, With<Player>)>,
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
    fn jump(&mut self);
    fn look_at(&mut self, pos: Vec3);
}

impl BotClientExt for azalea_client::Client {
    /// Queue a jump for the next tick.
    fn jump(&mut self) {
        let mut ecs = self.ecs.lock();
        ecs.send_event(JumpEvent(self.entity));
    }

    /// Turn the bot's head to look at the coordinate in the world.
    fn look_at(&mut self, position: Vec3) {
        let mut ecs = self.ecs.lock();
        ecs.send_event(LookAtEvent {
            entity: self.entity,
            position,
        });
    }
}

/// Event to jump once.
pub struct JumpEvent(pub Entity);

fn jump_listener(mut query: Query<(&mut Jumping, &mut Bot)>, mut events: EventReader<JumpEvent>) {
    for event in events.iter() {
        if let Ok((mut jumping, mut bot)) = query.get_mut(event.0) {
            **jumping = true;
            bot.jumping_once = true;
        }
    }
}

/// Make an entity look towards a certain position in the world.
pub struct LookAtEvent {
    pub entity: Entity,
    /// The position we want the entity to be looking at.
    pub position: Vec3,
}
fn look_at_listener(
    mut events: EventReader<LookAtEvent>,
    mut query: Query<(&Position, &mut Physics)>,
) {
    for event in events.iter() {
        if let Ok((position, mut physics)) = query.get_mut(event.entity) {
            let (y_rot, x_rot) = direction_looking_at(position, &event.position);
            set_rotation(&mut physics, y_rot, x_rot);
        }
    }
}

/// Return the (`y_rot`, `x_rot`) that would make a client at `current` be
/// looking at `target`.
fn direction_looking_at(current: &Vec3, target: &Vec3) -> (f32, f32) {
    // borrowed from mineflayer's Bot.lookAt because i didn't want to do math
    let delta = target - current;
    let y_rot = (PI - f64::atan2(-delta.x, -delta.z)) * (180.0 / PI);
    let ground_distance = f64::sqrt(delta.x * delta.x + delta.z * delta.z);
    let x_rot = f64::atan2(delta.y, ground_distance) * -(180.0 / PI);
    (y_rot as f32, x_rot as f32)
}

/// A [`PluginGroup`] for the plugins that add extra bot functionality to the
/// client.
pub struct DefaultBotPlugins;

impl PluginGroup for DefaultBotPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BotPlugin)
            .add(PathfinderPlugin)
    }
}
