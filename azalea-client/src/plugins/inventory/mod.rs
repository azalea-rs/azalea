pub mod enchantents;

use azalea_chat::FormattedText;
use azalea_core::tick::GameTick;
use azalea_entity::{PlayerAbilities, inventory::Inventory};
use azalea_inventory::operations::ClickOperation;
pub use azalea_inventory::*;
use azalea_protocol::packets::game::{
    s_container_click::{HashedStack, ServerboundContainerClick},
    s_container_close::ServerboundContainerClose,
    s_set_carried_item::ServerboundSetCarriedItem,
};
use azalea_registry::MenuKind;
use azalea_world::{InstanceContainer, InstanceName};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use indexmap::IndexMap;
use tracing::{error, warn};

use crate::{
    Client, inventory::enchantents::update_attributes_for_enchantments,
    packet::game::SendPacketEvent, respawn::perform_respawn,
};

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ClientSideCloseContainerEvent>()
            .add_event::<MenuOpenedEvent>()
            .add_event::<CloseContainerEvent>()
            .add_event::<ContainerClickEvent>()
            .add_event::<SetContainerContentEvent>()
            .add_event::<SetSelectedHotbarSlotEvent>()
            .add_systems(
                Update,
                (
                    (
                        handle_set_selected_hotbar_slot_event,
                        handle_menu_opened_event,
                        handle_set_container_content_event,
                        handle_container_click_event,
                        handle_container_close_event,
                        handle_client_side_close_container_event,
                    )
                        .chain(),
                    update_attributes_for_enchantments,
                )
                    .in_set(InventorySet)
                    .before(perform_respawn),
            )
            .add_systems(
                GameTick,
                ensure_has_sent_carried_item.after(super::mining::handle_mining_queued),
            );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct InventorySet;

impl Client {
    /// Return the menu that is currently open. If no menu is open, this will
    /// have the player's inventory.
    pub fn menu(&self) -> Menu {
        let mut ecs = self.ecs.lock();
        let inventory = self.query::<&Inventory>(&mut ecs);
        inventory.menu().clone()
    }

    /// Returns the index of the hotbar slot that's currently selected.
    ///
    /// If you want to access the actual held item, you can get the current menu
    /// with [`Client::menu`] and then get the slot index by offsetting from
    /// the start of [`azalea_inventory::Menu::hotbar_slots_range`].
    ///
    /// You can use [`Self::set_selected_hotbar_slot`] to change it.
    pub fn selected_hotbar_slot(&self) -> u8 {
        let mut ecs = self.ecs.lock();
        let inventory = self.query::<&Inventory>(&mut ecs);
        inventory.selected_hotbar_slot
    }

    /// Update the selected hotbar slot index.
    ///
    /// This will run next `Update`, so you might want to call
    /// `bot.wait_updates(1)` after calling this if you're using `azalea`.
    pub fn set_selected_hotbar_slot(&self, new_hotbar_slot_index: u8) {
        assert!(
            new_hotbar_slot_index < 9,
            "Hotbar slot index must be in the range 0..=8"
        );

        let mut ecs = self.ecs.lock();
        ecs.send_event(SetSelectedHotbarSlotEvent {
            entity: self.entity,
            slot: new_hotbar_slot_index,
        });
    }
}

/// Sent from the server when a menu (like a chest or crafting table) was
/// opened by the client.
#[derive(Event, Debug)]
pub struct MenuOpenedEvent {
    pub entity: Entity,
    pub window_id: i32,
    pub menu_type: MenuKind,
    pub title: FormattedText,
}
fn handle_menu_opened_event(
    mut events: EventReader<MenuOpenedEvent>,
    mut query: Query<&mut Inventory>,
) {
    for event in events.read() {
        let mut inventory = query.get_mut(event.entity).unwrap();
        inventory.id = event.window_id;
        inventory.container_menu = Some(Menu::from_kind(event.menu_type));
        inventory.container_menu_title = Some(event.title.clone());
    }
}

/// Tell the server that we want to close a container.
///
/// Note that this is also sent when the client closes its own inventory, even
/// though there is no packet for opening its inventory.
#[derive(Event)]
pub struct CloseContainerEvent {
    pub entity: Entity,
    /// The ID of the container to close. 0 for the player's inventory. If this
    /// is not the same as the currently open inventory, nothing will happen.
    pub id: i32,
}
fn handle_container_close_event(
    query: Query<(Entity, &Inventory)>,
    mut events: EventReader<CloseContainerEvent>,
    mut client_side_events: EventWriter<ClientSideCloseContainerEvent>,
    mut commands: Commands,
) {
    for event in events.read() {
        let (entity, inventory) = query.get(event.entity).unwrap();
        if event.id != inventory.id {
            warn!(
                "Tried to close container with ID {}, but the current container ID is {}",
                event.id, inventory.id
            );
            continue;
        }

        commands.trigger(SendPacketEvent::new(
            entity,
            ServerboundContainerClose {
                container_id: inventory.id,
            },
        ));
        client_side_events.write(ClientSideCloseContainerEvent {
            entity: event.entity,
        });
    }
}

/// Close a container without notifying the server.
///
/// Note that this also gets fired when we get a [`CloseContainerEvent`].
#[derive(Event)]
pub struct ClientSideCloseContainerEvent {
    pub entity: Entity,
}
pub fn handle_client_side_close_container_event(
    mut events: EventReader<ClientSideCloseContainerEvent>,
    mut query: Query<&mut Inventory>,
) {
    for event in events.read() {
        let mut inventory = query.get_mut(event.entity).unwrap();

        // copy the Player part of the container_menu to the inventory_menu
        if let Some(inventory_menu) = inventory.container_menu.take() {
            // this isn't the same as what vanilla does. i believe vanilla synchronizes the
            // slots between inventoryMenu and containerMenu by just having the player slots
            // point to the same ItemStack in memory, but emulating this in rust would
            // require us to wrap our `ItemStack`s as `Arc<Mutex<ItemStack>>` which would
            // have kinda terrible ergonomics.

            // the simpler solution i chose to go with here is to only copy the player slots
            // when the container is closed. this is perfectly fine for vanilla, but it
            // might cause issues if a server modifies id 0 while we have a container
            // open...

            // if we do encounter this issue in the wild then the simplest solution would
            // probably be to just add logic for updating the container_menu when the server
            // tries to modify id 0 for slots within `inventory`. not implemented for now
            // because i'm not sure if that's worth worrying about.

            let new_inventory =
                inventory_menu.slots()[inventory_menu.player_slots_range()].to_vec();
            let new_inventory = <[ItemStack; 36]>::try_from(new_inventory).unwrap();
            *inventory.inventory_menu.as_player_mut().inventory = new_inventory;
        }

        inventory.id = 0;
        inventory.container_menu_title = None;
    }
}

#[derive(Event, Debug)]
pub struct ContainerClickEvent {
    pub entity: Entity,
    pub window_id: i32,
    pub operation: ClickOperation,
}
pub fn handle_container_click_event(
    mut query: Query<(
        Entity,
        &mut Inventory,
        Option<&PlayerAbilities>,
        &InstanceName,
    )>,
    mut events: EventReader<ContainerClickEvent>,
    mut commands: Commands,
    instance_container: Res<InstanceContainer>,
) {
    for event in events.read() {
        let (entity, mut inventory, player_abilities, instance_name) =
            query.get_mut(event.entity).unwrap();
        if inventory.id != event.window_id {
            error!(
                "Tried to click container with ID {}, but the current container ID is {}. Click packet won't be sent.",
                event.window_id, inventory.id
            );
            continue;
        }

        let Some(instance) = instance_container.get(instance_name) else {
            continue;
        };

        let old_slots = inventory.menu().slots();
        inventory.simulate_click(
            &event.operation,
            player_abilities.unwrap_or(&PlayerAbilities::default()),
        );
        let new_slots = inventory.menu().slots();

        let registry_holder = &instance.read().registries;

        // see which slots changed after clicking and put them in the map the server
        // uses this to check if we desynced
        let mut changed_slots: IndexMap<u16, HashedStack> = IndexMap::new();
        for (slot_index, old_slot) in old_slots.iter().enumerate() {
            let new_slot = &new_slots[slot_index];
            if old_slot != new_slot {
                changed_slots.insert(
                    slot_index as u16,
                    HashedStack::from_item_stack(new_slot, registry_holder),
                );
            }
        }

        commands.trigger(SendPacketEvent::new(
            entity,
            ServerboundContainerClick {
                container_id: event.window_id,
                state_id: inventory.state_id,
                slot_num: event.operation.slot_num().map(|n| n as i16).unwrap_or(-999),
                button_num: event.operation.button_num(),
                click_type: event.operation.click_type(),
                changed_slots,
                carried_item: HashedStack::from_item_stack(&inventory.carried, registry_holder),
            },
        ));
    }
}

/// Sent from the server when the contents of a container are replaced. Usually
/// triggered by the `ContainerSetContent` packet.
#[derive(Event)]
pub struct SetContainerContentEvent {
    pub entity: Entity,
    pub slots: Vec<ItemStack>,
    pub container_id: i32,
}
fn handle_set_container_content_event(
    mut events: EventReader<SetContainerContentEvent>,
    mut query: Query<&mut Inventory>,
) {
    for event in events.read() {
        let mut inventory = query.get_mut(event.entity).unwrap();

        if event.container_id != inventory.id {
            warn!(
                "Got SetContainerContentEvent for container with ID {}, but the current container ID is {}",
                event.container_id, inventory.id
            );
            continue;
        }

        let menu = inventory.menu_mut();
        for (i, slot) in event.slots.iter().enumerate() {
            if let Some(slot_mut) = menu.slot_mut(i) {
                *slot_mut = slot.clone();
            }
        }
    }
}

/// An ECS event to switch our hand to a different hotbar slot.
///
/// This is equivalent to using the scroll wheel or number keys in Minecraft.
#[derive(Event)]
pub struct SetSelectedHotbarSlotEvent {
    pub entity: Entity,
    /// The hotbar slot to select. This should be in the range 0..=8.
    pub slot: u8,
}
pub fn handle_set_selected_hotbar_slot_event(
    mut events: EventReader<SetSelectedHotbarSlotEvent>,
    mut query: Query<&mut Inventory>,
) {
    for event in events.read() {
        let mut inventory = query.get_mut(event.entity).unwrap();

        // if the slot is already selected, don't send a packet
        if inventory.selected_hotbar_slot == event.slot {
            continue;
        }

        inventory.selected_hotbar_slot = event.slot;
    }
}

/// The item slot that the server thinks we have selected.
///
/// See [`ensure_has_sent_carried_item`].
#[derive(Component)]
pub struct LastSentSelectedHotbarSlot {
    pub slot: u8,
}
/// A system that makes sure that [`LastSentSelectedHotbarSlot`] is in sync with
/// [`Inventory::selected_hotbar_slot`].
///
/// This is necessary to make sure that [`ServerboundSetCarriedItem`] is sent in
/// the right order, since it's not allowed to happen outside of a tick.
pub fn ensure_has_sent_carried_item(
    mut commands: Commands,
    query: Query<(Entity, &Inventory, Option<&LastSentSelectedHotbarSlot>)>,
) {
    for (entity, inventory, last_sent) in query.iter() {
        if let Some(last_sent) = last_sent {
            if last_sent.slot == inventory.selected_hotbar_slot {
                continue;
            }

            commands.trigger(SendPacketEvent::new(
                entity,
                ServerboundSetCarriedItem {
                    slot: inventory.selected_hotbar_slot as u16,
                },
            ));
        }

        commands.entity(entity).insert(LastSentSelectedHotbarSlot {
            slot: inventory.selected_hotbar_slot,
        });
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use azalea_inventory::operations::{QuickCraftKind, QuickCraftStatusKind, QuickMoveClick};
    use azalea_registry::Item;

    use super::*;

    #[test]
    fn test_simulate_shift_click_in_crafting_table() {
        let spruce_planks = ItemStack::new(Item::SprucePlanks, 4);

        let mut inventory = Inventory {
            inventory_menu: Menu::Player(azalea_inventory::Player::default()),
            id: 1,
            container_menu: Some(Menu::Crafting {
                result: spruce_planks.clone(),
                // simulate_click won't delete the items from here
                grid: SlotList::default(),
                player: SlotList::default(),
            }),
            container_menu_title: None,
            carried: ItemStack::Empty,
            state_id: 0,
            quick_craft_status: QuickCraftStatusKind::Start,
            quick_craft_kind: QuickCraftKind::Middle,
            quick_craft_slots: HashSet::new(),
            selected_hotbar_slot: 0,
        };

        inventory.simulate_click(
            &ClickOperation::QuickMove(QuickMoveClick::Left { slot: 0 }),
            &PlayerAbilities::default(),
        );

        let new_slots = inventory.menu().slots();
        assert_eq!(&new_slots[0], &ItemStack::Empty);
        assert_eq!(
            &new_slots[*Menu::CRAFTING_PLAYER_SLOTS.start()],
            &spruce_planks
        );
    }
}
