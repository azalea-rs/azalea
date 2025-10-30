use azalea_core::{
    aabb::Aabb,
    direction::Direction,
    hit_result::{BlockHitResult, EntityHitResult, HitResult},
    position::Vec3,
};
use azalea_entity::{
    Attributes, Dead, LocalEntity, LookDirection, Physics, Position,
    dimensions::EntityDimensions,
    metadata::{
        AbstractArrow, AbstractBoat, AbstractLiving, AbstractMinecart, ArmorStand,
        ArmorStandMarker, EndCrystal, FallingBlock, InGround, Interaction, ShulkerBullet, Tnt,
    },
    view_vector,
};
use azalea_physics::{
    clip::{BlockShapeType, ClipContext, FluidPickType},
    collision::entity_collisions::{AabbQuery, get_entities},
};
use azalea_world::{Instance, InstanceContainer, InstanceName};
use bevy_ecs::prelude::*;
use derive_more::{Deref, DerefMut};

/// A component that contains the block or entity that the player is currently
/// looking at.
#[doc(alias("looking at", "looking at block", "crosshair"))]
#[derive(Component, Clone, Debug, Deref, DerefMut)]
pub struct HitResultComponent(HitResult);

#[allow(clippy::type_complexity)]
pub fn update_hit_result_component(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            Option<&mut HitResultComponent>,
            &Position,
            &EntityDimensions,
            &LookDirection,
            &InstanceName,
            &Physics,
            &Attributes,
        ),
        With<LocalEntity>,
    >,
    instance_container: Res<InstanceContainer>,
    aabb_query: AabbQuery,
    pickable_query: MaybePickableEntityQuery,
) {
    for (
        entity,
        hit_result_ref,
        position,
        dimensions,
        look_direction,
        world_name,
        physics,
        attributes,
    ) in &mut query
    {
        let block_pick_range = attributes.block_interaction_range.calculate();
        let entity_pick_range = attributes.entity_interaction_range.calculate();

        let eye_position = position.up(dimensions.eye_height.into());

        let Some(world_lock) = instance_container.get(world_name) else {
            continue;
        };
        let world = world_lock.read();

        let hit_result = pick(PickOpts {
            source_entity: entity,
            look_direction: *look_direction,
            eye_position,
            aabb: &physics.bounding_box,
            world: &world,
            entity_pick_range,
            block_pick_range,
            aabb_query: &aabb_query,
            pickable_query: &pickable_query,
        });
        if let Some(mut hit_result_ref) = hit_result_ref {
            **hit_result_ref = hit_result;
        } else {
            commands
                .entity(entity)
                .insert(HitResultComponent(hit_result));
        }
    }
}

pub type MaybePickableEntityQuery<'world, 'state, 'a> = Query<
    'world,
    'state,
    (Option<&'a ArmorStandMarker>, Option<&'a InGround>),
    // search "isPickable" in the decompiled minecraft code
    (
        Or<(
            // TODO: ender dragon parts are pickable but the ender dragon itself isn't. this needs
            // more investigation.
            (With<Tnt>, Without<Dead>),
            (With<FallingBlock>, Without<Dead>),
            (With<AbstractMinecart>, Without<Dead>),
            (With<AbstractBoat>, Without<Dead>),
            With<ArmorStand>,
            With<EndCrystal>,
            With<Interaction>,
            With<ShulkerBullet>,
            (With<AbstractLiving>, Without<Dead>),
            With<AbstractArrow>,
        )>,
    ),
>;

pub struct PickOpts<'world, 'state, 'a, 'b, 'c> {
    source_entity: Entity,
    look_direction: LookDirection,
    eye_position: Vec3,
    aabb: &'a Aabb,
    world: &'a Instance,
    entity_pick_range: f64,
    block_pick_range: f64,
    aabb_query: &'a AabbQuery<'world, 'state, 'b>,
    pickable_query: &'a MaybePickableEntityQuery<'world, 'state, 'c>,
}

/// Get the block or entity that a player would be looking at if their eyes were
/// at the given direction and position.
///
/// If you need to get the block/entity the player is looking at right now, use
/// [`HitResultComponent`].
///
/// Also see [`pick_block`].
pub fn pick(opts: PickOpts<'_, '_, '_, '_, '_>) -> HitResult {
    // vanilla does extra math here to calculate the pick result in between ticks by
    // interpolating, but since clients can still only interact on exact ticks, that
    // isn't relevant for us.

    let mut max_range = opts.entity_pick_range.max(opts.block_pick_range);
    let mut max_range_squared = max_range.powi(2);

    let block_hit_result = pick_block(
        opts.look_direction,
        opts.eye_position,
        &opts.world.chunks,
        max_range,
    );
    let block_hit_result_dist_squared = block_hit_result
        .location
        .distance_squared_to(opts.eye_position);
    if !block_hit_result.miss {
        max_range_squared = block_hit_result_dist_squared;
        max_range = block_hit_result_dist_squared.sqrt();
    }

    let view_vector = view_vector(opts.look_direction);
    let end_position = opts.eye_position + (view_vector * max_range);
    let inflate_by = 1.;
    let pick_aabb = opts
        .aabb
        .expand_towards(view_vector * max_range)
        .inflate_all(inflate_by);

    let is_pickable = |entity: Entity| {
        if entity == opts.source_entity {
            return false;
        }

        // TODO: ender dragon has extra logic here. also, we shouldn't be able to pick
        // spectators.
        if let Ok((armor_stand_marker, arrow_in_ground)) = opts.pickable_query.get(entity) {
            !(armor_stand_marker == Some(&ArmorStandMarker(true))
                || arrow_in_ground == Some(&InGround(true)))
        } else {
            false
        }
    };
    let entity_hit_result = pick_entity(PickEntityOpts {
        source_entity: opts.source_entity,
        eye_position: opts.eye_position,
        end_position,
        world: opts.world,
        pick_range_squared: max_range_squared,
        predicate: &is_pickable,
        aabb: &pick_aabb,
        aabb_query: opts.aabb_query,
    });

    if let Some(entity_hit_result) = entity_hit_result
        && entity_hit_result
            .location
            .distance_squared_to(opts.eye_position)
            < block_hit_result_dist_squared
    {
        filter_hit_result(
            HitResult::Entity(entity_hit_result),
            opts.eye_position,
            opts.entity_pick_range,
        )
    } else {
        filter_hit_result(
            HitResult::Block(block_hit_result),
            opts.eye_position,
            opts.block_pick_range,
        )
    }
}

fn filter_hit_result(hit_result: HitResult, eye_position: Vec3, range: f64) -> HitResult {
    let location = hit_result.location();
    if !location.closer_than(eye_position, range) {
        let direction = Direction::nearest(location - eye_position);
        HitResult::new_miss(location, direction, location.into())
    } else {
        hit_result
    }
}

/// Get the block that a player would be looking at if their eyes were at the
/// given direction and position.
///
/// This does not consider entities.
///
/// Also see [`pick`].
pub fn pick_block(
    look_direction: LookDirection,
    eye_position: Vec3,
    chunks: &azalea_world::ChunkStorage,
    pick_range: f64,
) -> BlockHitResult {
    let view_vector = view_vector(look_direction);
    let end_position = eye_position + (view_vector * pick_range);

    azalea_physics::clip::clip(
        chunks,
        ClipContext {
            from: eye_position,
            to: end_position,
            block_shape_type: BlockShapeType::Outline,
            fluid_pick_type: FluidPickType::None,
        },
    )
}

struct PickEntityOpts<'world, 'state, 'a, 'b> {
    source_entity: Entity,
    eye_position: Vec3,
    end_position: Vec3,
    world: &'a azalea_world::Instance,
    pick_range_squared: f64,
    predicate: &'a dyn Fn(Entity) -> bool,
    aabb: &'a Aabb,
    aabb_query: &'a AabbQuery<'world, 'state, 'b>,
}

// port of getEntityHitResult
fn pick_entity(opts: PickEntityOpts) -> Option<EntityHitResult> {
    let mut picked_distance_squared = opts.pick_range_squared;
    let mut result = None;

    for (candidate, candidate_aabb) in get_entities(
        opts.world,
        Some(opts.source_entity),
        opts.aabb,
        opts.predicate,
        opts.aabb_query,
    ) {
        // TODO: if the entity is "REDIRECTABLE_PROJECTILE" then this should be 1.0.
        // azalea needs support for entity tags first for this to be possible. see
        // getPickRadius in decompiled minecraft source
        let candidate_pick_radius = 0.;
        let candidate_aabb = candidate_aabb.inflate_all(candidate_pick_radius);
        let clip_location = candidate_aabb.clip(opts.eye_position, opts.end_position);

        if candidate_aabb.contains(opts.eye_position) {
            if picked_distance_squared >= 0. {
                result = Some(EntityHitResult {
                    location: clip_location.unwrap_or(opts.eye_position),
                    entity: candidate,
                });
                picked_distance_squared = 0.;
            }
        } else if let Some(clip_location) = clip_location {
            let distance_squared = opts.eye_position.distance_squared_to(clip_location);
            if distance_squared < picked_distance_squared || picked_distance_squared == 0. {
                // TODO: don't pick the entity we're riding on
                // if candidate_root_vehicle == entity_root_vehicle {
                //     if picked_distance_squared == 0. {
                //         picked_entity = Some(candidate);
                //         picked_location = Some(clip_location);
                //     }
                // } else {
                result = Some(EntityHitResult {
                    location: clip_location,
                    entity: candidate,
                });
                picked_distance_squared = distance_squared;
            }
        }
    }

    result
}
