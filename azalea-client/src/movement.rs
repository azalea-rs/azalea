use crate::Client;
use azalea_core::EntityPos;
use azalea_protocol::packets::game::serverbound_move_player_packet_pos_rot::ServerboundMovePlayerPacketPosRot;
use azalea_world::MoveEntityError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MovePlayerError {
    #[error("Player is not in world")]
    PlayerNotInWorld,
    #[error("{0}")]
    Io(#[from] std::io::Error),
}

impl Client {
    /// Set the client's position to the given coordinates.
    pub async fn move_to(&mut self, new_pos: EntityPos) -> Result<(), MovePlayerError> {
        {
            let mut dimension_lock = self.dimension.lock().unwrap();
            let dimension = dimension_lock.as_mut().unwrap();

            let player_lock = self.player.lock().unwrap();

            let player_id = if let Some(player_lock) = player_lock.entity(dimension) {
                player_lock.id
            } else {
                return Err(MovePlayerError::PlayerNotInWorld);
            };

            match dimension.move_entity(player_id, new_pos) {
                Ok(_) => Ok(()),
                Err(e) => match e {
                    MoveEntityError::EntityDoesNotExist => Err(MovePlayerError::PlayerNotInWorld),
                },
            }?;
        }

        self.conn
            .lock()
            .await
            .write(
                ServerboundMovePlayerPacketPosRot {
                    x: new_pos.x,
                    y: new_pos.y,
                    z: new_pos.z,
                    x_rot: 0.0,
                    y_rot: 0.0,
                    on_ground: false,
                }
                .get(),
            )
            .await?;

        Ok(())
    }
}
