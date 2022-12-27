use std::backtrace::Backtrace;

use crate::{Client, PhysicsState};
use azalea_core::Vec3;
use azalea_physics::collision::{move_colliding, MoverType};
use azalea_protocol::packets::game::serverbound_player_command_packet::ServerboundPlayerCommandPacket;
use azalea_protocol::packets::game::{
    serverbound_move_player_pos_packet::ServerboundMovePlayerPosPacket,
    serverbound_move_player_pos_rot_packet::ServerboundMovePlayerPosRotPacket,
    serverbound_move_player_rot_packet::ServerboundMovePlayerRotPacket,
    serverbound_move_player_status_only_packet::ServerboundMovePlayerStatusOnlyPacket,
};
use azalea_world::{entity, MoveEntityError};
use bevy_ecs::system::Query;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MovePlayerError {
    #[error("Player is not in world")]
    PlayerNotInWorld(Backtrace),
    #[error("{0}")]
    Io(#[from] std::io::Error),
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

impl Client {
    /// This gets called automatically every tick.
    pub(crate) fn send_position(
        &mut self,
        mut query: Query<(
            &entity::Position,
            &mut entity::Physics,
            &entity::metadata::Sprinting,
        )>,
    ) -> Result<(), MovePlayerError> {
        let (player_pos, mut physics, sprinting) = query
            .get_mut((*self.entity_id.read()).into())
            .expect("Player should always be in world");

        let mut physics_state = self.physics_state.lock();

        self.send_sprinting_if_needed(sprinting, &mut physics_state)?;

        let packet = {
            // TODO: the camera being able to be controlled by other entities isn't
            // implemented yet if !self.is_controlled_camera() { return };

            // i don't like this
            let entity_storage_lock = self.world().entities.clone();
            let mut entity_storage = entity_storage_lock.write();
            let (player_pos, mut physics) = entity_storage
                .query_entity_mut::<(&entity::Position, &mut entity::Physics)>(
                    *self.entity_id.read(),
                );

            let player_old_pos = physics.last_pos;

            let x_delta = player_pos.x - player_old_pos.x;
            let y_delta = player_pos.y - player_old_pos.y;
            let z_delta = player_pos.z - player_old_pos.z;
            let y_rot_delta = (physics.y_rot - physics.y_rot_last) as f64;
            let x_rot_delta = (physics.x_rot - physics.x_rot_last) as f64;

            physics_state.position_remainder += 1;

            // boolean sendingPosition = Mth.lengthSquared(xDelta, yDelta, zDelta) >
            // Mth.square(2.0E-4D) || this.positionReminder >= 20;
            let sending_position = ((x_delta.powi(2) + y_delta.powi(2) + z_delta.powi(2))
                > 2.0e-4f64.powi(2))
                || physics_state.position_remainder >= 20;
            let sending_rotation = y_rot_delta != 0.0 || x_rot_delta != 0.0;

            // if self.is_passenger() {
            //   TODO: posrot packet for being a passenger
            // }
            let packet = if sending_position && sending_rotation {
                Some(
                    ServerboundMovePlayerPosRotPacket {
                        x: player_pos.x,
                        y: player_pos.y,
                        z: player_pos.z,
                        x_rot: physics.x_rot,
                        y_rot: physics.y_rot,
                        on_ground: physics.on_ground,
                    }
                    .get(),
                )
            } else if sending_position {
                Some(
                    ServerboundMovePlayerPosPacket {
                        x: player_pos.x,
                        y: player_pos.y,
                        z: player_pos.z,
                        on_ground: physics.on_ground,
                    }
                    .get(),
                )
            } else if sending_rotation {
                Some(
                    ServerboundMovePlayerRotPacket {
                        x_rot: physics.x_rot,
                        y_rot: physics.y_rot,
                        on_ground: physics.on_ground,
                    }
                    .get(),
                )
            } else if physics.last_on_ground != physics.on_ground {
                Some(
                    ServerboundMovePlayerStatusOnlyPacket {
                        on_ground: physics.on_ground,
                    }
                    .get(),
                )
            } else {
                None
            };

            if sending_position {
                physics.last_pos = **player_pos;
                physics_state.position_remainder = 0;
            }
            if sending_rotation {
                physics.y_rot_last = physics.y_rot;
                physics.x_rot_last = physics.x_rot;
            }

            physics.last_on_ground = physics.on_ground;
            // minecraft checks for autojump here, but also autojump is bad so

            packet
        };

        if let Some(packet) = packet {
            tokio::spawn(self.write_packet(packet));
        }

        Ok(())
    }

    fn send_sprinting_if_needed(
        &mut self,
        sprinting: &entity::metadata::Sprinting,
        physics_state: &mut PhysicsState,
    ) -> Result<(), MovePlayerError> {
        let was_sprinting = physics_state.was_sprinting;
        if **sprinting != was_sprinting {
            let sprinting_action = if **sprinting {
                azalea_protocol::packets::game::serverbound_player_command_packet::Action::StartSprinting
            } else {
                azalea_protocol::packets::game::serverbound_player_command_packet::Action::StopSprinting
            };
            let player_entity_id = *self.entity_id.read();
            tokio::spawn(
                self.write_packet(
                    ServerboundPlayerCommandPacket {
                        id: *player_entity_id,
                        action: sprinting_action,
                        data: 0,
                    }
                    .get(),
                ),
            );
            physics_state.was_sprinting = **sprinting;
        }

        Ok(())
    }

    // Set our current position to the provided Vec3, potentially clipping through
    // blocks.
    pub async fn set_position(&mut self, new_pos: Vec3) -> Result<(), MovePlayerError> {
        let player_entity_id = *self.entity_id.read();
        let mut world_lock = self.world.write();

        world_lock.set_entity_pos(player_entity_id, new_pos)?;

        Ok(())
    }

    /// Makes the bot do one physics tick. Note that this is already handled
    /// automatically by the client.
    pub fn ai_step(
        &mut self,
        query: Query<(
            &mut entity::Physics,
            &mut entity::Position,
            &mut entity::metadata::Sprinting,
            &entity::Attributes,
        )>,
    ) {
        self.tick_controls(None);

        let (mut physics, mut position, mut sprinting, attributes) =
            query.get_mut((*self.entity_id.read()).into()).unwrap();

        // server ai step
        {
            let physics_state = self.physics_state.lock();
            physics.xxa = physics_state.left_impulse;
            physics.zza = physics_state.forward_impulse;
        }

        // TODO: food data and abilities
        // let has_enough_food_to_sprint = self.food_data().food_level ||
        // self.abilities().may_fly;
        let has_enough_food_to_sprint = true;

        // TODO: double tapping w to sprint i think

        let trying_to_sprint = self.physics_state.lock().trying_to_sprint;

        if !self.sprinting()
            && (
                // !self.is_in_water()
                // || self.is_underwater() &&
                self.has_enough_impulse_to_start_sprinting()
                    && has_enough_food_to_sprint
                    // && !self.using_item()
                    // && !self.has_effect(MobEffects.BLINDNESS)
                    && trying_to_sprint
            )
        {
            self.set_sprinting(true);
        }

        azalea_physics::ai_step(
            *self.entity_id.read(),
            &self.world(),
            &mut physics,
            &mut position,
            &sprinting,
            attributes,
        );
    }

    /// Update the impulse from self.move_direction. The multipler is used for
    /// sneaking.
    pub(crate) fn tick_controls(&mut self, multiplier: Option<f32>) {
        let mut physics_state = self.physics_state.lock();

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
        physics_state.forward_impulse = forward_impulse;
        physics_state.left_impulse = left_impulse;

        if let Some(multiplier) = multiplier {
            physics_state.forward_impulse *= multiplier;
            physics_state.left_impulse *= multiplier;
        }
    }

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
    pub fn walk(&mut self, direction: WalkDirection) {
        {
            let mut physics_state = self.physics_state.lock();
            physics_state.move_direction = direction;
        }

        self.set_sprinting(false);
    }

    /// Start sprinting in the given direction. To stop moving, call
    /// [`Client::walk(WalkDirection::None)`]
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
    pub fn sprint(&mut self, direction: SprintDirection) {
        let mut physics_state = self.physics_state.lock();
        physics_state.move_direction = WalkDirection::from(direction);
        physics_state.trying_to_sprint = true;
    }

    // Whether we're currently sprinting.
    pub fn sprinting(&self) -> bool {
        self.entity().metadata.sprinting
    }

    /// Change whether we're sprinting by adding an attribute modifier to the
    /// player. You should use the [`walk`] and [`sprint`] methods instead.
    /// Returns if the operation was successful.
    fn set_sprinting(&mut self, sprinting: bool) -> bool {
        let mut player_entity = self.entity();
        player_entity.metadata.sprinting = sprinting;
        if sprinting {
            player_entity
                .attributes
                .speed
                .insert(azalea_world::entity::attributes::sprinting_modifier())
                .is_ok()
        } else {
            player_entity
                .attributes
                .speed
                .remove(&azalea_world::entity::attributes::sprinting_modifier().uuid)
                .is_none()
        }
    }

    /// Set whether we're jumping. This acts as if you held space in
    /// vanilla. If you want to jump once, use the `jump` function.
    ///
    /// If you're making a realistic client, calling this function every tick is
    /// recommended.
    pub fn set_jumping(&mut self, jumping: bool) {
        let mut player_entity = self.entity();
        let physics = self.query::<Query<&entity::Physics>>();
        player_entity.jumping = jumping;
    }

    /// Returns whether the player will try to jump next tick.
    pub fn jumping(&self) -> bool {
        let player_entity = self.entity();
        player_entity.jumping
    }

    /// Sets your rotation. `y_rot` is yaw (looking to the side), `x_rot` is
    /// pitch (looking up and down). You can get these numbers from the vanilla
    /// f3 screen.
    /// `y_rot` goes from -180 to 180, and `x_rot` goes from -90 to 90.
    pub fn set_rotation(&mut self, y_rot: f32, x_rot: f32) {
        let mut player_entity = self.entity();
        player_entity.set_rotation(y_rot, x_rot);
    }

    // Whether the player is moving fast enough to be able to start sprinting.
    fn has_enough_impulse_to_start_sprinting(&self) -> bool {
        // if self.underwater() {
        //     self.has_forward_impulse()
        // } else {
        let physics_state = self.physics_state.lock();
        physics_state.forward_impulse > 0.8
        // }
    }
}

#[derive(Clone, Copy, Debug, Default)]
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
