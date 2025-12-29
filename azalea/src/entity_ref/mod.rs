use azalea_core::position::Vec3;
use azalea_entity::{
    Attributes, EntityUuid, Position, dimensions::EntityDimensions, metadata::Health,
};
use azalea_world::InstanceName;
use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{QueryData, QueryItem},
};
use parking_lot::MappedRwLockReadGuard;
use uuid::Uuid;

use crate::Client;

/// A reference to an entity in a world, allowing simpler access to certain
/// functions.
///
/// Note that this stores the [`Client`] alongside the entity.
#[derive(Clone)]
pub struct EntityRef {
    client: Client,
    entity: Entity,
}

impl EntityRef {
    pub fn new(client: Client, entity: Entity) -> Self {
        Self { client, entity }
    }

    /// Returns the ECS identifier for the entity.
    pub fn id(&self) -> Entity {
        self.entity
    }

    /// Get a component on the entity.
    ///
    /// This allows you to access certain data stored about the entity that
    /// isn't accessible in a simpler way.
    ///
    /// See [`Client::component`] for more details.
    ///
    /// # Panics
    ///
    /// This will panic if the component doesn't exist on the client. Use
    /// [`Self::get_component`] to avoid this.
    ///
    /// # Examples
    ///
    /// ```
    /// # use azalea_world::InstanceName;
    /// # fn example(client: &azalea::Client) {
    /// let world_name = client.component::<InstanceName>();
    /// # }
    pub fn component<T: Component>(&self) -> MappedRwLockReadGuard<'_, T> {
        self.client.entity_component(self.entity)
    }

    /// Get a component on this client, or `None` if it doesn't exist.
    ///
    /// If the component is guaranteed to be present, consider using
    /// [`Self::component`].
    ///
    /// See [`Client::component`] for more details.
    pub fn get_component<T: Component>(&self) -> Option<MappedRwLockReadGuard<'_, T>> {
        self.client.get_entity_component(self.entity)
    }

    /// Query the ECS for data from the entity.
    ///
    /// You can use this to mutate data on the entity.
    ///
    /// Also see [`Client::query_self`] and [`Client::query_entity`].
    ///
    /// # Panics
    ///
    /// This will panic if the entity is missing a component required by the
    /// query.
    pub fn query_self<D: QueryData, R>(&self, f: impl FnOnce(QueryItem<D>) -> R) -> R {
        self.client.query_entity(self.entity, f)
    }
}

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
    /// Get the name of the instance (world) that the bot is in.
    ///
    /// This can be used to check if the client is in the same world as another
    /// entity.
    #[doc(alias("world_name", "dimension_name"))]
    pub fn instance_name(&self) -> InstanceName {
        (*self.component::<InstanceName>()).clone()
    }

}
