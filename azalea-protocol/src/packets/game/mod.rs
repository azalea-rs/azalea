pub mod clientbound_change_difficulty_packet;
pub mod clientbound_custom_payload_packet;
pub mod clientbound_login_packet;
pub mod clientbound_update_view_distance_packet;

use packet_macros::declare_state_packets;

declare_state_packets!(
    GamePacket,
    Serverbound => {},
    Clientbound => {
        0x0e: clientbound_change_difficulty_packet::ClientboundChangeDifficultyPacket,
        0x18: clientbound_custom_payload_packet::ClientboundCustomPayloadPacket,
        0x26: clientbound_login_packet::ClientboundLoginPacket,
        0x4a: clientbound_update_view_distance_packet::ClientboundUpdateViewDistancePacket,
    }
);
