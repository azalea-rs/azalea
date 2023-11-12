//! Simulate the Minecraft world, currently only used for tests.

use std::{sync::Arc, time::Duration};

use azalea_client::{inventory::InventoryComponent, PhysicsState};
use azalea_core::{position::Vec3, resource_location::ResourceLocation};
use azalea_entity::{
    attributes::AttributeInstance, metadata::Sprinting, Attributes, EntityDimensions, Physics,
    Position,
};
use azalea_world::{ChunkStorage, Instance, InstanceContainer, InstanceName, MinecraftEntityId};
use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::*;
use bevy_time::{Fixed, Time};
use parking_lot::RwLock;

#[derive(Bundle, Clone)]
pub struct SimulatedPlayerBundle {
    pub position: Position,
    pub physics: Physics,
    pub physics_state: PhysicsState,
    pub attributes: Attributes,
    pub inventory: InventoryComponent,
}

impl SimulatedPlayerBundle {
    pub fn new(position: Vec3) -> Self {
        let dimensions = EntityDimensions {
            width: 0.6,
            height: 1.8,
        };

        SimulatedPlayerBundle {
            position: Position::new(position),
            physics: Physics::new(dimensions, &position),
            physics_state: PhysicsState::default(),
            attributes: Attributes {
                speed: AttributeInstance::new(0.1),
                attack_speed: AttributeInstance::new(4.0),
            },
            inventory: InventoryComponent::default(),
        }
    }
}

/// Simulate the Minecraft world to see if certain movements would be possible.
pub struct Simulation {
    pub app: App,
    pub entity: Entity,
    _instance: Arc<RwLock<Instance>>,
}

impl Simulation {
    pub fn new(chunks: ChunkStorage, player: SimulatedPlayerBundle) -> Self {
        let instance_name = ResourceLocation::new("azalea:simulation");

        let instance = Arc::new(RwLock::new(Instance {
            chunks,
            ..Default::default()
        }));

        let mut app = App::new();
        // we don't use all the default azalea plugins because we don't need all of them
        app.add_plugins((
            azalea_physics::PhysicsPlugin,
            azalea_entity::EntityPlugin,
            azalea_client::movement::PlayerMovePlugin,
            super::PathfinderPlugin,
            crate::BotPlugin,
            azalea_client::task_pool::TaskPoolPlugin::default(),
        ))
        // make sure it doesn't do fixed ticks without us telling it to
        .insert_resource(Time::<Fixed>::from_duration(Duration::MAX))
        .insert_resource(InstanceContainer {
            instances: [(instance_name.clone(), Arc::downgrade(&instance.clone()))]
                .iter()
                .cloned()
                .collect(),
        })
        .add_event::<azalea_client::SendPacketEvent>();

        app.edit_schedule(bevy_app::Main, |schedule| {
            schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
        });

        let entity = app
            .world
            .spawn((
                MinecraftEntityId(0),
                InstanceName(instance_name),
                azalea_entity::LocalEntity,
                azalea_entity::Jumping::default(),
                azalea_entity::LookDirection::default(),
                Sprinting(true),
                azalea_entity::metadata::Player,
                azalea_entity::EyeHeight::new(player.physics.dimensions.height * 0.85),
                player,
            ))
            .id();

        Self {
            app,
            entity,
            _instance: instance,
        }
    }
    pub fn tick(&mut self) {
        self.app.world.run_schedule(FixedUpdate);
        self.app.update();
    }
    pub fn position(&self) -> Vec3 {
        **self.app.world.get::<Position>(self.entity).unwrap()
    }
}
