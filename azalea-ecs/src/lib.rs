#![feature(trait_alias)]

//! Re-export the necessary parts of `bevy_ecs` and `bevy_app`.
//!
//! This is completely compatible with `bevy_ecs`, so it won't cause issues if
//! you use plugins meant for Bevy.

use std::{
    task::{Context, Poll},
    time::Duration,
};

pub mod component {
    pub use azalea_ecs_macros::Component;
    pub use bevy_ecs::component::{ComponentStorage, TableStorage};

    // we do this because re-exporting Component would re-export the macro as well,
    // which is bad (since we have our own Component macro)
    // instead, we have to do this so Component is a trait alias and the original
    // impl-able trait is still available as BevyComponent
    pub trait Component = bevy_ecs::component::Component;
    pub use bevy_ecs::component::Component as BevyComponent;
}
pub mod ecs {
    pub use bevy_ecs::world::World as Ecs;
    pub use bevy_ecs::world::{EntityMut, EntityRef, Mut};
}
pub use bevy_app::*;
use bevy_ecs::schedule::*;
pub use bevy_ecs::system;
pub use bevy_ecs::{event, query, schedule};
use ecs::Ecs;
use futures::task::noop_waker_ref;
use tokio::time::Interval;

pub struct TickPlugin;
impl Plugin for TickPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_before(
            CoreStage::Update,
            TickLabel,
            TickStage::from_stage(SystemStage::parallel()),
        );
    }
}

#[derive(StageLabel)]
struct TickLabel;

/// A [`Stage`] that runs every 50 milliseconds.
pub struct TickStage {
    pub interval: Interval,
    stage: Box<dyn Stage>,
}

impl TickStage {
    pub fn from_stage(stage: impl Stage) -> Self {
        let mut game_tick_interval = tokio::time::interval(Duration::from_millis(50));
        // TODO: Minecraft bursts up to 10 ticks and then skips, we should too
        game_tick_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Burst);

        TickStage {
            interval: game_tick_interval,
            stage: Box::new(stage),
        }
    }
}
impl Stage for TickStage {
    fn run(&mut self, ecs: &mut Ecs) {
        // keep calling run until it's caught up
        while let Poll::Ready(r) = self
            .interval
            .poll_tick(&mut Context::from_waker(&noop_waker_ref()))
        {
            self.stage.run(ecs);
        }
    }
}

pub trait AppTickExt {
    fn add_tick_system_set(&mut self, system_set: SystemSet) -> &mut App;
}

impl AppTickExt for App {
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
}
