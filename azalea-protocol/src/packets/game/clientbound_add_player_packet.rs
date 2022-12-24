use azalea_buf::McBuf;
use azalea_core::Vec3;
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
    fn as_bundle(p: &ClientboundAddPlayerPacket) -> PlayerBundle {
        PlayerBundle {
            entity: EntityBundle::new(p.uuid, p.position, EntityKind::Player),
            metadata: PlayerMetadataBundle::default(),
        }
    }
}
