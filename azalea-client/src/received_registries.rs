use azalea_protocol::packets::game::clientbound_login_packet::registry::RegistryRoot;
use bevy_ecs::component::Component;
use derive_more::Deref;

/// The registries that the server sent us on login.
#[derive(Clone, Debug, Component, Deref)]
pub struct ReceivedRegistries(pub RegistryRoot);
