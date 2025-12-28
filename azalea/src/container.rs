use std::{fmt, fmt::Debug};

use azalea_chat::FormattedText;
use azalea_client::{
    inventory::{CloseContainerEvent, ContainerClickEvent},
    packet::game::ReceiveGamePacketEvent,
};
use azalea_core::position::BlockPos;
use azalea_entity::inventory::Inventory;
use azalea_inventory::{
    ItemStack, Menu,
    operations::{ClickOperation, PickupClick, QuickMoveClick},
};
use azalea_physics::collision::BlockWithShape;
use azalea_protocol::packets::game::ClientboundGamePacket;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::{component::Component, prelude::MessageReader, system::Commands};
use derive_more::Deref;

use crate::Client;

pub struct ContainerPlugin;
impl Plugin for ContainerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_menu_opened_event);
    }
}

impl Client {
    /// Open a container in the world, like a chest.
    ///
    /// Use [`Client::open_inventory`] to open your own inventory.
    ///
    /// This function times out after 5 seconds (100 ticks). Use
    /// [`Self::open_container_at_with_timeout_ticks`] if you would like to
    /// configure this.
    ///
    /// ```
    /// # use azalea::{prelude::*, registry::builtin::BlockKind};
    /// # async fn example(mut bot: azalea::Client) {
    /// let target_pos = bot
    ///     .world()
    ///     .read()
    ///     .find_block(bot.position(), &BlockKind::Chest.into());
    /// let Some(target_pos) = target_pos else {
    ///     bot.chat("no chest found");
    ///     return;
    /// };
    /// let container = bot.open_container_at(target_pos).await;
    /// # }
    /// ```
    pub async fn open_container_at(&self, pos: BlockPos) -> Option<ContainerHandle> {
        self.open_container_at_with_timeout_ticks(pos, Some(20 * 5))
            .await
    }

    /// Open a container in the world, or time out after a specified amount of
    /// ticks.
    ///
    /// See [`Self::open_container_at`] for more information. That function
    /// defaults to a timeout of 5 seconds (100 ticks), which is usually good
    /// enough. However to detect failures faster or to account for server
    /// lag, you may find it useful to adjust the timeout to a different
    /// value.
    ///
    /// The timeout is measured in game ticks (on the client, not the server),
    /// i.e. 1/20th of a second.
    pub async fn open_container_at_with_timeout_ticks(
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
            .write()
            .entity_mut(self.entity)
            .insert(WaitingForInventoryOpen);
        self.block_interact(pos);

        self.wait_for_container_open(timeout_ticks).await
    }

    /// Wait until a container is open, up to the specified number of ticks.
    ///
    /// Returns `None` if the container was immediately opened and closed, or if
    /// the timeout expired.
    ///
    /// If `timeout_ticks` is None, there will be no timeout.
    pub async fn wait_for_container_open(
        &self,
        timeout_ticks: Option<usize>,
    ) -> Option<ContainerHandle> {
        let mut ticks = self.get_tick_broadcaster();
        let mut elapsed_ticks = 0;
        while ticks.recv().await.is_ok() {
            let ecs = self.ecs.read();
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

        let ecs = self.ecs.read();
        let inventory = ecs.get::<Inventory>(self.entity).expect("no inventory");
        if inventory.id == 0 {
            None
        } else {
            Some(ContainerHandle::new(inventory.id, self.clone()))
        }
    }

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
    pub fn open_inventory(&self) -> Option<ContainerHandle> {
        let inventory = self.component::<Inventory>();
        if inventory.id == 0 {
            Some(ContainerHandle::new(0, self.clone()))
        } else {
            None
        }
    }

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
    pub fn get_inventory(&self) -> ContainerHandleRef {
        ContainerHandleRef::new(self.component::<Inventory>().id, self.clone())
    }

    /// Get the item in the bot's hotbar that is currently being held in its
    /// main hand.
    pub fn get_held_item(&self) -> ItemStack {
        self.component::<Inventory>().held_item().clone()
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
        self.client.ecs.write().trigger(CloseContainerEvent {
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
        self.map_inventory(|inv| {
            if self.id == 0 {
                inv.inventory_menu.clone()
            } else {
                inv.container_menu.clone().unwrap()
            }
        })
    }

    fn map_inventory<R>(&self, f: impl FnOnce(&Inventory) -> R) -> Option<R> {
        self.client.query_self::<&Inventory, _>(|inv| {
            if inv.id == self.id {
                Some(f(inv))
            } else {
                // a different inventory is open
                None
            }
        })
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

    /// Returns the title of the container, or `None` if no container is open.
    ///
    /// ```no_run
    /// # use azalea::prelude::*;
    /// # fn example(bot: &Client) {
    /// let inventory = bot.get_inventory();
    /// let inventory_title = inventory.title().unwrap_or_default().to_string();
    /// // would be true if an unnamed chest is open
    /// assert_eq!(inventory_title, "Chest");
    /// # }
    /// ```
    pub fn title(&self) -> Option<FormattedText> {
        self.map_inventory(|inv| inv.container_menu_title.clone())
            .flatten()
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
        self.client.ecs.write().trigger(ContainerClickEvent {
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
