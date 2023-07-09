use azalea_block::BlockState;
use azalea_core::{BlockHitResult, BlockPos, Direction, GameMode, Vec3};
use azalea_inventory::{ItemSlot, ItemSlotData};
use azalea_nbt::NbtList;
use azalea_physics::clip::{BlockShapeType, ClipContext, FluidPickType};
use azalea_protocol::packets::game::{
    serverbound_interact_packet::InteractionHand,
    serverbound_use_item_on_packet::{BlockHit, ServerboundUseItemOnPacket},
};
use azalea_world::{
    entity::{clamp_look_direction, view_vector, EyeHeight, LookDirection, Position, WorldName},
    Instance, InstanceContainer,
};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::EventReader,
    prelude::Event,
    schedule::IntoSystemConfigs,
    system::{Commands, Query, Res},
};
use derive_more::{Deref, DerefMut};
use log::warn;

use crate::{
    inventory::InventoryComponent,
    local_player::{handle_send_packet_event, LocalGameMode},
    Client, LocalPlayer,
};

/// A plugin that allows clients to interact with blocks in the world.
pub struct InteractPlugin;
impl Plugin for InteractPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BlockInteractEvent>().add_systems(
            Update,
            (
                update_hit_result_component.after(clamp_look_direction),
                handle_block_interact_event,
            )
                .before(handle_send_packet_event)
                .chain(),
        );
    }
}

impl Client {
    /// Right click a block. The behavior of this depends on the target block,
    /// and it'll either place the block you're holding in your hand or use the
    /// block you clicked (like toggling a lever).
    ///
    /// Note that this may trigger anticheats as it doesn't take into account
    /// whether you're actually looking at the block.
    pub fn block_interact(&mut self, position: BlockPos) {
        self.ecs.lock().send_event(BlockInteractEvent {
            entity: self.entity,
            position,
        });
    }
}

/// Right click a block. The behavior of this depends on the target block,
/// and it'll either place the block you're holding in your hand or use the
/// block you clicked (like toggling a lever).
#[derive(Event)]
pub struct BlockInteractEvent {
    /// The local player entity that's opening the container.
    pub entity: Entity,
    /// The coordinates of the container.
    pub position: BlockPos,
}

/// A component that contains the number of changes this client has made to
/// blocks.
#[derive(Component, Copy, Clone, Debug, Default, Deref, DerefMut)]
pub struct CurrentSequenceNumber(u32);

/// A component that contains the block that the player is currently looking at.
#[derive(Component, Clone, Debug, Deref, DerefMut)]
pub struct HitResultComponent(BlockHitResult);

pub fn handle_block_interact_event(
    mut events: EventReader<BlockInteractEvent>,
    mut query: Query<(
        &LocalPlayer,
        &mut CurrentSequenceNumber,
        &HitResultComponent,
    )>,
) {
    for event in events.iter() {
        let Ok((local_player, mut sequence_number, hit_result)) = query.get_mut(event.entity) else {
            warn!("Sent BlockInteractEvent for entity that isn't LocalPlayer");
            continue;
        };

        // TODO: check to make sure we're within the world border

        **sequence_number += 1;

        // minecraft also does the interaction client-side (so it looks like clicking a
        // button is instant) but we don't really need that

        // the block_hit data will depend on whether we're looking at the block and
        // whether we can reach it

        let block_hit = if hit_result.block_pos == event.position {
            // we're looking at the block :)
            BlockHit {
                block_pos: hit_result.block_pos,
                direction: hit_result.direction,
                location: hit_result.location,
                inside: hit_result.inside,
            }
        } else {
            // we're not looking at the block, so make up some numbers
            BlockHit {
                block_pos: event.position,
                direction: Direction::Up,
                location: event.position.center(),
                inside: false,
            }
        };

        local_player.write_packet(
            ServerboundUseItemOnPacket {
                hand: InteractionHand::MainHand,
                block_hit,
                sequence: sequence_number.0,
            }
            .get(),
        )
    }
}

#[allow(clippy::type_complexity)]
fn update_hit_result_component(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        Option<&mut HitResultComponent>,
        &LocalGameMode,
        &Position,
        &EyeHeight,
        &LookDirection,
        &WorldName,
    )>,
    instance_container: Res<InstanceContainer>,
) {
    for (entity, hit_result_ref, game_mode, position, eye_height, look_direction, world_name) in
        &mut query
    {
        let pick_range = if game_mode.current == GameMode::Creative {
            6.
        } else {
            4.5
        };
        let eye_position = Vec3 {
            x: position.x,
            y: position.y + **eye_height as f64,
            z: position.z,
        };

        let Some(instance_lock) = instance_container.get(world_name) else {
            continue;
        };
        let instance = instance_lock.read();

        let hit_result = pick(look_direction, &eye_position, &instance, pick_range);
        if let Some(mut hit_result_ref) = hit_result_ref {
            **hit_result_ref = hit_result;
        } else {
            commands
                .entity(entity)
                .insert(HitResultComponent(hit_result));
        }
    }
}

/// Get the block that a player would be looking at if their eyes were at the
/// given direction and position.
///
/// If you need to get the block the player is looking at right now, use
/// [`HitResultComponent`].
pub fn pick(
    look_direction: &LookDirection,
    eye_position: &Vec3,
    instance: &Instance,
    pick_range: f64,
) -> BlockHitResult {
    let view_vector = view_vector(look_direction);
    let end_position = eye_position + &(view_vector * pick_range);
    azalea_physics::clip::clip(
        &instance.chunks,
        ClipContext {
            from: *eye_position,
            to: end_position,
            block_shape_type: BlockShapeType::Outline,
            fluid_pick_type: FluidPickType::None,
        },
    )
}

/// Whether we can't interact with the block, based on your gamemode. If
/// this is false, then we can interact with the block.
///
/// Passing the inventory, block position, and instance is necessary for the
/// adventure mode check.
pub fn check_is_interaction_restricted(
    instance: &Instance,
    block_pos: &BlockPos,
    game_mode: &GameMode,
    inventory: &InventoryComponent,
) -> bool {
    match game_mode {
        GameMode::Adventure => {
            // vanilla checks for abilities.mayBuild here but servers have no
            // way of modifying that

            let held_item = inventory.held_item();
            if let ItemSlot::Present(item) = &held_item {
                let block = instance.chunks.get_block_state(block_pos);
                let Some(block) = block else {
                    // block isn't loaded so just say that it is restricted
                    return true;
                };
                check_block_can_be_broken_by_item_in_adventure_mode(item, &block)
            } else {
                true
            }
        }
        GameMode::Spectator => true,
        _ => false,
    }
}

/// Check if the item has the `CanDestroy` tag for the block.
pub fn check_block_can_be_broken_by_item_in_adventure_mode(
    item: &ItemSlotData,
    _block: &BlockState,
) -> bool {
    // minecraft caches the last checked block but that's kind of an unnecessary
    // optimization and makes the code too complicated

    let Some(can_destroy) = item
        .nbt
        .as_compound()
        .and_then(|nbt| nbt.get("tag").and_then(|nbt| nbt.as_compound()))
        .and_then(|nbt| nbt.get("CanDestroy").and_then(|nbt| nbt.as_list())) else {
            // no CanDestroy tag
            return false;
        };

    let NbtList::String(_can_destroy) = can_destroy else {
        // CanDestroy tag must be a list of strings
        return false;
    };

    false

    // for block_predicate in can_destroy {
    //     // TODO
    //     // defined in BlockPredicateArgument.java
    // }

    // true
}
