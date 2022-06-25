use crate::Client;
use azalea_core::EntityPos;
use azalea_protocol::packets::game::serverbound_move_player_packet_pos_rot::ServerboundMovePlayerPacketPosRot;

impl Client {
    /// Set the client's position to the given coordinates.
    pub async fn move_to(&mut self, new_pos: EntityPos) -> Result<(), String> {
        let mut state_lock = self.state.lock().unwrap();

        let world = state_lock.world.as_ref().unwrap();

        let player = &state_lock.player;
        let player_id = if let Some(player) = player.entity(world) {
            player.id
        } else {
            return Err("Player entity not found".to_string());
        };

        let world = state_lock.world.as_mut().unwrap();
        world.move_entity(player_id, new_pos)?;

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
            .await;

        Ok(())
    }
}
