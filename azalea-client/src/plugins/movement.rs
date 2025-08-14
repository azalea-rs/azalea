use std::{backtrace::Backtrace, io};

use azalea_core::{
    game_type::GameMode,
    position::{Vec2, Vec3},
    tick::GameTick,
};
use azalea_entity::{
    Attributes, Crouching, HasClientLoaded, Jumping, LastSentPosition, LocalEntity, LookDirection,
    Physics, PlayerAbilities, Pose, Position,
    dimensions::calculate_dimensions,
    metadata::{self, Sprinting},
    update_bounding_box,
};
use azalea_physics::{
    PhysicsSet, ai_step,
    collision::entity_collisions::{CollidableEntityQuery, PhysicsQuery},
    travel::no_collision,
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
            s_move_player_status_only::ServerboundMovePlayerStatusOnly,
        },
    },
};
use azalea_registry::EntityKind;
use azalea_world::{Instance, MinecraftEntityId, MoveEntityError};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use thiserror::Error;

use crate::{
    client::Client,
    local_player::{InstanceHolder, LocalGameMode},
    packet::game::SendPacketEvent,
};

#[derive(Error, Debug)]
pub enum MovePlayerError {
    #[error("Player is not in world")]
    PlayerNotInWorld(Backtrace),
    #[error("{0}")]
    Io(#[from] io::Error),
}

impl From<MoveEntityError> for MovePlayerError {
    fn from(err: MoveEntityError) -> Self {
        match err {
            MoveEntityError::EntityDoesNotExist(backtrace) => {
                MovePlayerError::PlayerNotInWorld(backtrace)
            }
        }
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartWalkEvent>()
            .add_event::<StartSprintEvent>()
            .add_event::<KnockbackEvent>()
            .add_systems(
                Update,
                (handle_sprint, handle_walk, handle_knockback)
                    .chain()
                    .in_set(MoveEventsSet)
                    .after(update_bounding_box),
            )
            .add_systems(
                GameTick,
                (
                    (tick_controls, local_player_ai_step, update_pose)
                        .chain()
                        .in_set(PhysicsSet)
                        .before(ai_step)
                        .before(azalea_physics::fluids::update_in_water_state_and_do_fluid_pushing),
                    send_player_input_packet,
                    send_sprinting_if_needed.after(azalea_entity::update_in_loaded_chunk),
                    send_position.after(PhysicsSet),
                )
                    .chain(),
            );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MoveEventsSet;

impl Client {
    /// Set whether we're jumping. This acts as if you held space in
    /// vanilla. If you want to jump once, use the `jump` function.
    ///
    /// If you're making a realistic client, calling this function every tick is
    /// recommended.
    pub fn set_jumping(&self, jumping: bool) {
        let mut ecs = self.ecs.lock();
        let mut jumping_mut = self.query::<&mut Jumping>(&mut ecs);
        **jumping_mut = jumping;
    }

    /// Returns whether the player will try to jump next tick.
    pub fn jumping(&self) -> bool {
        *self.component::<Jumping>()
    }

    pub fn set_crouching(&self, crouching: bool) {
        let mut ecs = self.ecs.lock();
        let mut physics_state = self.query::<&mut PhysicsState>(&mut ecs);
        physics_state.trying_to_crouch = crouching;
    }

    /// Whether the client is currently trying to sneak.
    ///
    /// You may want to check the [`Pose`] instead.
    pub fn crouching(&self) -> bool {
        let mut ecs = self.ecs.lock();
        let physics_state = self.query::<&PhysicsState>(&mut ecs);
        physics_state.trying_to_crouch
    }

    /// Sets the direction the client is looking. `y_rot` is yaw (looking to the
    /// side), `x_rot` is pitch (looking up and down). You can get these
    /// numbers from the vanilla f3 screen.
    /// `y_rot` goes from -180 to 180, and `x_rot` goes from -90 to 90.
    pub fn set_direction(&self, y_rot: f32, x_rot: f32) {
        let mut ecs = self.ecs.lock();
        let mut look_direction = self.query::<&mut LookDirection>(&mut ecs);

        look_direction.update(LookDirection::new(y_rot, x_rot));
    }

    /// Returns the direction the client is looking. The first value is the y
    /// rotation (ie. yaw, looking to the side) and the second value is the x
    /// rotation (ie. pitch, looking up and down).
    pub fn direction(&self) -> (f32, f32) {
        let look_direction: LookDirection = self.component::<LookDirection>();
        (look_direction.y_rot(), look_direction.x_rot())
    }
}

/// A component that contains the look direction that was last sent over the
/// network.
#[derive(Debug, Component, Clone, Default)]
pub struct LastSentLookDirection {
    pub x_rot: f32,
    pub y_rot: f32,
}

/// Component for entities that can move and sprint. Usually only in
/// [`LocalEntity`]s.
///
/// [`LocalEntity`]: azalea_entity::LocalEntity
#[derive(Default, Component, Clone)]
pub struct PhysicsState {
    /// Minecraft only sends a movement packet either after 20 ticks or if the
    /// player moved enough. This is that tick counter.
    pub position_remainder: u32,
    pub was_sprinting: bool,
    // Whether we're going to try to start sprinting this tick. Equivalent to
    // holding down ctrl for a tick.
    pub trying_to_sprint: bool,

    /// Whether our player is currently trying to sneak.
    ///
    /// This is distinct from
    /// [`AbstractEntityShiftKeyDown`](azalea_entity::metadata::AbstractEntityShiftKeyDown),
    /// which is a metadata value that is controlled by the server and affects
    /// how the nametags of other entities are displayed.
    ///
    /// To check whether we're actually sneaking, you can check the
    /// [`Crouching`] or [`Pose`] components.
    pub trying_to_crouch: bool,

    pub move_direction: WalkDirection,
    pub move_vector: Vec2,
}

#[allow(clippy::type_complexity)]
pub fn send_position(
    mut query: Query<
        (
            Entity,
            &Position,
            &LookDirection,
            &mut PhysicsState,
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
            commands.trigger(SendPacketEvent {
                sent_by: entity,
                packet,
            });
        }
    }
}

#[derive(Debug, Default, Component, Clone, PartialEq, Eq)]
pub struct LastSentInput(pub ServerboundPlayerInput);
pub fn send_player_input_packet(
    mut query: Query<(
        Entity,
        &PhysicsState,
        &Jumping,
        &Crouching,
        Option<&LastSentInput>,
    )>,
    mut commands: Commands,
) {
    for (entity, physics_state, jumping, sneaking, last_sent_input) in query.iter_mut() {
        let dir = physics_state.move_direction;
        type D = WalkDirection;
        let input = ServerboundPlayerInput {
            forward: matches!(dir, D::Forward | D::ForwardLeft | D::ForwardRight),
            backward: matches!(dir, D::Backward | D::BackwardLeft | D::BackwardRight),
            left: matches!(dir, D::Left | D::ForwardLeft | D::BackwardLeft),
            right: matches!(dir, D::Right | D::ForwardRight | D::BackwardRight),
            jump: **jumping,
            shift: **sneaking,
            sprint: physics_state.trying_to_sprint,
        };

        // if LastSentInput isn't present, we default to assuming we're not pressing any
        // keys and insert it anyways every time it changes
        let last_sent_input = last_sent_input.cloned().unwrap_or_default();

        if input != last_sent_input.0 {
            commands.trigger(SendPacketEvent {
                sent_by: entity,
                packet: input.clone().into_variant(),
            });
            commands.entity(entity).insert(LastSentInput(input));
        }
    }
}

pub fn send_sprinting_if_needed(
    mut query: Query<(Entity, &MinecraftEntityId, &Sprinting, &mut PhysicsState)>,
    mut commands: Commands,
) {
    for (entity, minecraft_entity_id, sprinting, mut physics_state) in query.iter_mut() {
        let was_sprinting = physics_state.was_sprinting;
        if **sprinting != was_sprinting {
            let sprinting_action = if **sprinting {
                azalea_protocol::packets::game::s_player_command::Action::StartSprinting
            } else {
                azalea_protocol::packets::game::s_player_command::Action::StopSprinting
            };
            commands.trigger(SendPacketEvent::new(
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
pub(crate) fn tick_controls(mut query: Query<&mut PhysicsState>) {
    for mut physics_state in query.iter_mut() {
        let mut forward_impulse: f32 = 0.;
        let mut left_impulse: f32 = 0.;
        let move_direction = physics_state.move_direction;
        match move_direction {
            WalkDirection::Forward | WalkDirection::ForwardRight | WalkDirection::ForwardLeft => {
                forward_impulse += 1.;
            }
            WalkDirection::Backward
            | WalkDirection::BackwardRight
            | WalkDirection::BackwardLeft => {
                forward_impulse -= 1.;
            }
            _ => {}
        };
        match move_direction {
            WalkDirection::Right | WalkDirection::ForwardRight | WalkDirection::BackwardRight => {
                left_impulse += 1.;
            }
            WalkDirection::Left | WalkDirection::ForwardLeft | WalkDirection::BackwardLeft => {
                left_impulse -= 1.;
            }
            _ => {}
        };

        let move_vector = Vec2::new(left_impulse, forward_impulse).normalized();
        physics_state.move_vector = move_vector;
    }
}

/// Makes the bot do one physics tick. Note that this is already handled
/// automatically by the client.
#[allow(clippy::type_complexity)]
pub fn local_player_ai_step(
    mut query: Query<
        (
            Entity,
            &PhysicsState,
            &PlayerAbilities,
            &metadata::Swimming,
            &metadata::SleepingPos,
            &InstanceHolder,
            &Position,
            &mut Physics,
            &mut Sprinting,
            &mut Crouching,
            &mut Attributes,
        ),
        (With<HasClientLoaded>, With<LocalEntity>),
    >,
    physics_query: PhysicsQuery,
    collidable_entity_query: CollidableEntityQuery,
) {
    for (
        entity,
        physics_state,
        abilities,
        swimming,
        sleeping_pos,
        instance_holder,
        position,
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

        let world = instance_holder.instance.read();
        let ctx = CanPlayerFitCtx {
            world: &world,
            entity,
            position: *position,
            physics_query: &physics_query,
            collidable_entity_query: &collidable_entity_query,
            physics: &physics,
        };

        let new_crouching = !abilities.flying
            && !is_swimming
            && !is_passenger
            && can_player_fit_within_blocks_and_entities_when(&ctx, Pose::Crouching)
            && (physics_state.trying_to_crouch
                || !is_sleeping
                    && !can_player_fit_within_blocks_and_entities_when(&ctx, Pose::Standing));
        if **crouching != new_crouching {
            **crouching = new_crouching;
        }

        // TODO: replace those booleans when using items and passengers are properly
        // implemented
        let move_vector = modify_input(
            physics_state.move_vector,
            false,
            false,
            **crouching,
            &attributes,
        );
        physics.x_acceleration = move_vector.x;
        physics.z_acceleration = move_vector.y;

        // TODO: food data and abilities
        // let has_enough_food_to_sprint = self.food_data().food_level ||
        // self.abilities().may_fly;
        let has_enough_food_to_sprint = true;

        // TODO: double tapping w to sprint i think

        let trying_to_sprint = physics_state.trying_to_sprint;

        if !**sprinting
            && (
                // !self.is_in_water()
                // || self.is_underwater() &&
                has_enough_impulse_to_start_sprinting(physics_state)
                    && has_enough_food_to_sprint
                    // && !self.using_item()
                    // && !self.has_effect(MobEffects.BLINDNESS)
                    && trying_to_sprint
            )
        {
            set_sprinting(true, &mut sprinting, &mut attributes);
        }
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

impl Client {
    /// Start walking in the given direction. To sprint, use
    /// [`Client::sprint`]. To stop walking, call walk with
    /// `WalkDirection::None`.
    ///
    /// # Examples
    ///
    /// Walk for 1 second
    /// ```rust,no_run
    /// # use azalea_client::{Client, WalkDirection};
    /// # use std::time::Duration;
    /// # async fn example(mut bot: Client) {
    /// bot.walk(WalkDirection::Forward);
    /// tokio::time::sleep(Duration::from_secs(1)).await;
    /// bot.walk(WalkDirection::None);
    /// # }
    /// ```
    pub fn walk(&self, direction: WalkDirection) {
        let mut ecs = self.ecs.lock();
        ecs.send_event(StartWalkEvent {
            entity: self.entity,
            direction,
        });
    }

    /// Start sprinting in the given direction. To stop moving, call
    /// [`bot.walk(WalkDirection::None)`](Self::walk)
    ///
    /// # Examples
    ///
    /// Sprint for 1 second
    /// ```rust,no_run
    /// # use azalea_client::{Client, WalkDirection, SprintDirection};
    /// # use std::time::Duration;
    /// # async fn example(mut bot: Client) {
    /// bot.sprint(SprintDirection::Forward);
    /// tokio::time::sleep(Duration::from_secs(1)).await;
    /// bot.walk(WalkDirection::None);
    /// # }
    /// ```
    pub fn sprint(&self, direction: SprintDirection) {
        let mut ecs = self.ecs.lock();
        ecs.send_event(StartSprintEvent {
            entity: self.entity,
            direction,
        });
    }
}

/// An event sent when the client starts walking. This does not get sent for
/// non-local entities.
///
/// To stop walking or sprinting, send this event with `WalkDirection::None`.
#[derive(Event, Debug)]
pub struct StartWalkEvent {
    pub entity: Entity,
    pub direction: WalkDirection,
}

/// The system that makes the player start walking when they receive a
/// [`StartWalkEvent`].
pub fn handle_walk(
    mut events: EventReader<StartWalkEvent>,
    mut query: Query<(&mut PhysicsState, &mut Sprinting, &mut Attributes)>,
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

/// An event sent when the client starts sprinting. This does not get sent for
/// non-local entities.
#[derive(Event)]
pub struct StartSprintEvent {
    pub entity: Entity,
    pub direction: SprintDirection,
}
/// The system that makes the player start sprinting when they receive a
/// [`StartSprintEvent`].
pub fn handle_sprint(
    mut query: Query<&mut PhysicsState>,
    mut events: EventReader<StartSprintEvent>,
) {
    for event in events.read() {
        if let Ok(mut physics_state) = query.get_mut(event.entity) {
            physics_state.move_direction = WalkDirection::from(event.direction);
            physics_state.trying_to_sprint = true;
        }
    }
}

/// Change whether we're sprinting by adding an attribute modifier to the
/// player. You should use the [`walk`] and [`sprint`] methods instead.
/// Returns if the operation was successful.
fn set_sprinting(
    sprinting: bool,
    currently_sprinting: &mut Sprinting,
    attributes: &mut Attributes,
) -> bool {
    **currently_sprinting = sprinting;
    if sprinting {
        attributes
            .speed
            .try_insert(azalea_entity::attributes::sprinting_modifier())
            .is_ok()
    } else {
        attributes
            .speed
            .remove(&azalea_entity::attributes::sprinting_modifier().id)
            .is_none()
    }
}

// Whether the player is moving fast enough to be able to start sprinting.
fn has_enough_impulse_to_start_sprinting(physics_state: &PhysicsState) -> bool {
    // if self.underwater() {
    //     self.has_forward_impulse()
    // } else {
    physics_state.move_vector.y > 0.8
    // }
}

/// An event sent by the server that sets or adds to our velocity. Usually
/// `KnockbackKind::Set` is used for normal knockback and `KnockbackKind::Add`
/// is used for explosions, but some servers (notably Hypixel) use explosions
/// for knockback.
#[derive(Event)]
pub struct KnockbackEvent {
    pub entity: Entity,
    pub knockback: KnockbackType,
}

pub enum KnockbackType {
    Set(Vec3),
    Add(Vec3),
}

pub fn handle_knockback(mut query: Query<&mut Physics>, mut events: EventReader<KnockbackEvent>) {
    for event in events.read() {
        if let Ok(mut physics) = query.get_mut(event.entity) {
            match event.knockback {
                KnockbackType::Set(velocity) => {
                    physics.velocity = velocity;
                }
                KnockbackType::Add(velocity) => {
                    physics.velocity += velocity;
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WalkDirection {
    #[default]
    None,
    Forward,
    Backward,
    Left,
    Right,
    ForwardRight,
    ForwardLeft,
    BackwardRight,
    BackwardLeft,
}

/// The directions that we can sprint in. It's a subset of [`WalkDirection`].
#[derive(Clone, Copy, Debug)]
pub enum SprintDirection {
    Forward,
    ForwardRight,
    ForwardLeft,
}

impl From<SprintDirection> for WalkDirection {
    fn from(d: SprintDirection) -> Self {
        match d {
            SprintDirection::Forward => WalkDirection::Forward,
            SprintDirection::ForwardRight => WalkDirection::ForwardRight,
            SprintDirection::ForwardLeft => WalkDirection::ForwardLeft,
        }
    }
}

pub fn update_pose(
    mut query: Query<(
        Entity,
        &mut Pose,
        &Physics,
        &Crouching,
        &LocalGameMode,
        &InstanceHolder,
        &Position,
    )>,
    physics_query: PhysicsQuery,
    collidable_entity_query: CollidableEntityQuery,
) {
    for (entity, mut pose, physics, crouching, game_mode, instance_holder, position) in
        query.iter_mut()
    {
        let world = instance_holder.instance.read();
        let world = &*world;
        let ctx = CanPlayerFitCtx {
            world,
            entity,
            position: *position,
            physics_query: &physics_query,
            collidable_entity_query: &collidable_entity_query,
            physics,
        };

        if !can_player_fit_within_blocks_and_entities_when(&ctx, Pose::Swimming) {
            continue;
        }

        // TODO: implement everything else from getDesiredPose: sleeping, swimming,
        // fallFlying, spinAttack
        let desired_pose = if **crouching {
            Pose::Crouching
        } else {
            Pose::Standing
        };

        // TODO: passengers
        let is_passenger = false;

        // canPlayerFitWithinBlocksAndEntitiesWhen
        let new_pose = if game_mode.current == GameMode::Spectator
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
    world: &'a Instance,
    entity: Entity,
    position: Position,
    physics_query: &'a PhysicsQuery<'world, 'state, 'b>,
    collidable_entity_query: &'a CollidableEntityQuery<'world, 'state>,
    physics: &'a Physics,
}
fn can_player_fit_within_blocks_and_entities_when(ctx: &CanPlayerFitCtx, pose: Pose) -> bool {
    // return this.level().noCollision(this,
    // this.getDimensions(var1).makeBoundingBox(this.position()).deflate(1.0E-7));
    no_collision(
        ctx.world,
        Some(ctx.entity),
        ctx.physics_query,
        ctx.collidable_entity_query,
        ctx.physics,
        &calculate_dimensions(EntityKind::Player, pose).make_bounding_box(*ctx.position),
        false,
    )
}
