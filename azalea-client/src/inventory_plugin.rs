use std::fmt::{Debug, Formatter};

use azalea_chat::FormattedText;
use azalea_core::BlockPos;
use azalea_inventory::{ItemSlot, Menu};
use azalea_protocol::packets::game::serverbound_container_close_packet::ServerboundContainerClosePacket;
use azalea_registry::MenuKind;
use bevy_app::{App, Plugin};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::EventReader,
    prelude::EventWriter,
    schedule::{IntoSystemConfig, IntoSystemConfigs},
    system::{Commands, Query},
};

use crate::{client::TickBroadcast, local_player::handle_send_packet_event, Client, LocalPlayer};

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ClientSideCloseContainerEvent>()
            .add_event::<MenuOpenedEvent>()
            .add_event::<CloseContainerEvent>()
            .add_systems(
                (
                    handle_menu_opened_event,
                    handle_container_close_event.before(handle_send_packet_event),
                    handle_client_side_close_container_event,
                )
                    .chain(),
            );
    }
}

#[derive(Component, Debug)]
pub struct WaitingForInventoryOpen;

impl Client {
    pub async fn open_container(&mut self, pos: BlockPos) -> Option<ContainerHandle> {
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

    /// Return the menu that is currently open. If no menu is open, this will
    /// have the player's inventory.
    pub fn menu(&self) -> Menu {
        let mut ecs = self.ecs.lock();
        let inventory = self.query::<&InventoryComponent>(&mut ecs);
        inventory.menu().clone()
    }
}

/// A handle to the open container. The container will be closed once this is
/// dropped.
pub struct ContainerHandle {
    pub id: i8,
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
    /// Returns the menu of the container. If the container is closed, this
    /// will return `None`.
    pub fn menu(&self) -> Option<Menu> {
        let ecs = self.client.ecs.lock();
        let inventory = ecs
            .get::<InventoryComponent>(self.client.entity)
            .expect("no inventory");
        if inventory.id == self.id {
            Some(inventory.container_menu.clone().unwrap())
        } else {
            None
        }
    }

    /// Returns the item slots in the container. If the container is closed,
    /// this will return `None`.
    pub fn contents(&self) -> Option<Vec<ItemSlot>> {
        self.menu().map(|menu| menu.contents())
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
    pub id: i8,
    /// The current container menu that the player has open. If no container is
    /// open, this will be `None`.
    pub container_menu: Option<azalea_inventory::Menu>,
    /// The item that is currently held by the cursor. `Slot::Empty` if nothing
    /// is currently being held.
    pub carried: ItemSlot,
    /// An identifier used by the server to track client inventory desyncs.
    pub state_id: u32,
    // minecraft also has these fields, but i don't think they're necessary?:
    // private final NonNullList<ItemStack> remoteSlots;
    // private final IntList remoteDataSlots;
    // private ItemStack remoteCarried;
}
impl InventoryComponent {
    /// Returns the currently active menu. If a container is open it'll return
    /// [`Self::container_menu`], otherwise [`Self::inventory_menu`].
    pub fn menu(&self) -> &azalea_inventory::Menu {
        if let Some(menu) = &self.container_menu {
            menu
        } else {
            &self.inventory_menu
        }
    }

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
    mut commands: Commands,
    mut events: EventReader<MenuOpenedEvent>,
    mut query: Query<&mut InventoryComponent>,
) {
    for event in events.iter() {
        commands
            .entity(event.entity)
            .remove::<WaitingForInventoryOpen>();

        let mut inventory = query.get_mut(event.entity).unwrap();
        inventory.id = event.window_id as i8;
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
    pub id: i8,
}
fn handle_container_close_event(
    mut events: EventReader<CloseContainerEvent>,
    mut client_side_events: EventWriter<ClientSideCloseContainerEvent>,
    query: Query<(&LocalPlayer, &InventoryComponent)>,
) {
    for event in events.iter() {
        let (local_player, inventory) = query.get(event.entity).unwrap();
        if event.id != inventory.id {
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
