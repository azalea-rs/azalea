use crate::Client;
use azalea_core::EntityPos;
use azalea_protocol::packets::game::serverbound_move_player_packet_pos_rot::ServerboundMovePlayerPacketPosRot;

impl Client {
    /// Set the client's position to the given coordinates.
    pub async fn move_to(&mut self, pos: &EntityPos) {
        self.conn
            .lock()
            .await
            .write(
                ServerboundMovePlayerPacketPosRot {
                    x: pos.x,
                    y: pos.y,
                    z: pos.z,
                    x_rot: 0.0,
                    y_rot: 0.0,
                    on_ground: false,
                }
                .get(),
            )
            .await;
    }
}
