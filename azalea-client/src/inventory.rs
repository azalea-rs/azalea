use std::collections::HashMap;

use azalea_chat::FormattedText;
use azalea_inventory::operations::ClickOperation;
pub use azalea_inventory::*;
use azalea_protocol::packets::game::{
    serverbound_container_click_packet::ServerboundContainerClickPacket,
    serverbound_container_close_packet::ServerboundContainerClosePacket,
};
use azalea_registry::MenuKind;
use bevy_app::{App, Plugin};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::EventReader,
    prelude::EventWriter,
    schedule::{IntoSystemConfig, IntoSystemConfigs},
    system::Query,
};
use log::warn;

use crate::{local_player::handle_send_packet_event, Client, LocalPlayer};

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ClientSideCloseContainerEvent>()
            .add_event::<MenuOpenedEvent>()
            .add_event::<CloseContainerEvent>()
            .add_event::<ContainerClickEvent>()
            .add_systems(
                (
                    handle_menu_opened_event,
                    handle_container_click_event,
                    handle_container_close_event.before(handle_send_packet_event),
                    handle_client_side_close_container_event,
                )
                    .chain(),
            );
    }
}

impl Client {
    /// Return the menu that is currently open. If no menu is open, this will
    /// have the player's inventory.
    pub fn menu(&self) -> Menu {
        let mut ecs = self.ecs.lock();
        let inventory = self.query::<&InventoryComponent>(&mut ecs);
        inventory.menu().clone()
    }
}

/// A component present on all local players that have an inventory.
#[derive(Component)]
pub struct InventoryComponent {
    /// A component that contains the player's inventory menu. This is
    /// guaranteed to be a `Menu::Player`.
    ///
    /// We keep it as a [`Menu`] since `Menu` has some useful functions that
    /// bare [`azalea_inventory::Player`] doesn't have.
    pub inventory_menu: azalea_inventory::Menu,

    /// The ID of the container that's currently open. Its value is not
    /// guaranteed to be anything specific, and may change every time you open a
    /// container (unless it's 0, in which case it means that no container is
    /// open).
    pub id: u8,
    /// The current container menu that the player has open. If no container is
    /// open, this will be `None`.
    pub container_menu: Option<azalea_inventory::Menu>,
    /// The item that is currently held by the cursor. `Slot::Empty` if nothing
    /// is currently being held.
    pub carried: ItemSlot,
    /// An identifier used by the server to track client inventory desyncs. This
    /// is sent on every container click, and it's only ever updated when the
    /// server sends a new container update.
    pub state_id: u32,
    // minecraft also has these fields, but i don't think they're necessary?:
    // private final NonNullList<ItemStack> remoteSlots;
    // private final IntList remoteDataSlots;
    // private ItemStack remoteCarried;
}
impl InventoryComponent {
    /// Returns a reference to the currently active menu. If a container is open
    /// it'll return [`Self::container_menu`], otherwise
    /// [`Self::inventory_menu`].
    ///
    /// Use [`Self::menu_mut`] if you need a mutable reference.
    pub fn menu(&self) -> &azalea_inventory::Menu {
        if let Some(menu) = &self.container_menu {
            menu
        } else {
            &self.inventory_menu
        }
    }

    /// Returns a mutable reference to the currently active menu. If a container
    /// is open it'll return [`Self::container_menu`], otherwise
    /// [`Self::inventory_menu`].
    ///
    /// Use [`Self::menu`] if you don't need a mutable reference.
    pub fn menu_mut(&mut self) -> &mut azalea_inventory::Menu {
        if let Some(menu) = &mut self.container_menu {
            menu
        } else {
            &mut self.inventory_menu
        }
    }
}

impl Default for InventoryComponent {
    fn default() -> Self {
        InventoryComponent {
            inventory_menu: Menu::Player(azalea_inventory::Player::default()),
            id: 0,
            container_menu: None,
            carried: ItemSlot::Empty,
            state_id: 0,
        }
    }
}

/// Sent from the server when a menu (like a chest or crafting table) was
/// opened by the client.
pub struct MenuOpenedEvent {
    pub entity: Entity,
    pub window_id: u32,
    pub menu_type: MenuKind,
    pub title: FormattedText,
}
fn handle_menu_opened_event(
    mut events: EventReader<MenuOpenedEvent>,
    mut query: Query<&mut InventoryComponent>,
) {
    for event in events.iter() {
        let mut inventory = query.get_mut(event.entity).unwrap();
        inventory.id = event.window_id as u8;
        inventory.container_menu = Some(Menu::from_kind(event.menu_type));
    }
}

/// Tell the server that we want to close a container.
///
/// Note that this is also sent when the client closes its own inventory, even
/// though there is no packet for opening its inventory.
pub struct CloseContainerEvent {
    pub entity: Entity,
    /// The ID of the container to close. 0 for the player's inventory. If this
    /// is not the same as the currently open inventory, nothing will happen.
    pub id: u8,
}
fn handle_container_close_event(
    mut events: EventReader<CloseContainerEvent>,
    mut client_side_events: EventWriter<ClientSideCloseContainerEvent>,
    query: Query<(&LocalPlayer, &InventoryComponent)>,
) {
    for event in events.iter() {
        let (local_player, inventory) = query.get(event.entity).unwrap();
        if event.id != inventory.id {
            warn!(
                "Tried to close container with ID {}, but the current container ID is {}",
                event.id, inventory.id
            );
            continue;
        }

        local_player.write_packet(
            ServerboundContainerClosePacket {
                container_id: inventory.id as u8,
            }
            .get(),
        );
        client_side_events.send(ClientSideCloseContainerEvent {
            entity: event.entity,
        });
    }
}

/// Close a container without notifying the server.
///
/// Note that this also gets fired when we get a [`CloseContainerEvent`].
pub struct ClientSideCloseContainerEvent {
    pub entity: Entity,
}
fn handle_client_side_close_container_event(
    mut events: EventReader<ClientSideCloseContainerEvent>,
    mut query: Query<&mut InventoryComponent>,
) {
    for event in events.iter() {
        let mut inventory = query.get_mut(event.entity).unwrap();
        inventory.container_menu = None;
        inventory.id = 0;
    }
}

#[derive(Debug)]
pub struct ContainerClickEvent {
    pub entity: Entity,
    pub window_id: u8,
    pub operation: ClickOperation,
}
fn handle_container_click_event(
    mut events: EventReader<ContainerClickEvent>,
    mut query: Query<(&mut InventoryComponent, &LocalPlayer)>,
) {
    for event in events.iter() {
        let (mut inventory, local_player) = query.get_mut(event.entity).unwrap();
        if inventory.id != event.window_id {
            warn!(
                "Tried to click container with ID {}, but the current container ID is {}",
                event.window_id, inventory.id
            );
            continue;
        }

        let menu = inventory.menu_mut();
        let old_slots = menu.slots().clone();

        // menu.click(&event.operation);

        // see which slots changed after clicking and put them in the hashmap
        // the server uses this to check if we desynced
        let mut changed_slots: HashMap<u16, ItemSlot> = HashMap::new();
        for slot_index in 0..menu.len() {
            let old_slot = &old_slots[slot_index];
            let new_slot = &menu.slots()[slot_index];
            if old_slot != new_slot {
                changed_slots.insert(slot_index as u16, new_slot.clone());
            }
        }

        local_player.write_packet(
            ServerboundContainerClickPacket {
                container_id: event.window_id,
                state_id: inventory.state_id,
                slot_num: event.operation.slot_num().map(|n| n as i16).unwrap_or(-999),
                button_num: event.operation.button_num(),
                click_type: event.operation.click_type(),
                changed_slots: changed_slots,
                carried_item: inventory.carried.clone(),
            }
            .get(),
        )
    }
}
