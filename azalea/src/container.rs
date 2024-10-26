use std::fmt::Debug;
use std::fmt::Formatter;

use azalea_client::{
    inventory::{CloseContainerEvent, ContainerClickEvent, Inventory},
    packet_handling::game::PacketEvent,
    Client,
};
use azalea_core::position::BlockPos;
use azalea_inventory::{operations::ClickOperation, ItemSlot, Menu};
use azalea_protocol::packets::game::ClientboundGamePacket;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::{component::Component, prelude::EventReader, system::Commands};
use futures_lite::Future;

use crate::bot::BotClientExt;

pub struct ContainerPlugin;
impl Plugin for ContainerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_menu_opened_event);
    }
}

pub trait ContainerClientExt {
    fn open_container_at(
        &mut self,
        pos: BlockPos,
    ) -> impl Future<Output = Option<ContainerHandle>> + Send;
    fn open_inventory(&mut self) -> Option<ContainerHandle>;
    fn get_open_container(&self) -> Option<ContainerHandleRef>;
}

impl ContainerClientExt for Client {
    /// Open a container in the world, like a chest. Use
    /// [`Client::open_inventory`] to open your own inventory.
    ///
    /// ```
    /// # use azalea::prelude::*;
    /// # async fn example(mut bot: azalea::Client) {
    /// let target_pos = bot
    ///     .world()
    ///     .read()
    ///     .find_block(bot.position(), &azalea::registry::Block::Chest.into());
    /// let Some(target_pos) = target_pos else {
    ///     bot.chat("no chest found");
    ///     return;
    /// };
    /// let container = bot.open_container_at(target_pos).await;
    /// # }
    /// ```
    async fn open_container_at(&mut self, pos: BlockPos) -> Option<ContainerHandle> {
        self.ecs
            .lock()
            .entity_mut(self.entity)
            .insert(WaitingForInventoryOpen);
        self.block_interact(pos);

        let mut receiver = self.get_tick_broadcaster();
        while receiver.recv().await.is_ok() {
            let ecs = self.ecs.lock();
            if ecs.get::<WaitingForInventoryOpen>(self.entity).is_none() {
                break;
            }
        }

        let ecs = self.ecs.lock();
        let inventory = ecs.get::<Inventory>(self.entity).expect("no inventory");
        if inventory.id == 0 {
            None
        } else {
            Some(ContainerHandle::new(inventory.id, self.clone()))
        }
    }

    /// Open the player's inventory. This will return None if another
    /// container is open.
    ///
    /// Note that this will send a packet to the server once it's dropped. Also,
    /// due to how it's implemented, you could call this function multiple times
    /// while another inventory handle already exists (but you shouldn't).
    ///
    /// If you just want to get the items in the player's inventory without
    /// sending any packets, use [`Client::menu`], [`Menu::player_slots_range`],
    /// and [`Menu::slots`].
    fn open_inventory(&mut self) -> Option<ContainerHandle> {
        let ecs = self.ecs.lock();
        let inventory = ecs.get::<Inventory>(self.entity).expect("no inventory");

        if inventory.id == 0 {
            Some(ContainerHandle::new(0, self.clone()))
        } else {
            None
        }
    }

    /// Get a handle to the open container. This will return None if no
    /// container is open. This will not close the container when it's dropped.
    ///
    /// See [`Client::open_inventory`] or [`Client::menu`] if you want to open
    /// your own inventory.
    fn get_open_container(&self) -> Option<ContainerHandleRef> {
        let ecs = self.ecs.lock();
        let inventory = ecs.get::<Inventory>(self.entity).expect("no inventory");

        if inventory.id == 0 {
            None
        } else {
            Some(ContainerHandleRef {
                id: inventory.id,
                client: self.clone(),
            })
        }
    }
}

/// A handle to a container that may be open. This does not close the container
/// when it's dropped. See [`ContainerHandle`] if that behavior is desired.
pub struct ContainerHandleRef {
    id: u8,
    client: Client,
}
impl Debug for ContainerHandleRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContainerHandle")
            .field("id", &self.id())
            .finish()
    }
}
impl ContainerHandleRef {
    pub fn close(&self) {
        self.client.ecs.lock().send_event(CloseContainerEvent {
            entity: self.client.entity,
            id: self.id,
        });
    }

    /// Get the id of the container. If this is 0, that means it's the player's
    /// inventory. Otherwise, the number isn't really meaningful since only one
    /// container can be open at a time.
    pub fn id(&self) -> u8 {
        self.id
    }

    /// Returns the menu of the container. If the container is closed, this
    /// will return `None`.
    ///
    /// Note that any modifications you make to the `Menu` you're given will not
    /// actually cause any packets to be sent. If you're trying to modify your
    /// inventory, use [`ContainerHandle::click`] instead
    pub fn menu(&self) -> Option<Menu> {
        let ecs = self.client.ecs.lock();
        let inventory = ecs
            .get::<Inventory>(self.client.entity)
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

/// A handle to the open container. The container will be closed once this is
/// dropped.
pub struct ContainerHandle(ContainerHandleRef);

impl Drop for ContainerHandle {
    fn drop(&mut self) {
        self.0.close();
    }
}
impl Debug for ContainerHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContainerHandle")
            .field("id", &self.id())
            .finish()
    }
}
impl ContainerHandle {
    fn new(id: u8, client: Client) -> Self {
        Self(ContainerHandleRef { id, client })
    }

    /// Get the id of the container. If this is 0, that means it's the player's
    /// inventory. Otherwise, the number isn't really meaningful since only one
    /// container can be open at a time.
    pub fn id(&self) -> u8 {
        self.0.id()
    }

    /// Returns the menu of the container. If the container is closed, this
    /// will return `None`.
    ///
    /// Note that any modifications you make to the `Menu` you're given will not
    /// actually cause any packets to be sent. If you're trying to modify your
    /// inventory, use [`ContainerHandle::click`] instead
    pub fn menu(&self) -> Option<Menu> {
        self.0.menu()
    }

    /// Returns the item slots in the container, not including the player's
    /// inventory. If the container is closed, this will return `None`.
    pub fn contents(&self) -> Option<Vec<ItemSlot>> {
        self.0.contents()
    }

    pub fn click(&self, operation: impl Into<ClickOperation>) {
        self.0.click(operation);
    }
}

#[derive(Component, Debug)]
pub struct WaitingForInventoryOpen;

fn handle_menu_opened_event(mut commands: Commands, mut events: EventReader<PacketEvent>) {
    for event in events.read() {
        if let ClientboundGamePacket::ContainerSetContent { .. } = event.packet.as_ref() {
            commands
                .entity(event.entity)
                .remove::<WaitingForInventoryOpen>();
        }
    }
}
