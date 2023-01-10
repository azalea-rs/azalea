use azalea_buf::McBuf;
use azalea_core::{ResourceLocation, Vec3};
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::EntityKind;
use azalea_world::entity::{metadata::PlayerMetadataBundle, EntityBundle, PlayerBundle};
use uuid::Uuid;

/// This packet is sent by the server when a player comes into visible range,
/// not when a player joins.
#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundAddPlayerPacket {
    #[var]
    pub id: u32,
    pub uuid: Uuid,
    pub position: Vec3,
    pub x_rot: i8,
    pub y_rot: i8,
}

impl ClientboundAddPlayerPacket {
    pub fn as_player_bundle(&self, world_name: ResourceLocation) -> PlayerBundle {
        PlayerBundle {
            entity: EntityBundle::new(self.uuid, self.position, EntityKind::Player, world_name),
            metadata: PlayerMetadataBundle::default(),
        }
    }
}
