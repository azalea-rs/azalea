#![doc = include_str!("../README.md")]
#![feature(type_changing_struct_update)]

#[cfg(doc)]
pub mod _docs;
pub mod accept_resource_packs;
pub mod auto_reconnect;
pub mod auto_respawn;
pub mod auto_tool;
pub mod bot;
mod builder;
mod client_impl;
pub mod container;
mod entity_ref;
pub mod events;
mod join_opts;
pub mod nearest_entity;
pub mod pathfinder;
pub mod prelude;
pub mod swarm;
pub mod tick_broadcast;

use std::ops::Deref;

pub use azalea_auth as auth;
pub use azalea_block as block;
#[doc(hidden)]
#[deprecated = "moved to `azalea::block`"]
pub mod blocks {
    pub type BlockStates = azalea_block::BlockStates;
    pub type BlockState = azalea_block::BlockState;
    pub trait BlockTrait: azalea_block::BlockTrait {}
    // azalea_block has more items but rust doesn't mark them deprecated if we
    // `use azalea_block::*`, so hopefully the three types above are enough for
    // most users :(
}

pub use azalea_brigadier as brigadier;
pub use azalea_buf as buf;
pub use azalea_chat::FormattedText;
pub use azalea_client::*;
pub use azalea_core as core;
// these are re-exported on this level because they're very common
pub use azalea_core::position::{BlockPos, Vec3};
pub use azalea_entity as entity;
pub use azalea_physics as physics;
pub use azalea_protocol as protocol;
pub use azalea_registry as registry;
#[doc(hidden)]
#[deprecated(note = "renamed to `Identifier`.")]
pub type ResourceLocation = azalea_registry::identifier::Identifier;

// TODO: replace this mod with the commented line below
// pub use azalea_chat as chat;
pub mod chat {
    pub use azalea_chat::*;
    #[deprecated = "moved to `azalea::client_chat`."]
    pub type ChatPacket = azalea_client::client_chat::ChatPacket;
}

pub use azalea_registry::identifier::Identifier;
pub use azalea_world as world;
pub use bevy_app as app;
pub use bevy_ecs as ecs;
use bevy_ecs::{component::Component, resource::Resource};
pub use builder::ClientBuilder;
use futures::future::BoxFuture;
pub use join_opts::JoinOpts;

pub use crate::{
    client_impl::{Client, StartClientOpts, error},
    entity_ref::EntityRef,
    events::Event,
};

// for convenience, adds the alias `azalea::Result` instead of
// `azalea::error::AzaleaResult`. the user should probably be using anyhow/eyre,
// but in some cases they may prefer to have the errors more strictly defined.
pub type Result<T> = error::AzaleaResult<T>;

pub type BoxHandleFn<S, R> = Box<dyn Fn(Client, Event, S) -> BoxFuture<'static, R> + Send>;
pub type HandleFn<S, Fut> = fn(Client, Event, S) -> Fut;

/// A marker that can be used in place of a State in [`ClientBuilder`] or
/// [`SwarmBuilder`].
///
/// You probably don't need to use this manually since the compiler will infer
/// it for you.
///
/// [`SwarmBuilder`]: swarm::SwarmBuilder
#[derive(Clone, Component, Default)]
pub struct NoState;

/// A reference to a `tokio::runtime::Handle`, allowing you to spawn Tokio tasks
/// inside of ECS systems.
///
/// There are times in which you may want to use something like `tokio::spawn`
/// inside of an ECS system, but you don't want to bother with message passing
/// or Bevy's `AsyncComputeTaskPool`. Bevy doesn't run systems inside of a Tokio
/// runtime, which results in an error if you try to use `tokio::spawn` or
/// `tokio::task::spawn_local`. However, if you have a reference to a `Handle`,
/// then Tokio will let you use it to spawn new tasks. This `Resource` exists
/// for that -- it simply gives you a handle to a Tokio runtime to do whatever
/// you want with.
///
/// ```
/// fn example(rt: Res<azalea::TokioRuntimeHandle>) {
///     rt.spawn(async {
///         // ...
///     });
/// }
/// ```
#[derive(Resource)]
pub struct TokioRuntimeHandle(pub tokio::runtime::Handle);
impl Deref for TokioRuntimeHandle {
    type Target = tokio::runtime::Handle;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
