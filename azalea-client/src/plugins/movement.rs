use azalea_core::{
    entity_id::MinecraftEntityId,
    game_type::GameMode,
    position::{Vec2, Vec3},
    tick::GameTick,
};
use azalea_entity::{
    Attributes, Crouching, EntityGeometryUpdateSystems, HasClientLoaded, Jumping, LastSentPosition,
    LocalEntity, LookDirection, OnClimbable, Physics, PlayerAbilities, Pose, Position,
    dimensions::calculate_dimensions,
    inventory::Inventory,
    metadata::{self, FallFlying, Sprinting},
    update_bounding_box,
};
use azalea_inventory::components::{self, EquipmentSlot};
use azalea_physics::{
    PhysicsSystems, ai_step,
    client_movement::{ClientMovementState, SprintDirection, WalkDirection},
    collision::entity_collisions::{AabbQuery, CollidableEntityQuery, update_last_bounding_box},
    travel::{no_collision, travel},
};
use azalea_protocol::{
    common::movements::MoveFlags,
    packets::{
        Packet,
        game::{
            ServerboundPlayerCommand, ServerboundPlayerInput,
            s_move_player_pos::ServerboundMovePlayerPos,
            s_move_player_pos_rot::ServerboundMovePlayerPosRot,
            s_move_player_rot::ServerboundMovePlayerRot,
            s_move_player_status_only::ServerboundMovePlayerStatusOnly, s_player_command,
        },
    },
};
use azalea_registry::builtin::EntityKind;
use azalea_world::World;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;

use crate::{
    local_player::{Hunger, WorldHolder},
    packet::game::SendGamePacketEvent,
};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<StartWalkEvent>()
            .add_message::<StartSprintEvent>()
            .add_systems(
                Update,
                (handle_sprint, handle_walk)
                    .chain()
                    .in_set(MoveEventsSystems)
                    .after(update_bounding_box)
                    .after(update_last_bounding_box),
            )
            .add_systems(
                GameTick,
                (
                    (
                        tick_controls,
                        local_player_ai_step,
                        process_fall_flying_activation,
                    )
                        .chain()
                        .in_set(PhysicsSystems)
                        .before(ai_step)
                        .before(azalea_physics::fluids::update_in_water_state_and_do_fluid_pushing),
                    send_player_input_packet,
                    update_pose.before(EntityGeometryUpdateSystems),
                    send_sprinting_if_needed
                        .after(azalea_entity::update_in_loaded_chunk)
                        .after(travel)
                        .after(EntityGeometryUpdateSystems),
                    send_position,
                )
                    .chain(),
            )
            .add_observer(handle_knockback);
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct MoveEventsSystems;

/// A component that contains the look direction that was last sent over the
/// network.
#[derive(Clone, Component, Debug, Default)]
pub struct LastSentLookDirection {
    pub x_rot: f32,
    pub y_rot: f32,
}

#[allow(clippy::type_complexity)]
pub fn send_position(
    mut query: Query<
        (
            Entity,
            &Position,
            &LookDirection,
            &mut ClientMovementState,
            &mut LastSentPosition,
            &mut Physics,
            &mut LastSentLookDirection,
        ),
        With<HasClientLoaded>,
    >,
    mut commands: Commands,
) {
    for (
        entity,
        position,
        direction,
        mut physics_state,
        mut last_sent_position,
        mut physics,
        mut last_direction,
    ) in query.iter_mut()
    {
        let packet = {
            // TODO: the camera being able to be controlled by other entities isn't
            // implemented yet if !self.is_controlled_camera() { return };

            let x_delta = position.x - last_sent_position.x;
            let y_delta = position.y - last_sent_position.y;
            let z_delta = position.z - last_sent_position.z;
            let y_rot_delta = (direction.y_rot() - last_direction.y_rot) as f64;
            let x_rot_delta = (direction.x_rot() - last_direction.x_rot) as f64;

            physics_state.position_remainder += 1;

            // boolean sendingPosition = Mth.lengthSquared(xDelta, yDelta, zDelta) >
            // Mth.square(2.0E-4D) || this.positionReminder >= 20;
            let is_delta_large_enough =
                (x_delta.powi(2) + y_delta.powi(2) + z_delta.powi(2)) > 2.0e-4f64.powi(2);
            let sending_position = is_delta_large_enough || physics_state.position_remainder >= 20;
            let sending_direction = y_rot_delta != 0.0 || x_rot_delta != 0.0;

            // if self.is_passenger() {
            //   TODO: posrot packet for being a passenger
            // }
            let flags = MoveFlags {
                on_ground: physics.on_ground(),
                horizontal_collision: physics.horizontal_collision,
            };
            let packet = if sending_position && sending_direction {
                Some(
                    ServerboundMovePlayerPosRot {
                        pos: **position,
                        look_direction: *direction,
                        flags,
                    }
                    .into_variant(),
                )
            } else if sending_position {
                Some(
                    ServerboundMovePlayerPos {
                        pos: **position,
                        flags,
                    }
                    .into_variant(),
                )
            } else if sending_direction {
                Some(
                    ServerboundMovePlayerRot {
                        look_direction: *direction,
                        flags,
                    }
                    .into_variant(),
                )
            } else if physics.last_on_ground() != physics.on_ground() {
                Some(ServerboundMovePlayerStatusOnly { flags }.into_variant())
            } else {
                None
            };

            if sending_position {
                **last_sent_position = **position;
                physics_state.position_remainder = 0;
            }
            if sending_direction {
                last_direction.y_rot = direction.y_rot();
                last_direction.x_rot = direction.x_rot();
            }

            let on_ground = physics.on_ground();
            physics.set_last_on_ground(on_ground);
            // minecraft checks for autojump here, but also autojump is bad so

            packet
        };

        if let Some(packet) = packet {
            commands.trigger(SendGamePacketEvent {
                sent_by: entity,
                packet,
            });
        }
    }
}

#[derive(Clone, Component, Debug, Default, Eq, PartialEq)]
pub struct LastSentInput(pub ServerboundPlayerInput);
pub fn send_player_input_packet(
    mut query: Query<(
        Entity,
        &ClientMovementState,
        &Jumping,
        Option<&LastSentInput>,
    )>,
    mut commands: Commands,
) {
    for (entity, physics_state, jumping, last_sent_input) in query.iter_mut() {
        let dir = physics_state.move_direction;
        let input = ServerboundPlayerInput {
            forward: dir.forward(),
            backward: dir.backward(),
            left: dir.left(),
            right: dir.right(),
            jump: **jumping,
            shift: physics_state.trying_to_crouch,
            sprint: physics_state.trying_to_sprint,
        };

        // if LastSentInput isn't present, we default to assuming we're not pressing any
        // keys and insert it anyways every time it changes
        let last_sent_input = last_sent_input.cloned().unwrap_or_default();

        if input != last_sent_input.0 {
            commands.trigger(SendGamePacketEvent {
                sent_by: entity,
                packet: input.clone().into_variant(),
            });
            commands.entity(entity).insert(LastSentInput(input));
        }
    }
}

pub fn send_sprinting_if_needed(
    mut query: Query<(
        Entity,
        &MinecraftEntityId,
        &Sprinting,
        &mut ClientMovementState,
    )>,
    mut commands: Commands,
) {
    for (entity, minecraft_entity_id, sprinting, mut physics_state) in query.iter_mut() {
        let was_sprinting = physics_state.was_sprinting;
        if **sprinting != was_sprinting {
            let sprinting_action = if **sprinting {
                s_player_command::Action::StartSprinting
            } else {
                s_player_command::Action::StopSprinting
            };
            commands.trigger(SendGamePacketEvent::new(
                entity,
                ServerboundPlayerCommand {
                    id: *minecraft_entity_id,
                    action: sprinting_action,
                    data: 0,
                },
            ));
            physics_state.was_sprinting = **sprinting;
        }
    }
}

/// Updates the [`PhysicsState::move_vector`] based on the
/// [`PhysicsState::move_direction`].
pub(crate) fn tick_controls(mut query: Query<&mut ClientMovementState>) {
    for mut physics_state in query.iter_mut() {
        let mut forward_impulse: f32 = 0.;
        let mut left_impulse: f32 = 0.;
        let move_direction = physics_state.move_direction;

        if move_direction.forward() {
            forward_impulse += 1.;
        } else if move_direction.backward() {
            forward_impulse -= 1.;
        }

        if move_direction.left() {
            left_impulse += 1.;
        } else if move_direction.right() {
            left_impulse -= 1.;
        }

        let move_vector = Vec2::new(left_impulse, forward_impulse).normalized();
        physics_state.move_vector = move_vector;
    }
}

/// Makes the bot do one physics tick.
///
/// This is handled automatically by the client.
#[allow(clippy::type_complexity)]
pub fn local_player_ai_step(
    mut query: Query<
        (
            Entity,
            &ClientMovementState,
            &PlayerAbilities,
            &metadata::Swimming,
            &metadata::SleepingPos,
            &WorldHolder,
            &Position,
            Option<&Hunger>,
            Option<&LastSentInput>,
            &FallFlying,
            &Pose,
            &mut Physics,
            &mut Sprinting,
            &mut Crouching,
            &mut Attributes,
        ),
        (With<HasClientLoaded>, With<LocalEntity>),
    >,
    aabb_query: AabbQuery,
    collidable_entity_query: CollidableEntityQuery,
) {
    for (
        entity,
        physics_state,
        abilities,
        swimming,
        sleeping_pos,
        world_holder,
        position,
        hunger,
        last_sent_input,
        fall_flying,
        pose,
        mut physics,
        mut sprinting,
        mut crouching,
        mut attributes,
    ) in query.iter_mut()
    {
        // server ai step

        let is_swimming = **swimming;
        // TODO: implement passengers
        let is_passenger = false;
        let is_sleeping = sleeping_pos.is_some();

        let world = world_holder.shared.read();
        let ctx = CanPlayerFitCtx {
            world: &world,
            entity,
            position: *position,
            aabb_query: &aabb_query,
            collidable_entity_query: &collidable_entity_query,
            physics: &physics,
        };

        let new_crouching = !abilities.flying
            && !is_swimming
            && !is_passenger
            && (last_sent_input.is_some_and(|i| i.0.shift)
                || !is_sleeping
                    && !can_player_fit_within_blocks_and_entities_when(&ctx, Pose::Standing))
            && can_player_fit_within_blocks_and_entities_when(&ctx, Pose::Crouching);
        if **crouching != new_crouching {
            **crouching = new_crouching;
        }

        // TODO: food data and abilities
        // let has_enough_food_to_sprint = self.food_data().food_level ||
        // self.abilities().may_fly;
        let has_enough_food_to_sprint = hunger.is_none_or(Hunger::is_enough_to_sprint);

        // TODO: double tapping w to sprint i think

        let trying_to_sprint = physics_state.trying_to_sprint;

        // TODO: swimming
        let is_underwater = false;
        let is_in_water = physics.is_in_water();

        let is_fall_flying = **fall_flying;
        // TODO: passenger
        let is_passenger = false;
        // TODO: using items
        let using_item = false;
        // TODO: status effects
        let has_blindness = false;

        let has_enough_impulse = has_enough_impulse_to_start_sprinting(physics_state);

        // LocalPlayer.canStartSprinting
        let can_start_sprinting = !**sprinting
            && has_enough_impulse
            && has_enough_food_to_sprint
            && !using_item
            && !has_blindness
            && (!is_passenger || is_underwater)
            && (!is_fall_flying || is_underwater)
            && (!is_moving_slowly(&crouching, fall_flying, pose, is_in_water) || is_underwater)
            && (!is_in_water || is_underwater);
        if trying_to_sprint && can_start_sprinting {
            set_sprinting(true, &mut sprinting, &mut attributes);
        }

        if **sprinting {
            // TODO: swimming

            let vehicle_can_sprint = false;
            // shouldStopRunSprinting
            let should_stop_sprinting = has_blindness
                || (is_passenger && !vehicle_can_sprint)
                || !has_enough_impulse
                || !has_enough_food_to_sprint
                || (physics.horizontal_collision && !physics.minor_horizontal_collision)
                || (is_in_water && !is_underwater);
            if should_stop_sprinting {
                set_sprinting(false, &mut sprinting, &mut attributes);
            }
        }

        // TODO: replace those booleans when using items and passengers are properly
        // implemented
        let move_vector = modify_input(
            physics_state.move_vector,
            false,
            false,
            is_moving_slowly(&crouching, fall_flying, pose, is_in_water),
            &attributes,
        );
        physics.x_acceleration = move_vector.x;
        physics.z_acceleration = move_vector.y;
    }
}

// this should technically be a step within local_player_ai_step, but
// 1. adds too much new query parameters if not extracted
// 2. is very local to interact with the elytra shared flag
// therefore I think it's safe to isolate into a separate system
pub fn process_fall_flying_activation(
    mut query: Query<
        (
            Entity,
            &MinecraftEntityId,
            &PlayerAbilities,
            Option<&LastSentInput>,
            &Jumping,
            &Inventory,
            &Physics,
            &OnClimbable,
            &mut FallFlying,
        ),
        (With<HasClientLoaded>, With<LocalEntity>),
    >,
    mut commands: Commands,
) {
    for (
        entity,
        minecraft_entity_id,
        abilities,
        last_sent_input,
        jumping,
        inv,
        physics,
        onclimbable,
        mut fall_flying,
    ) in query.iter_mut()
    {
        // TODO: creative fly toggle
        let creative_flight_toggled = false;

        if **jumping
            && !creative_flight_toggled
            && last_sent_input.is_some_and(|input| !input.0.jump)
            && !**onclimbable
            && can_start_fall_flying(&fall_flying, abilities, inv, physics)
        {
            // split `tryToStartFallFlying` into condition check
            **fall_flying = true; // Player.startFallFlying()
            commands.trigger(SendGamePacketEvent::new(
                entity,
                s_player_command::ServerboundPlayerCommand {
                    id: *minecraft_entity_id,
                    action: s_player_command::Action::StartFallFlying,
                    data: 0,
                },
            ));
        }
    }
}

// Player.tryToStartFallFlying()
fn can_start_fall_flying(
    already_fall_flying: &FallFlying,
    abilities: &PlayerAbilities,
    inv: &Inventory,
    physics: &Physics,
) -> bool {
    (!**already_fall_flying)
        && (!abilities.flying)

        // LivingEntity.canGlide()
        && !physics.on_ground()
        // TODO: && isPassenger()
        // TODO: slow falling status effect
        && EquipmentSlot::values().iter().any(|slot| {
            inv.get_equipment(*slot).is_some_and(|stack| {
                stack.get_component::<components::Glider>().is_some()
                    && stack.get_component::<components::Equippable>().is_some_and(
                        // TODO: check eltra durability
                        |equippable| equippable.slot == *slot/* && stack.nextDamageWillBreak() */
                    )
            })
        })

        && !physics.is_in_water()
}

// LocalPlayer.isMovingSlowly
fn is_moving_slowly(
    crouching: &Crouching,
    fall_flying: &FallFlying,
    pose: &Pose,
    is_in_water: bool,
) -> bool {
    if **crouching {
        return true;
    }

    // Entity.isVisuallyCrawling
    if is_in_water {
        return false;
    }

    // LivingEntity.isVisuallySwimming override
    match *pose {
        Pose::Swimming => true,
        Pose::FallFlying => !**fall_flying,
        // There is going to be a slowdown for a tick when the server sets the Fallflying shared
        // flag while the client is still in the FallFlying Pose And that is totally
        // intended
        _ => false,
    }
}

// LocalPlayer.modifyInput
fn modify_input(
    mut move_vector: Vec2,
    is_using_item: bool,
    is_passenger: bool,
    moving_slowly: bool,
    attributes: &Attributes,
) -> Vec2 {
    if move_vector.length_squared() == 0. {
        return move_vector;
    }

    move_vector *= 0.98;
    if is_using_item && !is_passenger {
        move_vector *= 0.2;
    }

    if moving_slowly {
        let sneaking_speed = attributes.sneaking_speed.calculate() as f32;
        move_vector *= sneaking_speed;
    }

    modify_input_speed_for_square_movement(move_vector)
}
fn modify_input_speed_for_square_movement(move_vector: Vec2) -> Vec2 {
    let length = move_vector.length();
    if length == 0. {
        return move_vector;
    }
    let scaled_to_inverse_length = move_vector * (1. / length);
    let dist = distance_to_unit_square(scaled_to_inverse_length);
    let scale = (length * dist).min(1.);
    scaled_to_inverse_length * scale
}
fn distance_to_unit_square(v: Vec2) -> f32 {
    let x = v.x.abs();
    let y = v.y.abs();
    let ratio = if y > x { x / y } else { y / x };
    (1. + ratio * ratio).sqrt()
}

/// An event sent when the client starts walking.
///
/// This does not get sent for non-local entities.
///
/// To stop walking or sprinting, send this event with `WalkDirection::None`.
#[derive(Debug, Message)]
pub struct StartWalkEvent {
    pub entity: Entity,
    pub direction: WalkDirection,
}

/// The system that makes the player start walking when they receive a
/// [`StartWalkEvent`].
pub fn handle_walk(
    mut events: MessageReader<StartWalkEvent>,
    mut query: Query<(&mut ClientMovementState, &mut Sprinting, &mut Attributes)>,
) {
    for event in events.read() {
        if let Ok((mut physics_state, mut sprinting, mut attributes)) = query.get_mut(event.entity)
        {
            physics_state.move_direction = event.direction;
            physics_state.trying_to_sprint = false;
            set_sprinting(false, &mut sprinting, &mut attributes);
        }
    }
}

/// An event sent when the client starts sprinting.
///
/// This does not get sent for non-local entities.
#[derive(Message)]
pub struct StartSprintEvent {
    pub entity: Entity,
    pub direction: SprintDirection,
}
/// The system that makes the player start sprinting when they receive a
/// [`StartSprintEvent`].
pub fn handle_sprint(
    mut query: Query<&mut ClientMovementState>,
    mut events: MessageReader<StartSprintEvent>,
) {
    for event in events.read() {
        if let Ok(mut physics_state) = query.get_mut(event.entity) {
            physics_state.move_direction = WalkDirection::from(event.direction);
            physics_state.trying_to_sprint = true;
        }
    }
}

/// Change whether we're sprinting by adding an attribute modifier to the
/// player.
///
/// You should use the [`Client::walk`] and [`Client::sprint`] functions
/// instead.
///
/// Returns true if the operation was successful.
fn set_sprinting(
    sprinting: bool,
    currently_sprinting: &mut Sprinting,
    attributes: &mut Attributes,
) -> bool {
    **currently_sprinting = sprinting;
    if sprinting {
        attributes
            .movement_speed
            .try_insert(azalea_entity::attributes::sprinting_modifier())
            .is_ok()
    } else {
        attributes
            .movement_speed
            .remove(&azalea_entity::attributes::sprinting_modifier().id)
            .is_none()
    }
}

// Whether the player is moving fast enough to be able to start sprinting.
fn has_enough_impulse_to_start_sprinting(physics_state: &ClientMovementState) -> bool {
    // if self.underwater() {
    //     self.has_forward_impulse()
    // } else {
    physics_state.move_vector.y > 0.8
    // }
}

/// An event sent by the server that sets or adds to our velocity.
///
/// Usually `KnockbackKind::Set` is used for normal knockback and
/// `KnockbackKind::Add` is used for explosions, but some servers (notably
/// Hypixel) use explosions for knockback.
#[derive(EntityEvent, Debug, Clone)]
pub struct KnockbackEvent {
    pub entity: Entity,
    pub data: KnockbackData,
}

#[derive(Debug, Clone)]
pub enum KnockbackData {
    Set(Vec3),
    Add(Vec3),
}

pub fn handle_knockback(knockback: On<KnockbackEvent>, mut query: Query<&mut Physics>) {
    if let Ok(mut physics) = query.get_mut(knockback.entity) {
        match knockback.data {
            KnockbackData::Set(velocity) => {
                physics.velocity = velocity;
            }
            KnockbackData::Add(velocity) => {
                physics.velocity += velocity;
            }
        }
    }
}

pub fn update_pose(
    mut query: Query<(
        Entity,
        &mut Pose,
        &Physics,
        &ClientMovementState,
        &FallFlying,
        &GameMode,
        &WorldHolder,
        &Position,
    )>,
    aabb_query: AabbQuery,
    collidable_entity_query: CollidableEntityQuery,
) {
    for (
        entity,
        mut pose,
        physics,
        physics_state,
        fall_flying,
        &game_mode,
        world_holder,
        position,
    ) in query.iter_mut()
    {
        let world = world_holder.shared.read();
        let world = &*world;
        let ctx = CanPlayerFitCtx {
            world,
            entity,
            position: *position,
            aabb_query: &aabb_query,
            collidable_entity_query: &collidable_entity_query,
            physics,
        };

        if !can_player_fit_within_blocks_and_entities_when(&ctx, Pose::Swimming) {
            continue;
        }

        // TODO: implement everything else from getDesiredPose: sleeping, swimming,
        // fallFlying, spinAttack
        let desired_pose = if physics_state.trying_to_crouch {
            Pose::Crouching
        } else if **fall_flying {
            Pose::FallFlying
        } else {
            Pose::Standing
        };

        // TODO: passengers
        let is_passenger = false;

        // canPlayerFitWithinBlocksAndEntitiesWhen
        let new_pose = if game_mode == GameMode::Spectator
            || is_passenger
            || can_player_fit_within_blocks_and_entities_when(&ctx, desired_pose)
        {
            desired_pose
        } else if can_player_fit_within_blocks_and_entities_when(&ctx, Pose::Crouching) {
            Pose::Crouching
        } else {
            Pose::Swimming
        };

        // avoid triggering change detection
        if new_pose != *pose {
            *pose = new_pose;
        }
    }
}

struct CanPlayerFitCtx<'world, 'state, 'a, 'b> {
    world: &'a World,
    entity: Entity,
    position: Position,
    aabb_query: &'a AabbQuery<'world, 'state, 'b>,
    collidable_entity_query: &'a CollidableEntityQuery<'world, 'state>,
    physics: &'a Physics,
}
fn can_player_fit_within_blocks_and_entities_when(ctx: &CanPlayerFitCtx, pose: Pose) -> bool {
    no_collision(
        ctx.world,
        Some(ctx.entity),
        ctx.aabb_query,
        ctx.collidable_entity_query,
        ctx.physics,
        &calculate_dimensions(EntityKind::Player, pose)
            .make_bounding_box(*ctx.position)
            .deflate_all(1.0e-7),
        false,
    )
}
