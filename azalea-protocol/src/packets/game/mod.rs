pub mod clientbound_change_difficulty_packet;
pub mod clientbound_custom_payload_packet;
pub mod clientbound_declare_commands_packet;
pub mod clientbound_disconnect_packet;
pub mod clientbound_login_packet;
pub mod clientbound_player_abilities_packet;
pub mod clientbound_set_carried_item_packet;
pub mod clientbound_update_tags_packet;
pub mod clientbound_update_view_distance_packet;

use packet_macros::declare_state_packets;

declare_state_packets!(
    GamePacket,
    Serverbound => {},
    Clientbound => {
        0x0e: clientbound_change_difficulty_packet::ClientboundChangeDifficultyPacket,
        0x12: clientbound_declare_commands_packet::ClientboundDeclareCommandsPacket,
        0x1a: clientbound_disconnect_packet::ClientboundDisconnectPacket,
        0x18: clientbound_custom_payload_packet::ClientboundCustomPayloadPacket,
        0x26: clientbound_login_packet::ClientboundLoginPacket,
        0x32: clientbound_player_abilities_packet::ClientboundPlayerAbilitiesPacket,
        0x48: clientbound_set_carried_item_packet::ClientboundSetCarriedItemPacket,
        0x4a: clientbound_update_view_distance_packet::ClientboundUpdateViewDistancePacket,
        0x67: clientbound_update_tags_packet::ClientboundUpdateTagsPacket
    }
);
