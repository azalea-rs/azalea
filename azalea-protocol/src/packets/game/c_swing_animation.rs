use azalea_buf::AzBuf;
use azalea_core::entity_id::MinecraftEntityId;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::item::components::SwingAnimation;

use crate::packets::game::s_interact::InteractionHand;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundSwingAnimation {
    #[var]
    pub entity_id: MinecraftEntityId,
    pub hand: InteractionHand,
    pub animation: SwingAnimation,
}
