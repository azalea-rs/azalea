use azalea_core::position::Vec3;
use azalea_entity::{
    Attributes, Dead, EntityUuid, Position, dimensions::EntityDimensions, metadata::Health,
};
use azalea_world::{InstanceName, MinecraftEntityId};
use uuid::Uuid;

use super::EntityRef;
use crate::Client;

macro_rules! impl_entity_functions {
    ( $(
        Client:
        $(#[$client_doc:meta])*
        EntityRef:
        $(#[$entityref_doc:meta])*
        pub fn $fn_name:ident (&$fn_self:ident) -> $fn_returns:ty $fn_impl:block
    )* ) => {
        $(
            impl Client {
                $(#[$client_doc])*
                pub fn $fn_name(&$fn_self) -> $fn_returns $fn_impl
            }
            impl EntityRef {
                $(#[$entityref_doc])*
                pub fn $fn_name(&$fn_self) -> $fn_returns $fn_impl
            }
        )*
    };
}

// these functions are defined this way because we want them to be duplicated
// for both Client and EntityRef but still have their own documentation
impl_entity_functions! {
    Client:
    /// Get the client's position in the world, which is the same as its feet
    /// position.
    ///
    /// This is a shortcut for `**bot.component::<Position>()`.
    ///
    /// To get the client's eye position, use [`Self::eye_position`].
    ///
    /// Note that this value is given a default of [`Vec3::ZERO`] when it
    /// receives the login packet, its true position may be set ticks
    /// later.
    EntityRef:
    /// Get the entity's position in the world, which is the same as its feet
    /// position.
    ///
    /// To get the client's eye position, use [`Self::eye_position`].
    ///
    /// Also see [`Client::position`].
    pub fn position(&self) -> Vec3 {
        **self.component::<Position>()
    }

    Client:
    /// Get the bounding box dimensions for our client, which contains our
    /// width, height, and eye height.
    ///
    /// This is a shortcut for
    /// `self.component::<EntityDimensions>()`.
    EntityRef:
    /// Get the bounding box dimensions for the entity, which contains its
    /// width, height, and eye height.
    ///
    /// Also see [`Client::dimensions`]
    pub fn dimensions(&self) -> EntityDimensions {
        self.component::<EntityDimensions>().clone()
    }

    Client:
    /// Get the position of this client's eyes.
    ///
    /// Also see [`Self::position`].
    ///
    /// This is a shortcut for
    /// `bot.position().up(bot.dimensions().eye_height)`.
    EntityRef:
    /// Get the position of this entity's eyes.
    ///
    /// Also see [`Client::eye_position`].
    pub fn eye_position(&self) -> Vec3 {
        self.query_self::<(&Position, &EntityDimensions), _>(|(pos, dim)| {
            pos.up(dim.eye_height as f64)
        })
    }

    Client:
    /// Get the health of this client.
    ///
    /// This is a shortcut for `*bot.component::<Health>()`.
    EntityRef:
    /// Get the health of this entity.
    ///
    /// Also see [`Client::health`].
    pub fn health(&self) -> f32 {
        **self.component::<Health>()
    }

    Client:
    /// Get the Minecraft UUID of this client.
    ///
    /// This is a shortcut for `**self.component::<EntityUuid>()`.
    EntityRef:
    /// Get the Minecraft UUID of this entity.
    ///
    /// Also see [`Client::uuid`].
    pub fn uuid(&self) -> Uuid {
        **self.component::<EntityUuid>()
    }

    Client:
    /// Get the Minecraft ID of this client.
    ///
    /// See [`MinecraftEntityId`] for more details. For persistent identifiers,
    /// consider using [`Self::uuid`] instead.
    ///
    /// This is a shortcut for `**self.component::<MinecraftEntityId>()`.
    EntityRef:
    /// Get the Minecraft UUID of this entity.
    ///
    /// See [`MinecraftEntityId`] for more details. For persistent identifiers,
    /// consider using [`Self::uuid`] instead.
    ///
    /// Also see [`Client::minecraft_id`].
    pub fn minecraft_id(&self) -> MinecraftEntityId {
        *self.component::<MinecraftEntityId>()
    }

    Client:
    /// Returns the attribute values of our player, which can be used to
    /// determine things like our movement speed.
    EntityRef:
    /// Returns the attribute values of the entity, which can be used to
    /// determine things like its movement speed.
    pub fn attributes(&self) -> Attributes {
        // this *could* return a mapped read guard for performance but that rarely
        // matters and it's just easier for the user if it doesn't.
        self.component::<Attributes>().clone()
    }

    Client:
    /// Get the name of the instance (world) that the bot is in.
    ///
    /// This can be used to check if the client is in the same world as another
    /// entity.
    #[doc(alias("world_name", "dimension_name"))]
    EntityRef:
    /// Get the name of the instance (world) that the entity is in.
    ///
    /// This can be used to check if the entity is in the same world as another
    /// entity.
    ///
    /// Also see [`Client::instance_name`],
    #[doc(alias("world_name", "dimension_name"))]
    pub fn instance_name(&self) -> InstanceName {
        (*self.component::<InstanceName>()).clone()
    }

    Client:
    /// Returns whether the client is alive and in the world.
    ///
    /// You should avoid using this if you have auto-respawn enabled (which is
    /// the default), instead consider watching for
    /// [`Event::Death`](crate::Event::Death) instead.
    ///
    /// Also see [`Self::exists`].
    EntityRef:
    /// Returns whether the entity is alive and hasn't despawned.
    ///
    /// Unlike most functions in `EntityRef`, this one will not panic if the
    /// entity is despawned. Because of this, it may be useful to check `is_alive`
    /// before calling functions that request data from the world.
    ///
    /// Also see [`Client::is_alive`] and [`Self::exists`].
    pub fn is_alive(&self) -> bool {
        self.try_query_self::<Option<&Dead>, _>(|dead| dead.is_none()).unwrap_or(false)
    }

    Client:
    /// Returns whether the client is in the world (has been assigned an entity ID).
    ///
    /// Like [`Self::is_alive`], this will not panic.
    EntityRef:
    /// Returns whether the entity is in the world and hasn't despawned.
    ///
    /// Like [`Self::is_alive`], this will not panic.
    ///
    /// Also see [`Client::exists`].
    pub fn exists(&self) -> bool {
        self.try_query_self::<Option<&MinecraftEntityId>, _>(|entity_id| entity_id.is_some()).unwrap_or(false)
    }
}
