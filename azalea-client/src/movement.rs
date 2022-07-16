use crate::Client;
use azalea_core::Vec3;
use azalea_protocol::packets::game::{
    serverbound_move_player_packet_pos::ServerboundMovePlayerPacketPos,
    serverbound_move_player_packet_pos_rot::ServerboundMovePlayerPacketPosRot,
    serverbound_move_player_packet_rot::ServerboundMovePlayerPacketRot,
    serverbound_move_player_packet_status_only::ServerboundMovePlayerPacketStatusOnly,
};

impl Client {
    pub async fn send_position(&mut self) -> Result<(), String> {
        let player_lock = self.player.lock().unwrap();

        let mut dimension_lock = self.dimension.lock().unwrap();
        let dimension_lock = dimension_lock.as_mut().expect("Dimension must exist");

        let player_entity = player_lock
            .mut_entity(dimension_lock)
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

        self.position_remainder += 1;

        // boolean sendingPosition = Mth.lengthSquared(xDelta, yDelta, zDelta) > Mth.square(2.0E-4D) || this.positionReminder >= 20;
        let sending_position = ((x_delta.powi(2) + y_delta.powi(2) + z_delta.powi(2))
            > 2.0e-4f64.powi(2))
            || self.position_remainder >= 20;
        let sending_rotation = y_rot_delta != 0.0 || x_rot_delta != 0.0;

        // if self.is_passenger() {
        //   TODO: posrot packet for being a passenger
        // }
        let packet = if sending_position && sending_rotation {
            Some(
                ServerboundMovePlayerPacketPosRot {
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
                ServerboundMovePlayerPacketPos {
                    x: player_pos.x,
                    y: player_pos.y,
                    z: player_pos.z,
                    on_ground: player_entity.on_ground,
                }
                .get(),
            )
        } else if sending_rotation {
            Some(
                ServerboundMovePlayerPacketRot {
                    x_rot: player_entity.x_rot,
                    y_rot: player_entity.y_rot,
                    on_ground: player_entity.on_ground,
                }
                .get(),
            )
        } else if player_entity.last_on_ground != player_entity.on_ground {
            Some(
                ServerboundMovePlayerPacketStatusOnly {
                    on_ground: player_entity.on_ground,
                }
                .get(),
            )
        } else {
            None
        };

        if let Some(packet) = packet {
            self.conn.lock().await.write(packet).await;
        }

        if sending_position {
            player_entity.last_pos = *player_entity.pos();
            self.position_remainder = 0;
        }
        if sending_rotation {
            player_entity.y_rot_last = player_entity.y_rot;
            player_entity.x_rot_last = player_entity.x_rot;
        }

        player_entity.last_on_ground = player_entity.on_ground;
        // minecraft checks for autojump here, but also autojump is bad so

        Ok(())
    }

    pub async fn move_to_nocollision(&mut self, new_pos: Vec3) -> Result<(), String> {
        let mut dimension_lock = self.dimension.lock().unwrap();
        let dimension = dimension_lock.as_mut().unwrap();

        let player_lock = self.player.lock().unwrap();

        let player_id = if let Some(player_lock) = player_lock.entity(dimension) {
            player_lock.id
        } else {
            return Err("Player entity not found".to_string());
        };

        dimension.move_entity(player_id, new_pos)?;

        Ok(())
    }
}
