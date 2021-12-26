use super::GamePacket;
use crate::mc_buf::{Readable, Writable};
use azalea_core::{game_type::GameType, resource_location::ResourceLocation};
use tokio::io::AsyncReadExt;

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
            buf.write_resource_location(resource_location)
        })?;
        buf.write_nbt(&self.registry_holder)?;
        buf.write_nbt(&self.dimension_type)?;
        buf.write_resource_location(&self.dimension)?;
        buf.write_long(self.seed)?;
        buf.write_varint(self.max_players)?;
        buf.write_varint(self.chunk_radius)?;
        buf.write_varint(self.simulation_distance)?;
        buf.write_boolean(self.reduced_debug_info)?;
        buf.write_boolean(self.show_death_screen)?;
        buf.write_boolean(self.is_debug)?;
        buf.write_boolean(self.is_flat)?;
        Ok(())
    }

    pub async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        buf: &mut T,
    ) -> Result<GamePacket, String> {
        let player_id = buf.read_int().await?;
        let hardcore = buf.read_boolean().await?;
        let game_type = GameType::from_id(buf.read_byte().await?)?;
        let previous_game_type = GameType::from_optional_id(buf.read_byte().await? as i8)?;

        let mut levels = Vec::new();
        let length = buf.read_varint().await?;
        for _ in 0..length {
            levels.push(buf.read_resource_location().await?);
        }

        // println!("about to read nbt");
        // // read all the bytes into a buffer, print it, and panic
        // let mut registry_holder_buf = Vec::new();
        // buf.read_to_end(&mut registry_holder_buf).await.unwrap();
        // println!("{:?}", String::from_utf8_lossy(&registry_holder_buf));
        // panic!("");

        let registry_holder = buf.read_nbt().await?;
        let dimension_type = buf.read_nbt().await?;
        let dimension = buf.read_resource_location().await?;
        let seed = buf.read_long().await?;
        let max_players = buf.read_varint().await?;
        let chunk_radius = buf.read_varint().await?;
        let simulation_distance = buf.read_varint().await?;
        let reduced_debug_info = buf.read_boolean().await?;
        let show_death_screen = buf.read_boolean().await?;
        let is_debug = buf.read_boolean().await?;
        let is_flat = buf.read_boolean().await?;

        Ok(ClientboundLoginPacket {
            player_id,
            hardcore,
            game_type,
            previous_game_type,
            levels,
            registry_holder,
            dimension_type,
            dimension,
            seed,
            max_players,
            chunk_radius,
            simulation_distance,
            reduced_debug_info,
            show_death_screen,
            is_debug,
            is_flat,
        }
        .get())
    }
}
