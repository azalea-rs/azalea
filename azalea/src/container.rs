use std::{fmt, fmt::Debug};

use azalea_client::{
    Client,
    inventory::{CloseContainerEvent, ContainerClickEvent, Inventory},
    packet::game::ReceiveGamePacketEvent,
};
use azalea_core::position::BlockPos;
use azalea_inventory::{
    ItemStack, Menu,
    operations::{ClickOperation, PickupClick, QuickMoveClick},
};
use azalea_physics::collision::BlockWithShape;
use azalea_protocol::packets::game::ClientboundGamePacket;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::{component::Component, prelude::MessageReader, system::Commands};
use derive_more::Deref;
use futures_lite::Future;

use crate::bot::BotClientExt;

pub struct ContainerPlugin;
impl Plugin for ContainerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_menu_opened_event);
    }
}

pub trait ContainerClientExt {
    /// Open a container in the world, like a chest.
    ///
    /// Use [`Client::open_inventory`] to open your own inventory.
    ///
    /// `timeout_ticks` indicates how long the client will wait before giving
    /// up and returning `None`. You may need to adjust it based on latency.
    /// Setting the timeout to `None` will result in waiting potentially
    /// forever.
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
    /// let container = bot.open_container_at(target_pos, None).await;
    /// # }
    /// ```
    fn open_container_at(
        &self,
        pos: BlockPos,
        timeout_ticks: Option<usize>,
    ) -> impl Future<Output = Option<ContainerHandle>> + Send;

    /// Wait until a container is open, up to the specified number of ticks.
    ///
    /// Returns `None` if the container was immediately opened and closed, or if
    /// the timeout expires.
    ///
    /// If `timeout_ticks` is None, there will be no timeout.
    fn wait_for_container_open(
        &self,
        timeout_ticks: Option<usize>,
    ) -> impl Future<Output = Option<ContainerHandle>> + Send;

    /// Open the player's inventory.
    ///
    /// This will return None if another container is open.
    ///
    /// Note that this will send a packet to the server once it's dropped. Also,
    /// due to how it's implemented, you could call this function multiple times
    /// while another inventory handle already exists (but you shouldn't).
    ///
    /// If you just want to get the items in the player's inventory without
    /// sending any packets, use [`Client::menu`], [`Menu::player_slots_range`],
    /// and [`Menu::slots`].
    fn open_inventory(&self) -> Option<ContainerHandle>;
    /// Returns a [`ContainerHandleRef`] to the client's currently open
    /// container, or their inventory.
    ///
    /// This will not send a packet to close the container when it's dropped,
    /// which may cause anticheat compatibility issues if you modify your
    /// inventory without closing it afterwards.
    ///
    /// To simulate opening your own inventory (like pressing 'e') in a way that
    /// won't trigger anticheats, use [`Client::open_inventory`].
    ///
    /// To open a container in the world, use [`Client::open_container_at`].
    fn get_inventory(&self) -> ContainerHandleRef;
    /// Get the item in the bot's hotbar that is currently being held in its
    /// main hand.
    fn get_held_item(&self) -> ItemStack;
}

impl ContainerClientExt for Client {
    async fn open_container_at(
        &self,
        pos: BlockPos,
        timeout_ticks: Option<usize>,
    ) -> Option<ContainerHandle> {
        let mut ticks = self.get_tick_broadcaster();
        // wait until it's not air (up to 10 ticks)
        for _ in 0..10 {
            let block = self.world().read().get_block_state(pos).unwrap_or_default();
            if !block.is_collision_shape_empty() {
                break;
            }
            let _ = ticks.recv().await;
        }

        self.ecs
            .lock()
            .entity_mut(self.entity)
            .insert(WaitingForInventoryOpen);
        self.block_interact(pos);

        self.wait_for_container_open(timeout_ticks).await
    }

    async fn wait_for_container_open(
        &self,
        timeout_ticks: Option<usize>,
    ) -> Option<ContainerHandle> {
        let mut ticks = self.get_tick_broadcaster();
        let mut elapsed_ticks = 0;
        while ticks.recv().await.is_ok() {
            let ecs = self.ecs.lock();
            if ecs.get::<WaitingForInventoryOpen>(self.entity).is_none() {
                break;
            }

            elapsed_ticks += 1;
            if let Some(timeout_ticks) = timeout_ticks
                && elapsed_ticks >= timeout_ticks
            {
                return None;
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

    fn open_inventory(&self) -> Option<ContainerHandle> {
        let ecs = self.ecs.lock();
        let inventory = ecs.get::<Inventory>(self.entity).expect("no inventory");
        if inventory.id == 0 {
            Some(ContainerHandle::new(0, self.clone()))
        } else {
            None
        }
    }

    fn get_inventory(&self) -> ContainerHandleRef {
        self.query_self::<&Inventory, _>(|inv| ContainerHandleRef::new(inv.id, self.clone()))
    }

    fn get_held_item(&self) -> ItemStack {
        self.query_self::<&Inventory, _>(|inv| inv.held_item())
    }
}

/// A handle to a container that may be open.
///
/// This does not close the container when it's dropped. See [`ContainerHandle`]
/// if that behavior is desired.
pub struct ContainerHandleRef {
    id: i32,
    client: Client,
}
impl Debug for ContainerHandleRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ContainerHandle")
            .field("id", &self.id())
            .finish()
    }
}
impl ContainerHandleRef {
    pub fn new(id: i32, client: Client) -> Self {
        Self { id, client }
    }

    pub fn close(&self) {
        self.client.ecs.lock().trigger(CloseContainerEvent {
            entity: self.client.entity,
            id: self.id,
        });
    }

    /// Get the ID of the container.
    ///
    /// If this is 0, that means it's the player's inventory. Otherwise, the
    /// number isn't really meaningful since only one container can be open
    /// at a time.
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Returns the menu of the container.
    ///
    /// If the container is closed, this will return `None`.
    ///
    /// Note that any modifications you make to the `Menu` you're given will not
    /// actually cause any packets to be sent. If you're trying to modify your
    /// inventory, use [`Self::click`] instead
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
    /// inventory.
    ///
    /// If the container is closed, this will return `None`.
    pub fn contents(&self) -> Option<Vec<ItemStack>> {
        self.menu().map(|menu| menu.contents())
    }

    /// Return the contents of the menu, including the player's inventory.
    ///
    /// If the container is closed, this will return `None`.
    pub fn slots(&self) -> Option<Vec<ItemStack>> {
        self.menu().map(|menu| menu.slots())
    }

    /// A shortcut for [`Self::click`] with `PickupClick::Left`.
    pub fn left_click(&self, slot: impl Into<usize>) {
        self.click(PickupClick::Left {
            slot: Some(slot.into() as u16),
        });
    }
    /// A shortcut for [`Self::click`] with `QuickMoveClick::Left`.
    pub fn shift_click(&self, slot: impl Into<usize>) {
        self.click(QuickMoveClick::Left {
            slot: slot.into() as u16,
        });
    }
    /// A shortcut for [`Self::click`] with `PickupClick::Right`.
    pub fn right_click(&self, slot: impl Into<usize>) {
        self.click(PickupClick::Right {
            slot: Some(slot.into() as u16),
        });
    }

    /// Simulate a click in the container and send the packet to perform the
    /// action.
    pub fn click(&self, operation: impl Into<ClickOperation>) {
        let operation = operation.into();
        self.client.ecs.lock().trigger(ContainerClickEvent {
            entity: self.client.entity,
            window_id: self.id,
            operation,
        });
    }
}

/// A handle to the open container.
///
/// The container will be closed once this is dropped.
#[derive(Deref)]
pub struct ContainerHandle(ContainerHandleRef);

impl Drop for ContainerHandle {
    fn drop(&mut self) {
        self.0.close();
    }
}
impl Debug for ContainerHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ContainerHandle")
            .field("id", &self.id())
            .finish()
    }
}
impl ContainerHandle {
    fn new(id: i32, client: Client) -> Self {
        Self(ContainerHandleRef { id, client })
    }

    /// Closes the inventory by dropping the handle.
    pub fn close(self) {
        // implicitly calls drop
    }
}

#[derive(Component, Debug)]
pub struct WaitingForInventoryOpen;

pub fn handle_menu_opened_event(
    mut commands: Commands,
    mut events: MessageReader<ReceiveGamePacketEvent>,
) {
    for event in events.read() {
        if let ClientboundGamePacket::ContainerSetContent { .. } = event.packet.as_ref() {
            commands
                .entity(event.entity)
                .remove::<WaitingForInventoryOpen>();
        }
    }
}
