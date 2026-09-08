use azalea_core::position::Vec3;
use azalea_entity::{
    GroundContact, HasClientLoaded, LocalEntity, MovementResult, Physics, Position,
};
use azalea_world::{WorldName, Worlds};
use bevy_ecs::prelude::*;

use crate::collision::world_collisions::{EntityCollisionContext, find_supporting_block};

#[derive(PartialEq)]
pub enum SupportingBlockUpdate {
    LocalMovement,
    PositionUpdateFromServer,
}

/// a `Component` for marking if the server sent position update packet for the
/// client
#[derive(Component)]
pub struct PositionUpdatedFromServer;

pub fn clear_server_update_flag(
    mut commands: Commands,
    query: Query<Entity, With<PositionUpdatedFromServer>>,
) {
    for entity in query {
        commands
            .entity(entity)
            .remove::<PositionUpdatedFromServer>();
    }
}


#[allow(clippy::type_complexity)]
pub fn update_main_supporting_block_pos_from_server(
    mut query: Query<
        (
            Entity,
            &Position,
            &WorldName,
            &MovementResult,
            &mut Physics,
            &mut GroundContact,
        ),
        (
            With<LocalEntity>,
            With<HasClientLoaded>,
            With<PositionUpdatedFromServer>,
        ),
    >,
    worlds: Res<Worlds>,
) {
    for (entity, position, world_name, movement_result, physics, mut ground_contact) in &mut query {
        let Some(world_lock) = worlds.get(world_name) else {
            continue;
        };
        let world = world_lock.read();
        update_main_supporting_block_pos(
            SupportingBlockUpdate::PositionUpdateFromServer,
            entity,
            position,
            &world,
            movement_result,
            &physics,
            &mut ground_contact,
        );
    }
}

#[allow(clippy::type_complexity)]
pub fn update_main_supporting_block_pos_local(
    mut query: Query<
        (
            Entity,
            &Position,
            &WorldName,
            &MovementResult,
            &Physics,
            &mut GroundContact,
        ),
        (With<LocalEntity>, With<HasClientLoaded>),
    >,
    worlds: Res<Worlds>,
) {
    for (entity, position, world_name, movement_result, physics, mut ground_contact) in &mut query {
        let Some(world_lock) = worlds.get(world_name) else {
            continue;
        };
        let world = world_lock.read();
        update_main_supporting_block_pos(
            SupportingBlockUpdate::LocalMovement,
            entity,
            position,
            &world,
            movement_result,
            physics,
            &mut ground_contact,
        );
    }
}

fn update_main_supporting_block_pos(
    update_context: SupportingBlockUpdate,
    entity: Entity,
    position: &Position,
    world: &azalea_world::World,
    movement_result: &MovementResult,
    physics: &Physics,
    ground_contact: &mut GroundContact,
) {
    if !ground_contact.on_ground() {
        ground_contact.on_ground_no_support = false;
        ground_contact.supporting_block = None;
        return;
    }

    let test_box = {
        let mut box_ = physics.bounding_box;
        box_.max.y = box_.min.y;
        box_.min.y -= 1e-6;
        box_
    }; // small volume under player foot

    let pos = find_supporting_block(
        world,
        test_box,
        **position,
        EntityCollisionContext::of(Some(entity)),
    );

    let pos = if pos.is_some() || ground_contact.on_ground_no_support {
        pos
    } else if update_context == SupportingBlockUpdate::LocalMovement {
        let movement = movement_result.actual;
        let fallback_testbox = test_box.move_relative(Vec3 {
            x: -movement.x,
            y: 0.0,
            z: -movement.z,
        });
        find_supporting_block(
            world,
            fallback_testbox,
            **position,
            EntityCollisionContext::of(Some(entity)),
        )
    } else {
        pos
    };

    ground_contact.supporting_block = pos;
    ground_contact.on_ground_no_support = pos.is_none();
}
