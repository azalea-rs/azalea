use super::GamePacket;
use crate::mc_buf::{Readable, Writable};
use azalea_core::resource_location::ResourceLocation;
use std::hash::Hash;
use tokio::io::BufReader;

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
	
}

impl ClientboundLoginPacket {
    pub fn get(self) -> GamePacket {
        GamePacket::ClientboundLoginPacket(self)
    }

    pub fn write(&self, buf: &mut Vec<u8>) {
        buf.write_varint(self.transaction_id as i32).unwrap();
        buf.write_utf(self.identifier.to_string().as_str()).unwrap();
        buf.write_bytes(&self.data).unwrap();
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
