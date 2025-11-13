use azalea_block::{
    BlockState,
    fluid_state::{FluidKind, FluidState},
};
use azalea_core::{
    direction::Direction,
    identifier::Identifier,
    position::{BlockPos, Vec3},
};
use azalea_entity::{HasClientLoaded, LocalEntity, Physics, Position};
use azalea_world::{Instance, InstanceContainer, InstanceName};
use bevy_ecs::prelude::*;

use crate::collision::legacy_blocks_motion;

#[allow(clippy::type_complexity)]
pub fn update_in_water_state_and_do_fluid_pushing(
    mut query: Query<
        (&mut Physics, &Position, &InstanceName),
        (With<LocalEntity>, With<HasClientLoaded>),
    >,
    instance_container: Res<InstanceContainer>,
) {
    for (mut physics, position, instance_name) in &mut query {
        let Some(world_lock) = instance_container.get(instance_name) else {
            continue;
        };
        let world = world_lock.read();

        // reset the heights since they're going to be set in
        // update_in_water_state_and_do_water_current_pushing
        physics.water_fluid_height = 0.;
        physics.lava_fluid_height = 0.;

        update_in_water_state_and_do_water_current_pushing(&mut physics, &world, *position);

        // right now doing registries.dimension_type() clones the entire registry which
        // is very inefficient, so for now we're doing this instead

        let is_ultrawarm = world
            .registries
            .map
            .get(&Identifier::new("minecraft:dimension_type"))
            .and_then(|d| {
                d.get(&**instance_name)
                    .map(|d| d.byte("ultrawarm") != Some(0))
            })
            .unwrap_or_default();
        let lava_push_factor = if is_ultrawarm {
            0.007
        } else {
            0.0023333333333333335
        };

        update_fluid_height_and_do_fluid_pushing(
            &mut physics,
            &world,
            FluidKind::Lava,
            lava_push_factor,
        );
    }
}
fn update_in_water_state_and_do_water_current_pushing(
    physics: &mut Physics,
    world: &Instance,
    _position: Position,
) {
    // TODO: implement vehicles and boats
    // if vehicle == AbstractBoat {
    //     if !boat.is_underwater() {
    //         *was_touching_water = false;
    //     }
    // }

    // updateFluidHeightAndDoFluidPushing
    if update_fluid_height_and_do_fluid_pushing(physics, world, FluidKind::Water, 0.014) {
        // if !was_touching_water && !first_tick {
        //     do_water_splash_effect();
        // }

        physics.reset_fall_distance();
        physics.was_touching_water = true;
        physics.clear_fire();
    } else {
        physics.was_touching_water = false;
    }
}

fn update_fluid_height_and_do_fluid_pushing(
    physics: &mut Physics,
    world: &Instance,
    checking_fluid: FluidKind,
    fluid_push_factor: f64,
) -> bool {
    // if touching_unloaded_chunk() {
    //     return false;
    // }

    let checking_liquids_aabb = physics.bounding_box.deflate_all(0.001);

    let min_x = checking_liquids_aabb.min.x.floor() as i32;
    let min_y = checking_liquids_aabb.min.y.floor() as i32;
    let min_z = checking_liquids_aabb.min.z.floor() as i32;

    let max_x = checking_liquids_aabb.max.x.ceil() as i32;
    let max_y = checking_liquids_aabb.max.y.ceil() as i32;
    let max_z = checking_liquids_aabb.max.z.ceil() as i32;

    let mut min_height_touching = 0.;
    let is_entity_pushable_by_fluid = true;
    let mut touching_fluid = false;
    let mut additional_player_delta = Vec3::ZERO;
    let mut num_fluids_being_touched = 0;

    for cur_x in min_x..max_x {
        for cur_y in min_y..max_y {
            for cur_z in min_z..max_z {
                let cur_pos = BlockPos::new(cur_x, cur_y, cur_z);
                let Some(fluid_at_cur_pos) = world.get_fluid_state(cur_pos) else {
                    continue;
                };
                if fluid_at_cur_pos.kind != checking_fluid {
                    continue;
                }
                let fluid_max_y = (cur_y as f32 + fluid_at_cur_pos.height()) as f64;
                if fluid_max_y < checking_liquids_aabb.min.y {
                    continue;
                }
                touching_fluid = true;
                min_height_touching = f64::max(
                    fluid_max_y - checking_liquids_aabb.min.y,
                    min_height_touching,
                );
                if !is_entity_pushable_by_fluid {
                    continue;
                }
                let mut additional_player_delta_for_fluid =
                    get_fluid_flow(&fluid_at_cur_pos, world, cur_pos);
                if min_height_touching < 0.4 {
                    additional_player_delta_for_fluid *= min_height_touching;
                };

                additional_player_delta += additional_player_delta_for_fluid;
                num_fluids_being_touched += 1;
            }
        }
    }

    if additional_player_delta.length() > 0. {
        additional_player_delta /= num_fluids_being_touched as f64;

        // if entity_kind != EntityKind::Player {
        //     additional_player_delta = additional_player_delta.normalize();
        // }

        let player_delta = physics.velocity;
        additional_player_delta *= fluid_push_factor;
        const MIN_PUSH: f64 = 0.003;
        const MIN_PUSH_LENGTH: f64 = MIN_PUSH * 1.5;

        if player_delta.x.abs() < MIN_PUSH
            && player_delta.z.abs() < MIN_PUSH
            && additional_player_delta.length() < MIN_PUSH_LENGTH
        {
            additional_player_delta = additional_player_delta.normalize() * MIN_PUSH_LENGTH;
        }

        physics.velocity += additional_player_delta;
    }

    match checking_fluid {
        FluidKind::Water => physics.water_fluid_height = min_height_touching,
        FluidKind::Lava => physics.lava_fluid_height = min_height_touching,
        FluidKind::Empty => panic!("FluidKind::Empty should not be passed to update_fluid_height"),
    };

    touching_fluid
}

pub fn update_swimming() {
    // TODO: swimming
}

// FlowingFluid.getFlow
pub fn get_fluid_flow(fluid: &FluidState, world: &Instance, pos: BlockPos) -> Vec3 {
    let mut z_flow: f64 = 0.;
    let mut x_flow: f64 = 0.;

    let cur_fluid_height = fluid.height();

    for direction in Direction::HORIZONTAL {
        let adjacent_block_pos = pos.offset_with_direction(direction);

        let adjacent_block_state = world
            .get_block_state(adjacent_block_pos)
            .unwrap_or_default();
        let adjacent_fluid_state = FluidState::from(adjacent_block_state);

        if !fluid.affects_flow(&adjacent_fluid_state) {
            continue;
        };
        let mut adjacent_fluid_height = adjacent_fluid_state.height();
        let mut adjacent_height_difference: f32 = 0.;

        if adjacent_fluid_height == 0. {
            if !legacy_blocks_motion(adjacent_block_state) {
                let block_pos_below_adjacent = adjacent_block_pos.down(1);
                let fluid_below_adjacent = world
                    .get_fluid_state(block_pos_below_adjacent)
                    .unwrap_or_default();

                if fluid.affects_flow(&fluid_below_adjacent) {
                    adjacent_fluid_height = fluid_below_adjacent.height();
                    if adjacent_fluid_height > 0. {
                        adjacent_height_difference =
                            cur_fluid_height - (adjacent_fluid_height - 0.8888889);
                    }
                }
            }
        } else if adjacent_fluid_height > 0. {
            adjacent_height_difference = cur_fluid_height - adjacent_fluid_height;
        }

        if adjacent_height_difference != 0. {
            x_flow += (direction.x() as f32 * adjacent_height_difference) as f64;
            z_flow += (direction.z() as f32 * adjacent_height_difference) as f64;
        }
    }

    let mut flow = Vec3::new(x_flow, 0., z_flow);
    if fluid.falling {
        for direction in Direction::HORIZONTAL {
            let adjacent_block_pos = pos.offset_with_direction(direction);
            if is_solid_face(fluid, world, adjacent_block_pos, direction)
                || is_solid_face(fluid, world, adjacent_block_pos.up(1), direction)
            {
                flow = flow.normalize() + Vec3::new(0., -6., 0.);
                break;
            }
        }
    }

    flow.normalize()
}

// i don't really get what this is for
fn is_solid_face(
    fluid: &FluidState,
    world: &Instance,
    adjacent_pos: BlockPos,
    direction: Direction,
) -> bool {
    let block_state = world.get_block_state(adjacent_pos).unwrap_or_default();
    let fluid_state = world.get_fluid_state(adjacent_pos).unwrap_or_default();
    if fluid_state.is_same_kind(fluid) {
        return false;
    }
    if direction == Direction::Up {
        return true;
    }
    let registry_block = azalea_registry::Block::from(block_state);
    if matches!(
        registry_block,
        // frosted ice is from frost walker
        azalea_registry::Block::Ice | azalea_registry::Block::FrostedIce
    ) {
        return false;
    }
    is_face_sturdy(block_state, world, adjacent_pos, direction)
}

fn is_face_sturdy(
    _block_state: BlockState,
    _world: &Instance,
    _pos: BlockPos,
    _direction: Direction,
) -> bool {
    // TODO: this does a whole bunch of physics shape checks for waterlogged blocks
    // that i honestly cannot be bothered to implement right now

    // see BlockBehavior.isFaceSturdy in the decompiled minecraft source

    // also, this probably should be in a module other than fluids.rs

    false
}
