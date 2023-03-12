use azalea_core::{BlockHitResult, BlockPos, Direction, GameMode, Vec3};
use azalea_physics::clip::{BlockShapeType, ClipContext, FluidPickType};
use azalea_protocol::packets::game::{
    serverbound_interact_packet::InteractionHand,
    serverbound_use_item_on_packet::{BlockHit, ServerboundUseItemOnPacket},
};
use azalea_world::{
    entity::{view_vector, EyeHeight, LookDirection, Position, WorldName},
    InstanceContainer,
};
use bevy_app::{App, Plugin};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::EventReader,
    system::{Query, Res},
};
use derive_more::{Deref, DerefMut};
use log::warn;

use crate::{local_player::LocalGameMode, Client, LocalPlayer};

/// A plugin that allows clients to interact with blocks in the world.
pub struct InteractPlugin;
impl Plugin for InteractPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BlockInteractEvent>()
            .add_systems((handle_block_interact_event, update_hit_result_component));
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

#[derive(Component, Clone, Debug, Deref, DerefMut)]
pub struct HitResultComponent(BlockHitResult);

fn handle_block_interact_event(
    mut events: EventReader<BlockInteractEvent>,
    mut query: Query<(&LocalPlayer, &mut CurrentSequenceNumber)>,
) {
    for event in events.iter() {
        let Ok((local_player, mut sequence_number)) = query.get_mut(event.entity) else {
            warn!("Sent BlockInteractEvent for entity that isn't LocalPlayer");
            continue;
        };

        // TODO: check to make sure we're within the world border

        **sequence_number += 1;

        // minecraft also does the interaction client-side (so it looks like clicking a
        // button is instant) but we don't really need that

        local_player.write_packet(
            ServerboundUseItemOnPacket {
                hand: InteractionHand::MainHand,
                block_hit: BlockHit {
                    block_pos: event.position,
                    direction: Direction::Up,
                    location: event.position.center(),
                    inside: false,
                },
                sequence: sequence_number.0,
            }
            .get(),
        )
    }
}

fn update_hit_result_component(
    mut query: Query<(
        &mut HitResultComponent,
        &LocalGameMode,
        &Position,
        &EyeHeight,
        &LookDirection,
        &WorldName,
    )>,
    instance_container: Res<InstanceContainer>,
) {
    for (mut hit_result_ref, game_mode, position, eye_height, look_direction, world_name) in
        &mut query
    {
        let pick_range = if game_mode.current == GameMode::Creative {
            6.
        } else {
            4.5
        };
        let view_vector = view_vector(look_direction);
        let eye_position = Vec3 {
            x: position.x,
            y: position.y + **eye_height as f64,
            z: position.z,
        };
        let end_position = eye_position + view_vector * pick_range;
        let instance_lock = instance_container
            .get(world_name)
            .expect("entities must always be in a valid world");
        let instance = instance_lock.read();
        let hit_result = azalea_physics::clip::clip(
            &instance.chunks,
            ClipContext {
                from: eye_position,
                to: end_position,
                block_shape_type: BlockShapeType::Outline,
                fluid_pick_type: FluidPickType::None,
            },
        );
        **hit_result_ref = hit_result;
    }
}
