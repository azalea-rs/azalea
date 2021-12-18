use super::GamePacket;
use crate::mc_buf::{Readable, Writable};
use azalea_core::{game_type::GameType, resource_location::ResourceLocation};
use std::hash::Hash;

#[derive(Hash, Clone, Debug)]
pub struct ClientboundLoginPacket {
    // private final int playerId;
    // private final boolean hardcore;
    // private final GameType gameType;
    // @Nullable
    // private final GameType previousGameType;
    // private final Set<ResourceKey<Level>> levels;
    // private final RegistryAccess.RegistryHolder registryHolder;
    // private final DimensionType dimensionType;
    // private final ResourceKey<Level> dimension;
    // private final long seed;
    // private final int maxPlayers;
    // private final int chunkRadius;
    // private final int simulationDistance;
    // private final boolean reducedDebugInfo;
    // private final boolean showDeathScreen;
    // private final boolean isDebug;
    // private final boolean isFlat;
    pub player_id: i32,
    pub hardcore: bool,
    pub game_type: GameType,
    pub previous_game_type: Option<GameType>,
    pub levels: Vec<ResourceLocation>,
    pub registry_holder: azalea_core::registry::RegistryAccess,
}

impl ClientboundLoginPacket {
    pub fn get(self) -> GamePacket {
        GamePacket::ClientboundLoginPacket(self)
    }

    pub fn write(&self, buf: &mut Vec<u8>) {
        buf.write_int(self.player_id);
        buf.write_bool(self.hardcore);
        // buf.write_byte(self.game_type.
    }

    pub async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        buf: &mut T,
    ) -> Result<GamePacket, String> {
        let transaction_id = buf.read_varint().await? as u32;
        let identifier = ResourceLocation::new(&buf.read_utf().await?)?;
        let data = buf.read_bytes(1048576).await?;
        Ok(ClientboundLoginPacket {
            transaction_id,
            identifier,
            data,
        }
        .get())
    }
}
