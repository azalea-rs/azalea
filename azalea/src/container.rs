use std::fmt::Formatter;

use azalea_client::{
    inventory::{CloseContainerEvent, ContainerClickEvent, InventoryComponent},
    packet_handling::PacketEvent,
    Client, TickBroadcast,
};
use azalea_core::BlockPos;
use azalea_inventory::{operations::ClickOperation, ItemSlot, Menu};
use azalea_protocol::packets::game::ClientboundGamePacket;
use bevy_app::{App, Plugin};
use bevy_ecs::{component::Component, prelude::EventReader, system::Commands};
use std::fmt::Debug;

pub struct ContainerPlugin;
impl Plugin for ContainerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_menu_opened_event);
    }
}

pub trait ContainerClientExt {
    async fn open_container(&mut self, pos: BlockPos) -> Option<ContainerHandle>;
    fn inventory(&mut self) -> Option<ContainerHandle>;
}

impl ContainerClientExt for Client {
    /// Open a container in the world, like a chest. Use [`Client::inventory`]
    /// to open your own inventory.
    ///
    /// ```
    /// # use azalea::prelude::*;
    /// # async fn example(mut bot: azalea::Client) {
    /// let target_pos = bot
    ///     .world()
    ///     .read()
    ///     .find_block(bot.position(), &azalea::Block::Chest.into());
    /// let Some(target_pos) = target_pos else {
    ///     bot.chat("no chest found");
    ///     return;
    /// };
    /// let container = bot.open_container(target_pos).await;
    /// # }
    /// ```
    async fn open_container(&mut self, pos: BlockPos) -> Option<ContainerHandle> {
        self.ecs
            .lock()
            .entity_mut(self.entity)
            .insert(WaitingForInventoryOpen);
        self.block_interact(pos);

        let mut receiver = {
            let ecs = self.ecs.lock();
            let tick_broadcast = ecs.resource::<TickBroadcast>();
            tick_broadcast.subscribe()
        };
        while receiver.recv().await.is_ok() {
            let ecs = self.ecs.lock();
            if ecs.get::<WaitingForInventoryOpen>(self.entity).is_none() {
                break;
            }
        }

        let ecs = self.ecs.lock();
        let inventory = ecs
            .get::<InventoryComponent>(self.entity)
            .expect("no inventory");
        if inventory.id == 0 {
            None
        } else {
            Some(ContainerHandle {
                id: inventory.id,
                client: self.clone(),
            })
        }
    }

    /// Open the player's inventory. This will return None if another
    /// container is open.
    ///
    /// Note that this will send a packet to the server once it's dropped. Also,
    /// due to how it's implemented, you could call this function multiple times
    /// while another inventory handle already exists (but you shouldn't).
    fn inventory(&mut self) -> Option<ContainerHandle> {
        let ecs = self.ecs.lock();
        let inventory = ecs
            .get::<InventoryComponent>(self.entity)
            .expect("no inventory");

        if inventory.id == 0 {
            Some(ContainerHandle {
                id: 0,
                client: self.clone(),
            })
        } else {
            None
        }
    }
}

/// A handle to the open container. The container will be closed once this is
/// dropped.
pub struct ContainerHandle {
    /// The id of the container. If this is 0, that means it's the player's
    /// inventory.
    id: u8,
    client: Client,
}
impl Drop for ContainerHandle {
    fn drop(&mut self) {
        self.client.ecs.lock().send_event(CloseContainerEvent {
            entity: self.client.entity,
            id: self.id,
        });
    }
}
impl Debug for ContainerHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContainerHandle")
            .field("id", &self.id)
            .finish()
    }
}
impl ContainerHandle {
    /// Get the id of the container. If this is 0, that means it's the player's
    /// inventory. Otherwise, the number isn't really meaningful since only one
    /// container can be open at a time.
    pub fn id(&self) -> u8 {
        self.id
    }

    /// Returns the menu of the container. If the container is closed, this
    /// will return `None`.
    pub fn menu(&self) -> Option<Menu> {
        let ecs = self.client.ecs.lock();
        let inventory = ecs
            .get::<InventoryComponent>(self.client.entity)
            .expect("no inventory");

        // this also makes sure we can't access the inventory while a container is open
        if inventory.id == self.id {
            if self.id == 0 {
                Some(inventory.inventory_menu.clone())
            } else {
                Some(inventory.container_menu.clone().unwrap())
            }
        } else {
            None
        }
    }

    /// Returns the item slots in the container, not including the player's
    /// inventory. If the container is closed, this will return `None`.
    pub fn contents(&self) -> Option<Vec<ItemSlot>> {
        self.menu().map(|menu| menu.contents())
    }

    pub fn click(&self, operation: impl Into<ClickOperation>) {
        let operation = operation.into();
        self.client.ecs.lock().send_event(ContainerClickEvent {
            entity: self.client.entity,
            window_id: self.id,
            operation,
        });
    }
}

#[derive(Component, Debug)]
pub struct WaitingForInventoryOpen;

fn handle_menu_opened_event(mut commands: Commands, mut events: EventReader<PacketEvent>) {
    for event in events.iter() {
        if let ClientboundGamePacket::ContainerSetContent { .. } = event.packet {
            commands
                .entity(event.entity)
                .remove::<WaitingForInventoryOpen>();
        }
    }
}
