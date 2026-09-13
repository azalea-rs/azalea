use azalea_buf::AzBuf;
use azalea_core::entity_id::MinecraftEntityId;
use azalea_inventory::components::SwingAnimation;
use azalea_protocol_macros::ClientboundGamePacket;

use crate::packets::game::s_interact::InteractionHand;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundSwingAnimation {
    pub entity_id: MinecraftEntityId,
    pub hand: InteractionHand,
    pub animation: SwingAnimation,
}
