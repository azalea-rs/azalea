#![doc = include_str!("../README.md")]
#![feature(type_changing_struct_update)]

pub mod accept_resource_packs;
pub mod auto_respawn;
pub mod auto_tool;
pub mod bot;
mod builder;
pub mod container;
pub mod nearest_entity;
pub mod pathfinder;
pub mod prelude;
pub mod swarm;

use std::net::SocketAddr;

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
use azalea_protocol::{address::ServerAddr, connect::Proxy};
pub use azalea_registry as registry;
#[doc(hidden)]
#[deprecated(note = "renamed to `Identifier`.")]
pub type ResourceLocation = azalea_registry::identifier::Identifier;
pub use azalea_registry::identifier::Identifier;
pub use azalea_world as world;
pub use bevy_app as app;
pub use bevy_ecs as ecs;
use bevy_ecs::component::Component;
pub use builder::ClientBuilder;
use futures::future::BoxFuture;

pub type BoxHandleFn<S, R> =
    Box<dyn Fn(Client, azalea_client::Event, S) -> BoxFuture<'static, R> + Send>;
pub type HandleFn<S, Fut> = fn(Client, azalea_client::Event, S) -> Fut;

/// A marker that can be used in place of a State in [`ClientBuilder`] or
/// [`SwarmBuilder`].
///
/// You probably don't need to use this manually since the compiler will infer
/// it for you.
///
/// [`SwarmBuilder`]: swarm::SwarmBuilder
#[derive(Clone, Component, Default)]
pub struct NoState;

/// Optional settings when adding an account to a swarm or client.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct JoinOpts {
    /// The SOCKS5 proxy that this bot will use for connecting to the Minecraft
    /// server.
    pub server_proxy: Option<Proxy>,
    /// The SOCKS5 proxy that will be used when authenticating the bot's join
    /// with Mojang.
    ///
    /// This should typically be either the same as [`Self::server_proxy`] or
    /// `None`.
    ///
    /// This is useful to set if a server has `prevent-proxy-connections`
    /// enabled.
    pub sessionserver_proxy: Option<Proxy>,
    /// Override the server address that this specific bot will send in the
    /// handshake packet.
    #[doc(alias = "custom_address")]
    pub custom_server_addr: Option<ServerAddr>,
    /// Override the IP and port that this specific bot will use to connect
    /// to the server.
    #[doc(alias = "custom_resolved_address")]
    pub custom_socket_addr: Option<SocketAddr>,
}

impl JoinOpts {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, other: &Self) {
        if let Some(proxy) = other.server_proxy.clone() {
            self.server_proxy = Some(proxy);
        }
        if let Some(proxy) = other.sessionserver_proxy.clone() {
            self.sessionserver_proxy = Some(proxy);
        }
        if let Some(custom_server_addr) = other.custom_server_addr.clone() {
            self.custom_server_addr = Some(custom_server_addr);
        }
        if let Some(custom_socket_addr) = other.custom_socket_addr {
            self.custom_socket_addr = Some(custom_socket_addr);
        }
    }

    /// Configure the SOCKS5 proxy used for connecting to the server and for
    /// authenticating with Mojang.
    ///
    /// To configure these separately, for example to only use the proxy for the
    /// Minecraft server and not for authentication, you may use
    /// [`Self::server_proxy`] and [`Self::sessionserver_proxy`] individually.
    #[must_use]
    pub fn proxy(self, proxy: Proxy) -> Self {
        self.server_proxy(proxy.clone()).sessionserver_proxy(proxy)
    }
    /// Configure the SOCKS5 proxy that will be used for connecting to the
    /// Minecraft server.
    ///
    /// To avoid errors on servers with the "prevent-proxy-connections" option
    /// set, you should usually use [`Self::proxy`] instead.
    ///
    /// Also see [`Self::sessionserver_proxy`].
    #[must_use]
    pub fn server_proxy(mut self, proxy: Proxy) -> Self {
        self.server_proxy = Some(proxy);
        self
    }
    /// Configure the SOCKS5 proxy that this bot will use for authenticating the
    /// server join with Mojang's API.
    ///
    /// Also see [`Self::proxy`] and [`Self::server_proxy`].
    #[must_use]
    pub fn sessionserver_proxy(mut self, proxy: Proxy) -> Self {
        self.sessionserver_proxy = Some(proxy);
        self
    }

    /// Set the custom address that this bot will send in the handshake packet.
    #[must_use]
    pub fn custom_server_addr(mut self, server_addr: ServerAddr) -> Self {
        self.custom_server_addr = Some(server_addr);
        self
    }
    /// Set the custom resolved address that this bot will use to connect to the
    /// server.
    #[must_use]
    pub fn custom_socket_addr(mut self, socket_addr: SocketAddr) -> Self {
        self.custom_socket_addr = Some(socket_addr);
        self
    }

    #[doc(hidden)]
    #[deprecated = "renamed to `custom_server_addr`."]
    pub fn custom_address(self, server_addr: ServerAddr) -> Self {
        self.custom_server_addr(server_addr)
    }
    #[doc(hidden)]
    #[deprecated = "renamed to `custom_socket_addr`."]
    pub fn custom_resolved_address(self, socket_addr: SocketAddr) -> Self {
        self.custom_socket_addr(socket_addr)
    }
}
