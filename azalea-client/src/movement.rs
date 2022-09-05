use crate::Client;
use azalea_core::Vec3;
use azalea_physics::collision::{MovableEntity, MoverType};
use azalea_physics::HasPhysics;
use azalea_protocol::packets::game::{
    serverbound_move_player_pos_packet::ServerboundMovePlayerPosPacket,
    serverbound_move_player_pos_rot_packet::ServerboundMovePlayerPosRotPacket,
    serverbound_move_player_rot_packet::ServerboundMovePlayerRotPacket,
    serverbound_move_player_status_only_packet::ServerboundMovePlayerStatusOnlyPacket,
};
use azalea_world::MoveEntityError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MovePlayerError {
    #[error("Player is not in world")]
    PlayerNotInWorld,
    #[error("{0}")]
    Io(#[from] std::io::Error),
}

impl From<MoveEntityError> for MovePlayerError {
    fn from(err: MoveEntityError) -> Self {
        match err {
            MoveEntityError::EntityDoesNotExist => MovePlayerError::PlayerNotInWorld,
        }
    }
}

impl Client {
    /// This gets called every tick.
    pub async fn send_position(&mut self) -> Result<(), MovePlayerError> {
        let packet = {
            let player_lock = self.player.lock().unwrap();
            let mut physics_state = self.physics_state.lock().unwrap();
            let mut dimension_lock = self.dimension.lock().unwrap();

            let mut player_entity = player_lock
                .entity_mut(&mut dimension_lock)
                .expect("Player must exist");
            let player_pos = player_entity.pos();
            let player_old_pos = player_entity.last_pos;

            // TODO: send sprinting and sneaking packets here if they changed

            // TODO: the camera being able to be controlled by other entities isn't implemented yet
            // if !self.is_controlled_camera() { return };

            let x_delta = player_pos.x - player_old_pos.x;
            let y_delta = player_pos.y - player_old_pos.y;
            let z_delta = player_pos.z - player_old_pos.z;
            let y_rot_delta = (player_entity.y_rot - player_entity.y_rot_last) as f64;
            let x_rot_delta = (player_entity.x_rot - player_entity.x_rot_last) as f64;

            physics_state.position_remainder += 1;

            // boolean sendingPosition = Mth.lengthSquared(xDelta, yDelta, zDelta) > Mth.square(2.0E-4D) || this.positionReminder >= 20;
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
                        x_rot: player_entity.x_rot,
                        y_rot: player_entity.y_rot,
                        on_ground: player_entity.on_ground,
                    }
                    .get(),
                )
            } else if sending_position {
                Some(
                    ServerboundMovePlayerPosPacket {
                        x: player_pos.x,
                        y: player_pos.y,
                        z: player_pos.z,
                        on_ground: player_entity.on_ground,
                    }
                    .get(),
                )
            } else if sending_rotation {
                Some(
                    ServerboundMovePlayerRotPacket {
                        x_rot: player_entity.x_rot,
                        y_rot: player_entity.y_rot,
                        on_ground: player_entity.on_ground,
                    }
                    .get(),
                )
            } else if player_entity.last_on_ground != player_entity.on_ground {
                Some(
                    ServerboundMovePlayerStatusOnlyPacket {
                        on_ground: player_entity.on_ground,
                    }
                    .get(),
                )
            } else {
                None
            };

            if sending_position {
                player_entity.last_pos = *player_entity.pos();
                physics_state.position_remainder = 0;
            }
            if sending_rotation {
                player_entity.y_rot_last = player_entity.y_rot;
                player_entity.x_rot_last = player_entity.x_rot;
            }

            player_entity.last_on_ground = player_entity.on_ground;
            // minecraft checks for autojump here, but also autojump is bad so

            packet
        };

        if let Some(packet) = packet {
            self.conn.lock().await.write(packet).await?;
        }

        Ok(())
    }

    // Set our current position to the provided Vec3, potentially clipping through blocks.
    pub async fn set_pos(&mut self, new_pos: Vec3) -> Result<(), MovePlayerError> {
        let player_lock = self.player.lock().unwrap();
        let mut dimension_lock = self.dimension.lock().unwrap();

        dimension_lock.set_entity_pos(player_lock.entity_id, new_pos)?;

        Ok(())
    }

    pub async fn move_entity(&mut self, movement: &Vec3) -> Result<(), MovePlayerError> {
        let mut dimension_lock = self.dimension.lock().unwrap();
        let player = self.player.lock().unwrap();

        let mut entity = player
            .entity_mut(&mut dimension_lock)
            .ok_or(MovePlayerError::PlayerNotInWorld)?;
        println!(
            "move entity bounding box: {} {:?}",
            entity.id, entity.bounding_box
        );

        entity.move_colliding(&MoverType::Own, movement)?;

        Ok(())
    }

    pub fn ai_step(&mut self) {
        self.tick_controls(None);

        let player_lock = self.player.lock().unwrap();
        let mut dimension_lock = self.dimension.lock().unwrap();

        let mut player_entity = player_lock
            .entity_mut(&mut dimension_lock)
            .expect("Player must exist");

        // server ai step
        {
            let physics_state = self.physics_state.lock().unwrap();
            player_entity.xxa = physics_state.left_impulse;
            player_entity.zza = physics_state.forward_impulse;
        }

        player_entity.ai_step();
    }

    /// Update the impulse from self.move_direction. The multipler is used for sneaking.
    pub(crate) fn tick_controls(&mut self, multiplier: Option<f32>) {
        let mut physics_state = self.physics_state.lock().unwrap();

        let mut forward_impulse: f32 = 0.;
        let mut left_impulse: f32 = 0.;
        match physics_state.move_direction {
            MoveDirection::Forward | MoveDirection::ForwardRight | MoveDirection::ForwardLeft => {
                forward_impulse += 1.;
            }
            MoveDirection::Backward
            | MoveDirection::BackwardRight
            | MoveDirection::BackwardLeft => {
                forward_impulse -= 1.;
            }
            _ => {}
        };
        match physics_state.move_direction {
            MoveDirection::Right | MoveDirection::ForwardRight | MoveDirection::BackwardRight => {
                left_impulse += 1.;
            }
            MoveDirection::Left | MoveDirection::ForwardLeft | MoveDirection::BackwardLeft => {
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

    pub fn walk(&mut self, direction: MoveDirection) {
        let mut physics_state = self.physics_state.lock().unwrap();
        physics_state.move_direction = direction;
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum MoveDirection {
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
