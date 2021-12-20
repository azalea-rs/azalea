use super::GamePacket;
use crate::mc_buf::{Readable, Writable};
use azalea_core::{game_type::GameType, resource_location::ResourceLocation};

#[derive(Clone, Debug)]
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
    pub registry_holder: azalea_nbt::Tag,
    pub dimension_type: azalea_nbt::Tag,
    pub dimension: ResourceLocation,
    pub seed: i64,
    pub max_players: i32,
    pub chunk_radius: i32,
    pub simulation_distance: i32,
    pub reduced_debug_info: bool,
    pub show_death_screen: bool,
    pub is_debug: bool,
    pub is_flat: bool,
}

impl ClientboundLoginPacket {
    pub fn get(self) -> GamePacket {
        GamePacket::ClientboundLoginPacket(self)
    }

    pub fn write(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        buf.write_int(self.player_id)?;
        buf.write_boolean(self.hardcore)?;
        buf.write_byte(self.game_type.to_id())?;
        buf.write_byte(GameType::to_optional_id(&self.previous_game_type) as u8)?;
        buf.write_list(&self.levels, |buf, resource_location| {
            buf.write_utf(&resource_location.to_string())
        })?;
        self.registry_holder
            .write(buf)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "write registry holder"))?;

        Ok(())
    }

    pub async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        buf: &mut T,
    ) -> Result<GamePacket, String> {
        let transaction_id = buf.read_varint().await? as u32;
        let identifier = ResourceLocation::new(&buf.read_utf().await?)?;
        let data = buf.read_bytes(1048576).await?;
        panic!("not implemented");
        // Ok(ClientboundLoginPacket {
        //     transaction_id,
        //     identifier,
        //     data,
        // }
        // .get())
    }
}
