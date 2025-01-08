use std::cmp;

use azalea_core::position::{BlockPos, Vec3};
use azalea_entity::{metadata::AbstractBoat, InLoadedChunk, LocalEntity, Physics, Position};
use azalea_registry::{EntityKind, Fluid};
use azalea_world::{Instance, InstanceContainer, InstanceName};
use bevy_ecs::prelude::*;

pub fn update_in_water_state_and_do_fluid_pushing(
    mut query: Query<
        (&mut Physics, &Position, &InstanceName),
        (With<LocalEntity>, With<InLoadedChunk>),
    >,
    instance_container: Res<InstanceContainer>,
) {
    for (mut physics, position, instance_name) in &mut query {
        let world_lock = instance_container
            .get(instance_name)
            .expect("All entities should be in a valid world");
        let world = world_lock.read();

        physics.water_fluid_height = 0.;
        physics.lava_fluid_height = 0.;

        update_in_water_state_and_do_water_current_pushing(&mut physics, &world, &position);
        let lava_push_factor = world
            .registries
            .dimension_type()
            .map(|d| d.lava_push_factor);
    }
}
fn update_in_water_state_and_do_water_current_pushing(
    physics: &mut Physics,
    world: &Instance,
    position: &Position,
) {
    // TODO: implement vehicles and boats
    // if vehicle == AbstractBoat {
    //     if !boat.is_underwater() {
    //         *was_touching_water = false;
    //     }
    // }

    // updateFluidHeightAndDoFluidPushing
    if update_fluid_height_and_do_fluid_pushing(physics, world, Fluid::Water, 0.014) {
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
    checking_fluid: Fluid,
    fluid_push_factor: f32,
) -> bool {
    // if touching_unloaded_chunk() {
    //     return false;
    // }

    let checking_liquids_aabb = physics.bounding_box.deflate_all(0.001);
    let min_x = checking_liquids_aabb.min.x.floor() as i32;
    let max_x = checking_liquids_aabb.max.x.ceil() as i32;
    let min_y = checking_liquids_aabb.min.y.floor() as i32;
    let max_y = checking_liquids_aabb.max.y.ceil() as i32;
    let min_z = checking_liquids_aabb.min.z.floor() as i32;
    let max_z = checking_liquids_aabb.max.z.ceil() as i32;

    let mut min_height_touching = 0.;
    let is_entity_pushable_by_fluid = true;
    let mut touching_fluid = false;
    let mut additional_player_delta = Vec3::default();
    let mut num_fluids_being_touched = 0;

    for cur_x in min_x..=max_x {
        for cur_y in min_y..=max_y {
            for cur_z in min_z..=max_z {
                let cur_pos = BlockPos::new(cur_x, cur_y, cur_z);
                let Some(fluid_at_cur_pos) = world.get_fluid_state(&cur_pos) else {
                    continue;
                };
                if fluid_at_cur_pos.fluid != checking_fluid {
                    continue;
                }
                let fluid_max_y = (cur_y as f32 + fluid_at_cur_pos.height()) as f64;
                if fluid_max_y < checking_liquids_aabb.min.y {
                    continue;
                }
                touching_fluid = true;
                min_height_touching = f64::min(
                    fluid_max_y - checking_liquids_aabb.min.y,
                    min_height_touching,
                );
                if !is_entity_pushable_by_fluid {
                    continue;
                }
                let mut additional_player_delta_for_fluid = fluid_at_cur_pos.flow();
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
        additionalPlayerDelta *= fluid_push_factor;
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
        Fluid::Water => physics.water_fluid_height = min_height_touching,
        Fluid::Lava => physics.lava_fluid_height = min_height_touching,
        checking_fluid => panic!("unknown fluid {checking_fluid}"),
    };

    touching_fluid
}

pub fn update_swimming() {
    // TODO: swimming
}
