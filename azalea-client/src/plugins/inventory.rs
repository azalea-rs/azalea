use std::{cmp, collections::HashSet};

use azalea_chat::FormattedText;
use azalea_core::tick::GameTick;
use azalea_entity::PlayerAbilities;
pub use azalea_inventory::*;
use azalea_inventory::{
    item::MaxStackSizeExt,
    operations::{
        ClickOperation, CloneClick, PickupAllClick, PickupClick, QuickCraftKind, QuickCraftStatus,
        QuickCraftStatusKind, QuickMoveClick, ThrowClick,
    },
};
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

use crate::{Client, packet::game::SendGamePacketEvent, respawn::perform_respawn};

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SetSelectedHotbarSlotEvent>()
            .add_systems(
                Update,
                handle_set_selected_hotbar_slot_event
                    .in_set(InventorySet)
                    .before(perform_respawn),
            )
            .add_systems(
                GameTick,
                ensure_has_sent_carried_item.after(super::mining::handle_mining_queued),
            )
            .add_observer(handle_client_side_close_container_trigger)
            .add_observer(handle_menu_opened_trigger)
            .add_observer(handle_container_close_event)
            .add_observer(handle_set_container_content_trigger);
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct InventorySet;

impl Client {
    /// Return the menu that is currently open. If no menu is open, this will
    /// have the player's inventory.
    pub fn menu(&self) -> Menu {
        self.query_self::<&Inventory, _>(|inv| inv.menu().clone())
    }

    /// Returns the index of the hotbar slot that's currently selected.
    ///
    /// If you want to access the actual held item, you can get the current menu
    /// with [`Client::menu`] and then get the slot index by offsetting from
    /// the start of [`azalea_inventory::Menu::hotbar_slots_range`].
    ///
    /// You can use [`Self::set_selected_hotbar_slot`] to change it.
    pub fn selected_hotbar_slot(&self) -> u8 {
        self.query_self::<&Inventory, _>(|inv| inv.selected_hotbar_slot)
    }

    /// Update the selected hotbar slot index.
    ///
    /// This will run next `Update`, so you might want to call
    /// `bot.wait_updates(1)` after calling this if you're using `azalea`.
    ///
    /// # Panics
    ///
    /// This will panic if `new_hotbar_slot_index` is not in the range 0..=8.
    pub fn set_selected_hotbar_slot(&self, new_hotbar_slot_index: u8) {
        assert!(
            new_hotbar_slot_index < 9,
            "Hotbar slot index must be in the range 0..=8"
        );

        let mut ecs = self.ecs.lock();
        ecs.write_message(SetSelectedHotbarSlotEvent {
            entity: self.entity,
            slot: new_hotbar_slot_index,
        });
    }
}

/// A component present on all local players that have an inventory.
#[derive(Component, Debug, Clone)]
pub struct Inventory {
    /// The player's inventory menu. This is guaranteed to be a `Menu::Player`.
    ///
    /// We keep it as a [`Menu`] since `Menu` has some useful functions that
    /// bare [`azalea_inventory::Player`] doesn't have.
    pub inventory_menu: azalea_inventory::Menu,

    /// The ID of the container that's currently open. Its value is not
    /// guaranteed to be anything specific, and may change every time you open a
    /// container (unless it's 0, in which case it means that no container is
    /// open).
    pub id: i32,
    /// The current container menu that the player has open. If no container is
    /// open, this will be `None`.
    pub container_menu: Option<azalea_inventory::Menu>,
    /// The custom name of the menu that's currently open. This is Some when
    /// `container_menu` is Some.
    pub container_menu_title: Option<FormattedText>,
    /// The item that is currently held by the cursor. `Slot::Empty` if nothing
    /// is currently being held.
    ///
    /// This is different from [`Self::selected_hotbar_slot`], which is the
    /// item that's selected in the hotbar.
    pub carried: ItemStack,
    /// An identifier used by the server to track client inventory desyncs. This
    /// is sent on every container click, and it's only ever updated when the
    /// server sends a new container update.
    pub state_id: u32,

    pub quick_craft_status: QuickCraftStatusKind,
    pub quick_craft_kind: QuickCraftKind,
    /// A set of the indexes of the slots that have been right clicked in
    /// this "quick craft".
    pub quick_craft_slots: HashSet<u16>,

    /// The index of the item in the hotbar that's currently being held by the
    /// player. This MUST be in the range 0..9 (not including 9).
    ///
    /// In a vanilla client this is changed by pressing the number keys or using
    /// the scroll wheel.
    pub selected_hotbar_slot: u8,
}

impl Inventory {
    /// Returns a reference to the currently active menu. If a container is open
    /// it'll return [`Self::container_menu`], otherwise
    /// [`Self::inventory_menu`].
    ///
    /// Use [`Self::menu_mut`] if you need a mutable reference.
    pub fn menu(&self) -> &azalea_inventory::Menu {
        match &self.container_menu {
            Some(menu) => menu,
            _ => &self.inventory_menu,
        }
    }

    /// Returns a mutable reference to the currently active menu. If a container
    /// is open it'll return [`Self::container_menu`], otherwise
    /// [`Self::inventory_menu`].
    ///
    /// Use [`Self::menu`] if you don't need a mutable reference.
    pub fn menu_mut(&mut self) -> &mut azalea_inventory::Menu {
        match &mut self.container_menu {
            Some(menu) => menu,
            _ => &mut self.inventory_menu,
        }
    }

    /// Modify the inventory as if the given operation was performed on it.
    pub fn simulate_click(
        &mut self,
        operation: &ClickOperation,
        player_abilities: &PlayerAbilities,
    ) {
        if let ClickOperation::QuickCraft(quick_craft) = operation {
            let last_quick_craft_status_tmp = self.quick_craft_status.clone();
            self.quick_craft_status = last_quick_craft_status_tmp.clone();
            let last_quick_craft_status = last_quick_craft_status_tmp;

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
                if self.quick_craft_kind == QuickCraftKind::Middle && player_abilities.instant_break
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
                if let Some(slot_item) = slot_item
                    && let ItemStack::Present(carried) = &self.carried
                {
                    // minecraft also checks slot.may_place(carried) and
                    // menu.can_drag_to(slot)
                    // but they always return true so they're not relevant for us
                    if can_item_quick_replace(slot_item, &self.carried, true)
                        && (self.quick_craft_kind == QuickCraftKind::Right
                            || carried.count as usize > self.quick_craft_slots.len())
                    {
                        self.quick_craft_slots.insert(slot);
                    }
                }
                return;
            }
            if self.quick_craft_status == QuickCraftStatusKind::End {
                if !self.quick_craft_slots.is_empty() {
                    if self.quick_craft_slots.len() == 1 {
                        // if we only clicked one slot, then turn this
                        // QuickCraftClick into a PickupClick
                        let slot = *self.quick_craft_slots.iter().next().unwrap();
                        self.reset_quick_craft();
                        self.simulate_click(
                            &match self.quick_craft_kind {
                                QuickCraftKind::Left => {
                                    PickupClick::Left { slot: Some(slot) }.into()
                                }
                                QuickCraftKind::Right => {
                                    PickupClick::Left { slot: Some(slot) }.into()
                                }
                                QuickCraftKind::Middle => {
                                    // idk just do nothing i guess
                                    return;
                                }
                            },
                            player_abilities,
                        );
                        return;
                    }

                    let ItemStack::Present(mut carried) = self.carried.clone() else {
                        // this should never happen
                        return self.reset_quick_craft();
                    };

                    let mut carried_count = carried.count;
                    let mut quick_craft_slots_iter = self.quick_craft_slots.iter();

                    loop {
                        let mut slot: &ItemStack;
                        let mut slot_index: u16;
                        let mut item_stack: &ItemStack;

                        loop {
                            let Some(&next_slot) = quick_craft_slots_iter.next() else {
                                carried.count = carried_count;
                                self.carried = ItemStack::Present(carried);
                                return self.reset_quick_craft();
                            };

                            slot = self.menu().slot(next_slot as usize).unwrap();
                            slot_index = next_slot;
                            item_stack = &self.carried;

                            if slot.is_present()
                                    && can_item_quick_replace(slot, item_stack, true)
                                    // this always returns true in most cases
                                    // && slot.may_place(item_stack)
                                    && (
                                        self.quick_craft_kind == QuickCraftKind::Middle
                                        || item_stack.count()  >= self.quick_craft_slots.len() as i32
                                    )
                            {
                                break;
                            }
                        }

                        // get the ItemStackData for the slot
                        let ItemStack::Present(slot) = slot else {
                            unreachable!("the loop above requires the slot to be present to break")
                        };

                        // if self.can_drag_to(slot) {
                        let mut new_carried = carried.clone();
                        let slot_item_count = slot.count;
                        get_quick_craft_slot_count(
                            &self.quick_craft_slots,
                            &self.quick_craft_kind,
                            &mut new_carried,
                            slot_item_count,
                        );
                        let max_stack_size = i32::min(
                            new_carried.kind.max_stack_size(),
                            i32::min(
                                new_carried.kind.max_stack_size(),
                                slot.kind.max_stack_size(),
                            ),
                        );
                        if new_carried.count > max_stack_size {
                            new_carried.count = max_stack_size;
                        }

                        carried_count -= new_carried.count - slot_item_count;
                        // we have to inline self.menu_mut() here to avoid the borrow checker
                        // complaining
                        let menu = match &mut self.container_menu {
                            Some(menu) => menu,
                            _ => &mut self.inventory_menu,
                        };
                        *menu.slot_mut(slot_index as usize).unwrap() =
                            ItemStack::Present(new_carried);
                    }
                }
            } else {
                return self.reset_quick_craft();
            }
        }
        // the quick craft status should always be in start if we're not in quick craft
        // mode
        if self.quick_craft_status != QuickCraftStatusKind::Start {
            return self.reset_quick_craft();
        }

        match operation {
            // left clicking outside inventory
            ClickOperation::Pickup(PickupClick::Left { slot: None }) => {
                if self.carried.is_present() {
                    // vanilla has `player.drop`s but they're only used
                    // server-side
                    // they're included as comments here in case you want to adapt this for a server
                    // implementation

                    // player.drop(self.carried, true);
                    self.carried = ItemStack::Empty;
                }
            }
            ClickOperation::Pickup(PickupClick::Right { slot: None }) => {
                if self.carried.is_present() {
                    let _item = self.carried.split(1);
                    // player.drop(item, true);
                }
            }
            &ClickOperation::Pickup(
                // lol
                ref pickup @ (PickupClick::Left { slot: Some(slot) }
                | PickupClick::Right { slot: Some(slot) }),
            ) => {
                let slot = slot as usize;
                let Some(slot_item) = self.menu().slot(slot) else {
                    return;
                };

                if self.try_item_click_behavior_override(operation, slot) {
                    return;
                }

                let is_left_click = matches!(pickup, PickupClick::Left { .. });

                match slot_item {
                    ItemStack::Empty => {
                        if self.carried.is_present() {
                            let place_count = if is_left_click {
                                self.carried.count()
                            } else {
                                1
                            };
                            self.carried =
                                self.safe_insert(slot, self.carried.clone(), place_count);
                        }
                    }
                    ItemStack::Present(_) => {
                        if !self.menu().may_pickup(slot) {
                            return;
                        }
                        if let ItemStack::Present(carried) = self.carried.clone() {
                            let slot_is_same_item_as_carried = slot_item
                                .as_present()
                                .is_some_and(|s| carried.is_same_item_and_components(s));

                            if self.menu().may_place(slot, &carried) {
                                if slot_is_same_item_as_carried {
                                    let place_count = if is_left_click { carried.count } else { 1 };
                                    self.carried =
                                        self.safe_insert(slot, self.carried.clone(), place_count);
                                } else if carried.count
                                    <= self
                                        .menu()
                                        .max_stack_size(slot)
                                        .min(carried.kind.max_stack_size())
                                {
                                    // swap slot_item and carried
                                    self.carried = slot_item.clone();
                                    let slot_item = self.menu_mut().slot_mut(slot).unwrap();
                                    *slot_item = carried.into();
                                }
                            } else if slot_is_same_item_as_carried
                                && let Some(removed) = self.try_remove(
                                    slot,
                                    slot_item.count(),
                                    carried.kind.max_stack_size() - carried.count,
                                )
                            {
                                self.carried.as_present_mut().unwrap().count += removed.count();
                                // slot.onTake(player, removed);
                            }
                        } else {
                            let pickup_count = if is_left_click {
                                slot_item.count()
                            } else {
                                (slot_item.count() + 1) / 2
                            };
                            if let Some(new_slot_item) =
                                self.try_remove(slot, pickup_count, i32::MAX)
                            {
                                self.carried = new_slot_item;
                                // slot.onTake(player, newSlot);
                            }
                        }
                    }
                }
            }
            &ClickOperation::QuickMove(
                QuickMoveClick::Left { slot } | QuickMoveClick::Right { slot },
            ) => {
                // in vanilla it also tests if QuickMove has a slot index of -999
                // but i don't think that's ever possible so it's not covered here
                let slot = slot as usize;
                loop {
                    let new_slot_item = self.menu_mut().quick_move_stack(slot);
                    let slot_item = self.menu().slot(slot).unwrap();
                    if new_slot_item.is_empty() || slot_item.kind() != new_slot_item.kind() {
                        break;
                    }
                }
            }
            ClickOperation::Swap(s) => {
                let source_slot_index = s.source_slot as usize;
                let target_slot_index = s.target_slot as usize;

                let Some(source_slot) = self.menu().slot(source_slot_index) else {
                    return;
                };
                let Some(target_slot) = self.menu().slot(target_slot_index) else {
                    return;
                };
                if source_slot.is_empty() && target_slot.is_empty() {
                    return;
                }

                if target_slot.is_empty() {
                    if self.menu().may_pickup(source_slot_index) {
                        let source_slot = source_slot.clone();
                        let target_slot = self.menu_mut().slot_mut(target_slot_index).unwrap();
                        *target_slot = source_slot;
                    }
                } else if source_slot.is_empty() {
                    let target_item = target_slot
                        .as_present()
                        .expect("target slot was already checked to not be empty");
                    if self.menu().may_place(source_slot_index, target_item) {
                        // get the target_item but mutable
                        let source_max_stack_size = self.menu().max_stack_size(source_slot_index);

                        let target_slot = self.menu_mut().slot_mut(target_slot_index).unwrap();
                        let new_source_slot =
                            target_slot.split(source_max_stack_size.try_into().unwrap());
                        *self.menu_mut().slot_mut(source_slot_index).unwrap() = new_source_slot;
                    }
                } else if self.menu().may_pickup(source_slot_index) {
                    let ItemStack::Present(target_item) = target_slot else {
                        unreachable!("target slot is not empty but is not present");
                    };
                    if self.menu().may_place(source_slot_index, target_item) {
                        let source_max_stack = self.menu().max_stack_size(source_slot_index);
                        if target_slot.count() > source_max_stack {
                            // if there's more than the max stack size in the target slot

                            let target_slot = self.menu_mut().slot_mut(target_slot_index).unwrap();
                            let new_source_slot =
                                target_slot.split(source_max_stack.try_into().unwrap());
                            *self.menu_mut().slot_mut(source_slot_index).unwrap() = new_source_slot;
                            // if !self.inventory_menu.add(new_source_slot) {
                            //     player.drop(new_source_slot, true);
                            // }
                        } else {
                            // normal swap
                            let new_target_slot = source_slot.clone();
                            let new_source_slot = target_slot.clone();

                            let target_slot = self.menu_mut().slot_mut(target_slot_index).unwrap();
                            *target_slot = new_target_slot;

                            let source_slot = self.menu_mut().slot_mut(source_slot_index).unwrap();
                            *source_slot = new_source_slot;
                        }
                    }
                }
            }
            ClickOperation::Clone(CloneClick { slot }) => {
                if !player_abilities.instant_break || self.carried.is_present() {
                    return;
                }
                let Some(source_slot) = self.menu().slot(*slot as usize) else {
                    return;
                };
                let ItemStack::Present(source_item) = source_slot else {
                    return;
                };
                let mut new_carried = source_item.clone();
                new_carried.count = new_carried.kind.max_stack_size();
                self.carried = ItemStack::Present(new_carried);
            }
            ClickOperation::Throw(c) => {
                if self.carried.is_present() {
                    return;
                }

                let (ThrowClick::Single { slot: slot_index }
                | ThrowClick::All { slot: slot_index }) = c;
                let slot_index = *slot_index as usize;

                let Some(slot) = self.menu_mut().slot_mut(slot_index) else {
                    return;
                };
                let ItemStack::Present(slot_item) = slot else {
                    return;
                };

                let dropping_count = match c {
                    ThrowClick::Single { .. } => 1,
                    ThrowClick::All { .. } => slot_item.count,
                };

                let _dropping = slot_item.split(dropping_count as u32);
                // player.drop(dropping, true);
            }
            ClickOperation::PickupAll(PickupAllClick {
                slot: source_slot_index,
                reversed,
            }) => {
                let source_slot_index = *source_slot_index as usize;

                let source_slot = self.menu().slot(source_slot_index).unwrap();
                let target_slot = self.carried.clone();

                if target_slot.is_empty()
                    || (source_slot.is_present() && self.menu().may_pickup(source_slot_index))
                {
                    return;
                }

                let ItemStack::Present(target_slot_item) = &target_slot else {
                    unreachable!("target slot is not empty but is not present");
                };

                for round in 0..2 {
                    let iterator: Box<dyn Iterator<Item = usize>> = if *reversed {
                        Box::new((0..self.menu().len()).rev())
                    } else {
                        Box::new(0..self.menu().len())
                    };

                    for i in iterator {
                        if target_slot_item.count < target_slot_item.kind.max_stack_size() {
                            let checking_slot = self.menu().slot(i).unwrap();
                            if let ItemStack::Present(checking_item) = checking_slot
                                && can_item_quick_replace(checking_slot, &target_slot, true)
                                && self.menu().may_pickup(i)
                                && (round != 0
                                    || checking_item.count != checking_item.kind.max_stack_size())
                            {
                                // get the checking_slot and checking_item again but mutable
                                let checking_slot = self.menu_mut().slot_mut(i).unwrap();

                                let taken_item = checking_slot.split(checking_slot.count() as u32);

                                // now extend the carried item
                                let target_slot = &mut self.carried;
                                let ItemStack::Present(target_slot_item) = target_slot else {
                                    unreachable!("target slot is not empty but is not present");
                                };
                                target_slot_item.count += taken_item.count();
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn reset_quick_craft(&mut self) {
        self.quick_craft_status = QuickCraftStatusKind::Start;
        self.quick_craft_slots.clear();
    }

    /// Get the item in the player's hotbar that is currently being held in its
    /// main hand.
    pub fn held_item(&self) -> ItemStack {
        let inventory = &self.inventory_menu;
        let hotbar_items = &inventory.slots()[inventory.hotbar_slots_range()];
        hotbar_items[self.selected_hotbar_slot as usize].clone()
    }

    /// TODO: implement bundles
    fn try_item_click_behavior_override(
        &self,
        _operation: &ClickOperation,
        _slot_item_index: usize,
    ) -> bool {
        false
    }

    fn safe_insert(&mut self, slot: usize, src_item: ItemStack, take_count: i32) -> ItemStack {
        let Some(slot_item) = self.menu_mut().slot_mut(slot) else {
            return src_item;
        };
        let ItemStack::Present(mut src_item) = src_item else {
            return src_item;
        };

        let take_count = cmp::min(
            cmp::min(take_count, src_item.count),
            src_item.kind.max_stack_size() - slot_item.count(),
        );
        if take_count <= 0 {
            return src_item.into();
        }
        let take_count = take_count as u32;

        if slot_item.is_empty() {
            *slot_item = src_item.split(take_count).into();
        } else if let ItemStack::Present(slot_item) = slot_item
            && slot_item.is_same_item_and_components(&src_item)
        {
            src_item.count -= take_count as i32;
            slot_item.count += take_count as i32;
        }

        src_item.into()
    }

    fn try_remove(&mut self, slot: usize, count: i32, limit: i32) -> Option<ItemStack> {
        if !self.menu().may_pickup(slot) {
            return None;
        }
        let mut slot_item = self.menu().slot(slot)?.clone();
        if !self.menu().allow_modification(slot) && limit < slot_item.count() {
            return None;
        }

        let count = count.min(limit);
        if count <= 0 {
            return None;
        }
        // vanilla calls .remove here but i think it has the same behavior as split?
        let removed = slot_item.split(count as u32);

        if removed.is_present() && slot_item.is_empty() {
            *self.menu_mut().slot_mut(slot).unwrap() = ItemStack::Empty;
        }

        Some(removed)
    }
}

fn can_item_quick_replace(
    target_slot: &ItemStack,
    item: &ItemStack,
    ignore_item_count: bool,
) -> bool {
    let ItemStack::Present(target_slot) = target_slot else {
        return false;
    };
    let ItemStack::Present(item) = item else {
        // i *think* this is what vanilla does
        // not 100% sure lol probably doesn't matter though
        return false;
    };

    if !item.is_same_item_and_components(target_slot) {
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

fn get_quick_craft_slot_count(
    quick_craft_slots: &HashSet<u16>,
    quick_craft_kind: &QuickCraftKind,
    item: &mut ItemStackData,
    slot_item_count: i32,
) {
    item.count = match quick_craft_kind {
        QuickCraftKind::Left => item.count / quick_craft_slots.len() as i32,
        QuickCraftKind::Right => 1,
        QuickCraftKind::Middle => item.kind.max_stack_size(),
    };
    item.count += slot_item_count;
}

impl Default for Inventory {
    fn default() -> Self {
        Inventory {
            inventory_menu: Menu::Player(azalea_inventory::Player::default()),
            id: 0,
            container_menu: None,
            container_menu_title: None,
            carried: ItemStack::Empty,
            state_id: 0,
            quick_craft_status: QuickCraftStatusKind::Start,
            quick_craft_kind: QuickCraftKind::Middle,
            quick_craft_slots: HashSet::new(),
            selected_hotbar_slot: 0,
        }
    }
}

/// A Bevy trigger that's fired when our client should show a new screen (like a
/// chest or crafting table).
///
/// To watch for the menu being closed, you could use
/// [`ClientsideCloseContainerEvent`]. To close it manually, use
/// [`CloseContainerEvent`].
#[derive(EntityEvent, Debug, Clone)]
pub struct MenuOpenedEvent {
    pub entity: Entity,
    pub window_id: i32,
    pub menu_type: MenuKind,
    pub title: FormattedText,
}
fn handle_menu_opened_trigger(event: On<MenuOpenedEvent>, mut query: Query<&mut Inventory>) {
    let mut inventory = query.get_mut(event.entity).unwrap();
    inventory.id = event.window_id;
    inventory.container_menu = Some(Menu::from_kind(event.menu_type));
    inventory.container_menu_title = Some(event.title.clone());
}

/// Tell the server that we want to close a container.
///
/// Note that this is also sent when the client closes its own inventory, even
/// though there is no packet for opening its inventory.
#[derive(EntityEvent)]
pub struct CloseContainerEvent {
    pub entity: Entity,
    /// The ID of the container to close. 0 for the player's inventory. If this
    /// is not the same as the currently open inventory, nothing will happen.
    pub id: i32,
}
fn handle_container_close_event(
    close_container: On<CloseContainerEvent>,
    mut commands: Commands,
    query: Query<(Entity, &Inventory)>,
) {
    let (entity, inventory) = query.get(close_container.entity).unwrap();
    if close_container.id != inventory.id {
        warn!(
            "Tried to close container with ID {}, but the current container ID is {}",
            close_container.id, inventory.id
        );
        return;
    }

    commands.trigger(SendGamePacketEvent::new(
        entity,
        ServerboundContainerClose {
            container_id: inventory.id,
        },
    ));
    commands.trigger(ClientsideCloseContainerEvent {
        entity: close_container.entity,
    });
}

/// A Bevy trigger that's fired when our client closed a container.
///
/// This can also be triggered directly to close a container silently without
/// sending any packets to the server. You probably don't want that though, and
/// should instead use [`CloseContainerEvent`].
///
/// If you want to watch for a container being opened, you should use
/// [`MenuOpenedEvent`].
#[derive(EntityEvent, Clone)]
pub struct ClientsideCloseContainerEvent {
    pub entity: Entity,
}
pub fn handle_client_side_close_container_trigger(
    event: On<ClientsideCloseContainerEvent>,
    mut query: Query<&mut Inventory>,
) {
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

        let new_inventory = inventory_menu.slots()[inventory_menu.player_slots_range()].to_vec();
        let new_inventory = <[ItemStack; 36]>::try_from(new_inventory).unwrap();
        *inventory.inventory_menu.as_player_mut().inventory = new_inventory;
    }

    inventory.id = 0;
    inventory.container_menu_title = None;
}

#[derive(EntityEvent, Debug)]
pub struct ContainerClickEvent {
    pub entity: Entity,
    pub window_id: i32,
    pub operation: ClickOperation,
}
pub fn handle_container_click_event(
    mut commands: Commands,
    container_click: On<ContainerClickEvent>,
    mut query: Query<(
        Entity,
        &mut Inventory,
        Option<&PlayerAbilities>,
        &InstanceName,
    )>,
    instance_container: Res<InstanceContainer>,
) {
    let (entity, mut inventory, player_abilities, instance_name) =
        query.get_mut(container_click.entity).unwrap();
    if inventory.id != container_click.window_id {
        error!(
            "Tried to click container with ID {}, but the current container ID is {}. Click packet won't be sent.",
            container_click.window_id, inventory.id
        );
        return;
    }

    let Some(instance) = instance_container.get(instance_name) else {
        return;
    };

    let old_slots = inventory.menu().slots();
    inventory.simulate_click(
        &container_click.operation,
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

    commands.trigger(SendGamePacketEvent::new(
        entity,
        ServerboundContainerClick {
            container_id: container_click.window_id,
            state_id: inventory.state_id,
            slot_num: container_click
                .operation
                .slot_num()
                .map(|n| n as i16)
                .unwrap_or(-999),
            button_num: container_click.operation.button_num(),
            click_type: container_click.operation.click_type(),
            changed_slots,
            carried_item: HashedStack::from_item_stack(&inventory.carried, registry_holder),
        },
    ));
}

/// Sent from the server when the contents of a container are replaced.
///
/// Usually triggered by the `ContainerSetContent` packet.
#[derive(EntityEvent)]
pub struct SetContainerContentEvent {
    pub entity: Entity,
    pub slots: Vec<ItemStack>,
    pub container_id: i32,
}
pub fn handle_set_container_content_trigger(
    set_container_content: On<SetContainerContentEvent>,
    mut query: Query<&mut Inventory>,
) {
    let mut inventory = query.get_mut(set_container_content.entity).unwrap();

    if set_container_content.container_id != inventory.id {
        warn!(
            "Got SetContainerContentEvent for container with ID {}, but the current container ID is {}",
            set_container_content.container_id, inventory.id
        );
        return;
    }

    let menu = inventory.menu_mut();
    for (i, slot) in set_container_content.slots.iter().enumerate() {
        if let Some(slot_mut) = menu.slot_mut(i) {
            *slot_mut = slot.clone();
        }
    }
}

/// An ECS message to switch our hand to a different hotbar slot.
///
/// This is equivalent to using the scroll wheel or number keys in Minecraft.
#[derive(Message)]
pub struct SetSelectedHotbarSlotEvent {
    pub entity: Entity,
    /// The hotbar slot to select. This should be in the range 0..=8.
    pub slot: u8,
}
pub fn handle_set_selected_hotbar_slot_event(
    mut events: MessageReader<SetSelectedHotbarSlotEvent>,
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

            commands.trigger(SendGamePacketEvent::new(
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
