#![feature(trait_alias)]

//! Re-export important parts of [`bevy_ecs`] and [`bevy_app`] and make them
//! more compatible with Azalea.
//!
//! This is completely compatible with `bevy_ecs`, so it won't cause issues if
//! you use plugins meant for Bevy.
//!
//! Changes:
//! - Add [`TickPlugin`], [`TickStage`] and [`AppTickExt`] (which adds
//!   `app.add_tick_system` and `app.add_tick_system_set`).
//! - Change the macros to use azalea/azalea_ecs instead of bevy/bevy_ecs
//! - Rename `world::World` to [`ecs::Ecs`]
//! - Re-export `bevy_app` in the [`app`] module.
//!
//! [`bevy_ecs`]: https://docs.rs/bevy_ecs
//! [`bevy_app`]: https://docs.rs/bevy_app

use std::time::{Duration, Instant};

pub mod ecs {
    pub use bevy_ecs::world::World as Ecs;
    pub use bevy_ecs::world::{EntityMut, EntityRef, Mut};
}
pub mod component {
    pub use azalea_ecs_macros::Component;
    pub use bevy_ecs::component::{ComponentId, ComponentStorage, Components, TableStorage};

    // we do this because re-exporting Component would re-export the macro as well,
    // which is bad (since we have our own Component macro)
    // instead, we have to do this so Component is a trait alias and the original
    // impl-able trait is still available as BevyComponent
    pub trait Component = bevy_ecs::component::Component;
    pub use bevy_ecs::component::Component as BevyComponent;
}
pub mod bundle {
    pub use azalea_ecs_macros::Bundle;
    pub trait Bundle = bevy_ecs::bundle::Bundle;
    pub use bevy_ecs::bundle::Bundle as BevyBundle;
}
pub mod system {
    pub use azalea_ecs_macros::Resource;
    pub use bevy_ecs::system::{
        Command, Commands, EntityCommands, Query, Res, ResMut, SystemState,
    };
    pub trait Resource = bevy_ecs::system::Resource;
    pub use bevy_ecs::system::Resource as BevyResource;
}
pub use bevy_app as app;
pub use bevy_ecs::{entity, event, ptr, query, schedule, storage};

use app::{App, CoreStage, Plugin};
use bevy_ecs::schedule::*;
use ecs::Ecs;

pub struct TickPlugin {
    /// How often a tick should happen. 50 milliseconds by default. Set to 0 to
    /// tick every update.
    pub tick_interval: Duration,
}
impl Plugin for TickPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_before(
            CoreStage::Update,
            TickLabel,
            TickStage {
                interval: self.tick_interval,
                next_tick: Instant::now(),
                stage: Box::new(SystemStage::parallel()),
            },
        );
    }
}
impl Default for TickPlugin {
    fn default() -> Self {
        Self {
            tick_interval: Duration::from_millis(50),
        }
    }
}

#[derive(StageLabel)]
struct TickLabel;

/// A [`Stage`] that runs every 50 milliseconds.
pub struct TickStage {
    pub interval: Duration,
    pub next_tick: Instant,
    stage: Box<dyn Stage>,
}

impl Stage for TickStage {
    fn run(&mut self, ecs: &mut Ecs) {
        // if the interval is 0, that means it runs every tick
        if self.interval.is_zero() {
            self.stage.run(ecs);
            return;
        }
        // keep calling run until it's caught up
        // TODO: Minecraft bursts up to 10 ticks and then skips, we should too (but
        // check the source so we do it right)
        while Instant::now() > self.next_tick {
            self.next_tick += self.interval;
            self.stage.run(ecs);
        }
    }
}

pub trait AppTickExt {
    fn add_tick_system_set(&mut self, system_set: SystemSet) -> &mut App;
    fn add_tick_system<Params>(&mut self, system: impl IntoSystemDescriptor<Params>) -> &mut App;
}

impl AppTickExt for App {
    /// Adds a set of ECS systems that will run every 50 milliseconds.
    ///
    /// Note that you should NOT have `EventReader`s in tick systems, as this
    /// will make them sometimes be missed.
    fn add_tick_system_set(&mut self, system_set: SystemSet) -> &mut App {
        let tick_stage = self
            .schedule
            .get_stage_mut::<TickStage>(TickLabel)
            .expect("Tick Stage not found");
        let stage = tick_stage
            .stage
            .downcast_mut::<SystemStage>()
            .expect("Fixed Timestep sub-stage is not a SystemStage");
        stage.add_system_set(system_set);
        self
    }

    /// Adds a new ECS system that will run every 50 milliseconds.
    ///
    /// Note that you should NOT have `EventReader`s in tick systems, as this
    /// will make them sometimes be missed.
    fn add_tick_system<Params>(&mut self, system: impl IntoSystemDescriptor<Params>) -> &mut App {
        let tick_stage = self
            .schedule
            .get_stage_mut::<TickStage>(TickLabel)
            .expect("Tick Stage not found");
        let stage = tick_stage
            .stage
            .downcast_mut::<SystemStage>()
            .expect("Fixed Timestep sub-stage is not a SystemStage");
        stage.add_system(system);
        self
    }
}
