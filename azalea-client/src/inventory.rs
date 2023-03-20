use std::collections::{HashMap, HashSet};

use azalea_chat::FormattedText;
use azalea_core::GameMode;
pub use azalea_inventory::*;
use azalea_inventory::{
    item::MaxStackSizeExt,
    operations::{
        ClickOperation, PickupClick, QuickCraftClick, QuickCraftKind, QuickCraftStatus,
        QuickCraftStatusKind,
    },
};
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

use crate::{client::PlayerAbilities, local_player::handle_send_packet_event, Client, LocalPlayer};

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

    pub quick_craft_status: QuickCraftStatusKind,
    pub quick_craft_kind: QuickCraftKind,
    pub quick_craft_slots: HashSet<u16>,
    // minecraft also has these fields, but i don't
    // think they're necessary?:
    // private final NonNullList<ItemStack>
    // remoteSlots;
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

    /// Modify the inventory as if the given operation was performed on it.
    pub fn simulate_click(
        &mut self,
        operation: &ClickOperation,
        player_abilities: &PlayerAbilities,
    ) {
        match operation {
            ClickOperation::QuickCraft(quick_craft) => {
                let last_quick_craft_status_tmp = self.quick_craft_status.clone();
                self.quick_craft_status = last_quick_craft_status_tmp.clone();
                let last_quick_craft_status = last_quick_craft_status_tmp.into();

                // no carried item, reset
                if self.carried.is_empty() {
                    return self.reset_quick_craft();
                }
                // if we were starting or ending, or now we aren't ending and the status
                // changed, reset
                if (last_quick_craft_status == QuickCraftStatusKind::Start
                    || last_quick_craft_status == QuickCraftStatusKind::End
                    || self.quick_craft_status != QuickCraftStatusKind::End)
                    && (self.quick_craft_status != last_quick_craft_status)
                {
                    return self.reset_quick_craft();
                }
                if self.quick_craft_status == QuickCraftStatusKind::Start {
                    self.quick_craft_kind = quick_craft.kind.clone();
                    if self.quick_craft_kind == QuickCraftKind::Middle
                        && player_abilities.instant_break
                    {
                        self.quick_craft_status = QuickCraftStatusKind::Add;
                        self.quick_craft_slots.clear();
                    } else {
                        self.reset_quick_craft();
                    }
                    return;
                }
                if let QuickCraftStatus::Add { slot } = quick_craft.status {
                    let slot_item = self.menu().slot(slot as usize);
                    if let Some(slot_item) = slot_item {
                        if let ItemSlot::Present(carried) = &self.carried {
                            // minecraft also checks slot.may_place(carried) and
                            // menu.can_drag_to(slot)
                            // but they always return true so they're not relevant for us
                            if Self::can_item_quick_replace(slot_item, carried, true)
                                && (self.quick_craft_kind == QuickCraftKind::Right
                                    || carried.count as usize > self.quick_craft_slots.len())
                            {
                                self.quick_craft_slots.insert(slot);
                            }
                        }
                    }
                    return;
                }
                if self.quick_craft_status == QuickCraftStatusKind::End {
                    if !self.quick_craft_slots.is_empty() {
                        if self.quick_craft_slots.len() == 1 {
                            let slot = *self.quick_craft_slots.iter().next().unwrap();
                            self.reset_quick_craft();
                            self.simulate_click(
                                &match self.quick_craft_kind {
                                    QuickCraftKind::Left => PickupClick::Left { slot }.into(),
                                    QuickCraftKind::Right => PickupClick::Left { slot }.into(),
                                    QuickCraftKind::Middle => {
                                        // idk just do nothing i guess
                                        return;
                                    }
                                },
                                player_abilities,
                            );
                            return;
                        }

                        let ItemSlot::Present(mut carried) = self.carried.clone() else {
                            // this should never happen
                            return self.reset_quick_craft();
                        };
                        let mut carried_count = carried.count;
                        let mut quick_craft_slots_iter = self.quick_craft_slots.iter();

                        loop {
                            let mut slot: &ItemSlot;
                            let mut item_stack: &ItemSlot;

                            loop {
                                loop {
                                    loop {
                                        loop {
                                            let Some(next_slot) = quick_craft_slots_iter.next() else {
                                                carried.count = carried_count;
                                                self.carried = ItemSlot::Present(carried.clone());
                                                return self.reset_quick_craft();
                                            };

                                            slot = self.menu().slot(*next_slot as usize).unwrap();
                                            item_stack = &self.carried;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            ClickOperation::Pickup(_) => todo!(),
            ClickOperation::QuickMove(_) => todo!(),
            ClickOperation::Swap(_) => todo!(),
            ClickOperation::Clone(_) => todo!(),
            ClickOperation::Throw(_) => todo!(),
            ClickOperation::PickupAll(_) => todo!(),
        }
    }

    fn reset_quick_craft(&mut self) {
        self.quick_craft_status = QuickCraftStatusKind::Start;
        self.quick_craft_slots.clear();
    }

    fn can_item_quick_replace(
        target_slot: &ItemSlot,
        item: &ItemSlotData,
        ignore_item_count: bool,
    ) -> bool {
        let ItemSlot::Present(target_slot) = target_slot else {
            return false;
        };
        if item != target_slot {
            return false;
        }
        let count = target_slot.count as u16
            + if ignore_item_count {
                0
            } else {
                item.count as u16
            };
        count <= item.kind.max_stack_size() as u16
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
            quick_craft_status: QuickCraftStatusKind::Start,
            quick_craft_kind: QuickCraftKind::Middle,
            quick_craft_slots: HashSet::new(),
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
