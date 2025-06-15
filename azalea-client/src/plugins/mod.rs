use bevy_app::{PluginGroup, PluginGroupBuilder};

pub mod attack;
pub mod auto_reconnect;
pub mod brand;
pub mod chat;
pub mod chat_signing;
pub mod chunks;
pub mod connection;
pub mod disconnect;
pub mod events;
pub mod interact;
pub mod inventory;
pub mod join;
pub mod loading;
pub mod login;
pub mod mining;
pub mod movement;
pub mod packet;
pub mod pong;
pub mod respawn;
pub mod task_pool;
pub mod tick_broadcast;
pub mod tick_end;

/// This plugin group will add all the default plugins necessary for Azalea to
/// work.
pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        #[allow(unused_mut)]
        let mut group = PluginGroupBuilder::start::<Self>()
            .add(crate::client::AmbiguityLoggerPlugin)
            .add(bevy_time::TimePlugin)
            .add(packet::PacketPlugin)
            .add(crate::client::AzaleaPlugin)
            .add(azalea_entity::EntityPlugin)
            .add(azalea_physics::PhysicsPlugin)
            .add(events::EventsPlugin)
            .add(task_pool::TaskPoolPlugin::default())
            .add(inventory::InventoryPlugin)
            .add(chat::ChatPlugin)
            .add(disconnect::DisconnectPlugin)
            .add(movement::MovementPlugin)
            .add(interact::InteractPlugin)
            .add(respawn::RespawnPlugin)
            .add(mining::MiningPlugin)
            .add(attack::AttackPlugin)
            .add(chunks::ChunksPlugin)
            .add(tick_end::TickEndPlugin)
            .add(loading::PlayerLoadedPlugin)
            .add(brand::BrandPlugin)
            .add(tick_broadcast::TickBroadcastPlugin)
            .add(pong::PongPlugin)
            .add(connection::ConnectionPlugin)
            .add(login::LoginPlugin)
            .add(join::JoinPlugin)
            .add(auto_reconnect::AutoReconnectPlugin)
            .add(chat_signing::ChatSigningPlugin);
        #[cfg(feature = "log")]
        {
            group = group.add(bevy_log::LogPlugin::default());
        }
        group
    }
}
