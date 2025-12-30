pub mod shared_impls;

use std::fmt::Debug;

use azalea_entity::EntityKindComponent;
use azalea_registry::builtin::EntityKind;
use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{QueryData, QueryEntityError, QueryItem},
};
use parking_lot::MappedRwLockReadGuard;

use crate::Client;

/// A reference to an entity in a world.
///
/// This is different from [`Entity`], since you can perform actions with just
/// an `EntityRef` instead of it only being an identifier.
///
/// Most functions on `EntityRef` that return a value will result in a panic if
/// the client has despawned, so if your code involves waiting, you should check
/// [`Self::is_alive`] or [`Self::exists`] before calling those functions.
///
/// Also, since `EntityRef` stores the [`Client`] alongside the entity, this
/// means that it supports interactions such as [`Self::attack`].
///
/// Not to be confused with Bevy's [`EntityRef`](bevy_ecs::world::EntityRef).
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
    /// This will panic if the entity doesn't exist or is missing a component
    /// required by the query. Consider using [`Self::try_query_self`] to
    /// avoid this.
    pub fn query_self<D: QueryData, R>(&self, f: impl FnOnce(QueryItem<D>) -> R) -> R {
        self.client.query_entity(self.entity, f)
    }

    /// Query the ECS for data from the entity, or return an error if the query
    /// fails.
    ///
    /// Also see [`Self::query_self`].
    pub fn try_query_self<D: QueryData, R>(
        &self,
        f: impl FnOnce(QueryItem<D>) -> R,
    ) -> Result<R, QueryEntityError> {
        self.client.try_query_entity(self.entity, f)
    }
}

impl Debug for EntityRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EntityRef")
            .field("client", &self.client.entity)
            .field("entity", &self.entity)
            .finish()
    }
}

impl EntityRef {
    /// Returns the type of entity that this is.
    pub fn kind(&self) -> EntityKind {
        **self.component::<EntityKindComponent>()
    }
}

impl EntityRef {
    /// Attack this entity from the client that created this `EntityRef`.
    ///
    /// Also see [`Client::attack`].
    pub fn attack(&self) {
        self.client.attack(self.entity);
    }

    /// Right-click this entity from the client that created this `EntityRef`.
    ///
    /// See [`Client::entity_interact`] for more information.
    pub fn interact(&self) {
        self.client.entity_interact(self.entity);
    }
}
