use crate::client::{Client, LocalPlayerInLoadedChunk};
use crate::{LocalPlayer, PhysicsState};
use azalea_core::Vec3;
use azalea_protocol::packets::game::serverbound_player_command_packet::ServerboundPlayerCommandPacket;
use azalea_protocol::packets::game::{
    serverbound_move_player_pos_packet::ServerboundMovePlayerPosPacket,
    serverbound_move_player_pos_rot_packet::ServerboundMovePlayerPosRotPacket,
    serverbound_move_player_rot_packet::ServerboundMovePlayerRotPacket,
    serverbound_move_player_status_only_packet::ServerboundMovePlayerStatusOnlyPacket,
};
use azalea_world::entity::EcsEntityId;
use azalea_world::{entity, MoveEntityError};
use bevy_ecs::system::Query;
use std::backtrace::Backtrace;
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
    /// Set whether we're jumping. This acts as if you held space in
    /// vanilla. If you want to jump once, use the `jump` function.
    ///
    /// If you're making a realistic client, calling this function every tick is
    /// recommended.
    pub fn set_jumping(&mut self, jumping: bool) {
        let physics = self.query::<&mut entity::Physics>();
        physics.jumping = jumping;
    }

    /// Returns whether the player will try to jump next tick.
    pub fn jumping(&self) -> bool {
        let physics = self.query::<&mut entity::Physics>();
        physics.jumping
    }

    /// Sets your rotation. `y_rot` is yaw (looking to the side), `x_rot` is
    /// pitch (looking up and down). You can get these numbers from the vanilla
    /// f3 screen.
    /// `y_rot` goes from -180 to 180, and `x_rot` goes from -90 to 90.
    pub fn set_rotation(&mut self, y_rot: f32, x_rot: f32) {
        let mut physics = self.query::<&mut entity::Physics>();

        entity::set_rotation(&mut physics, y_rot, x_rot);
    }
}

impl LocalPlayer {
    /// This gets called automatically every tick.
    pub(crate) fn send_position(
        mut query: Query<
            (
                EcsEntityId,
                &LocalPlayer,
                &entity::Position,
                &mut entity::Physics,
                &entity::metadata::Sprinting,
            ),
            &LocalPlayerInLoadedChunk,
        >,
    ) {
        for (entity, local_player, position, physics, sprinting) in &query {
            local_player.send_sprinting_if_needed(
                entity.into(),
                sprinting,
                &mut local_player.physics_state,
            );

            let packet = {
                // TODO: the camera being able to be controlled by other entities isn't
                // implemented yet if !self.is_controlled_camera() { return };

                let old_position = physics.last_pos;

                let x_delta = position.x - old_position.x;
                let y_delta = position.y - old_position.y;
                let z_delta = position.z - old_position.z;
                let y_rot_delta = (physics.y_rot - physics.y_rot_last) as f64;
                let x_rot_delta = (physics.x_rot - physics.x_rot_last) as f64;

                local_player.physics_state.position_remainder += 1;

                // boolean sendingPosition = Mth.lengthSquared(xDelta, yDelta, zDelta) >
                // Mth.square(2.0E-4D) || this.positionReminder >= 20;
                let sending_position = ((x_delta.powi(2) + y_delta.powi(2) + z_delta.powi(2))
                    > 2.0e-4f64.powi(2))
                    || local_player.physics_state.position_remainder >= 20;
                let sending_rotation = y_rot_delta != 0.0 || x_rot_delta != 0.0;

                // if self.is_passenger() {
                //   TODO: posrot packet for being a passenger
                // }
                let packet = if sending_position && sending_rotation {
                    Some(
                        ServerboundMovePlayerPosRotPacket {
                            x: position.x,
                            y: position.y,
                            z: position.z,
                            x_rot: physics.x_rot,
                            y_rot: physics.y_rot,
                            on_ground: physics.on_ground,
                        }
                        .get(),
                    )
                } else if sending_position {
                    Some(
                        ServerboundMovePlayerPosPacket {
                            x: position.x,
                            y: position.y,
                            z: position.z,
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
                    physics.last_pos = **position;
                    local_player.physics_state.position_remainder = 0;
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
                tokio::spawn(local_player.write_packet(packet));
            }
        }
    }

    fn send_sprinting_if_needed(
        &mut self,
        entity: entity::EntityId,
        sprinting: &entity::metadata::Sprinting,
        physics_state: &mut PhysicsState,
    ) {
        let was_sprinting = physics_state.was_sprinting;
        if **sprinting != was_sprinting {
            let sprinting_action = if **sprinting {
                azalea_protocol::packets::game::serverbound_player_command_packet::Action::StartSprinting
            } else {
                azalea_protocol::packets::game::serverbound_player_command_packet::Action::StopSprinting
            };
            tokio::spawn(
                self.write_packet(
                    ServerboundPlayerCommandPacket {
                        id: *entity,
                        action: sprinting_action,
                        data: 0,
                    }
                    .get(),
                ),
            );
            physics_state.was_sprinting = **sprinting;
        }
    }

    /// Makes the bot do one physics tick. Note that this is already handled
    /// automatically by the client.
    pub fn ai_step(
        query: Query<
            (
                EcsEntityId,
                &mut LocalPlayer,
                &mut entity::Physics,
                &mut entity::Position,
                &mut entity::metadata::Sprinting,
                &mut entity::Attributes,
            ),
            &LocalPlayerInLoadedChunk,
        >,
    ) {
        for (
            ecs_entity_id,
            mut local_player,
            mut physics,
            mut position,
            mut sprinting,
            mut attributes,
        ) in &mut query
        {
            let physics_state = &mut local_player.physics_state;

            Self::tick_controls(None, physics_state);

            // server ai step
            physics.xxa = physics_state.left_impulse;
            physics.zza = physics_state.forward_impulse;

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
                    Self::has_enough_impulse_to_start_sprinting(physics_state)
                    && has_enough_food_to_sprint
                    // && !self.using_item()
                    // && !self.has_effect(MobEffects.BLINDNESS)
                    && trying_to_sprint
                )
            {
                Self::set_sprinting(true, &mut sprinting, &mut attributes);
            }

            azalea_physics::ai_step(
                ecs_entity_id.into(),
                &local_player.world.read().shared,
                &mut physics,
                &mut position,
                &sprinting,
                &attributes,
            )
        }
    }

    /// Update the impulse from self.move_direction. The multipler is used for
    /// sneaking.
    pub(crate) fn tick_controls(multiplier: Option<f32>, physics_state: &mut PhysicsState) {
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
    pub fn walk(
        direction: WalkDirection,
        physics_state: &mut PhysicsState,
        sprinting: &mut entity::metadata::Sprinting,
        attributes: &mut entity::Attributes,
    ) {
        physics_state.move_direction = direction;

        Self::set_sprinting(false, sprinting, attributes);
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
    pub fn sprint(&mut self, direction: SprintDirection, physics_state: &mut PhysicsState) {
        physics_state.move_direction = WalkDirection::from(direction);
        physics_state.trying_to_sprint = true;
    }

    /// Change whether we're sprinting by adding an attribute modifier to the
    /// player. You should use the [`walk`] and [`sprint`] methods instead.
    /// Returns if the operation was successful.
    fn set_sprinting(
        sprinting: bool,
        currently_sprinting: &mut entity::metadata::Sprinting,
        attributes: &mut entity::Attributes,
    ) -> bool {
        **currently_sprinting = sprinting;
        if sprinting {
            attributes
                .speed
                .insert(entity::attributes::sprinting_modifier())
                .is_ok()
        } else {
            attributes
                .speed
                .remove(&entity::attributes::sprinting_modifier().uuid)
                .is_none()
        }
    }

    // Whether the player is moving fast enough to be able to start sprinting.
    fn has_enough_impulse_to_start_sprinting(physics_state: &PhysicsState) -> bool {
        // if self.underwater() {
        //     self.has_forward_impulse()
        // } else {
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
