#![feature(trait_alias)]

//! Re-export important parts of [`bevy_ecs`] and [`bevy_app`] and make them
//! more compatible with Azalea.
//!
//! This is completely compatible with `bevy_ecs`, so it won't cause issues if
//! you use plugins meant for Bevy.
//!
//! Changes:
//! - Change the macros to use azalea/azalea_ecs instead of bevy/bevy_ecs
//! - Rename `world::World` to [`ecs::Ecs`]
//! - Re-export `bevy_app` in the [`app`] module.
//!
//! [`bevy_ecs`]: https://docs.rs/bevy_ecs
//! [`bevy_app`]: https://docs.rs/bevy_app

pub mod ecs {
    pub use bevy_ecs::world::World as Ecs;
    pub use bevy_ecs::world::{EntityMut, EntityRef, Mut};
}
pub mod component {
    pub use azalea_ecs_macros::Component;
    pub use bevy_ecs::component::{
        ComponentDescriptor, ComponentId, ComponentIdFor, ComponentInfo, ComponentStorage,
        ComponentTicks, Components, StorageType, TableStorage, Tick, TickCells,
    };

    // we do this because re-exporting Component would re-export the macro as well,
    // which is bad (since we have our own Component macro)
    // instead, we have to do this so Component is a trait alias and the original
    // impl-able trait is still available as _BevyComponent
    pub trait Component = bevy_ecs::component::Component;
    pub use bevy_ecs::component::Component as _BevyComponent;
}
pub mod bundle {
    pub use azalea_ecs_macros::Bundle;
    pub use bevy_ecs::bundle::{BundleId, BundleInfo, Bundles};
    pub trait Bundle = bevy_ecs::bundle::Bundle;
    pub use bevy_ecs::bundle::Bundle as _BevyBundle;
}
pub mod system {
    pub use azalea_ecs_macros::Resource;
    pub use bevy_ecs::system::{
        Command, Commands, EntityCommands, Query, Res, ResMut, SystemState,
    };
    pub trait Resource = bevy_ecs::system::Resource;
    pub use bevy_ecs::system::Resource as _BevyResource;
}
pub mod schedule {
    pub use azalea_ecs_macros::SystemSet;
    pub use bevy_ecs::schedule::{
        BaseSystemSet, Condition, FreeSystemSet, IntoSystemConfig, IntoSystemConfigs,
        IntoSystemSet, IntoSystemSetConfig, IntoSystemSetConfigs, ScheduleLabel, States,
    };
    pub use bevy_ecs::schedule::{
        Dag, MainThreadExecutor, MultiThreadedExecutor, NextState, OnEnter, OnExit, OnUpdate,
        Schedule, ScheduleBuildSettings, ScheduleGraph, Schedules, SimpleExecutor,
        SingleThreadedExecutor, State, SystemConfig, SystemConfigs, SystemSchedule,
        SystemSetConfig, SystemSetConfigs, SystemTypeSet,
    };
    pub trait SystemSet = bevy_ecs::schedule::SystemSet;
    pub use bevy_ecs::schedule::SystemSet as _BevySystemSet;
}
pub use bevy_app as app;
pub use bevy_ecs::{entity, event, ptr, query, storage};
