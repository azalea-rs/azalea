pub mod clientbound_custom_query_packet;
pub mod clientbound_game_profile_packet;
pub mod clientbound_hello_packet;
pub mod clientbound_login_compression_packet;
pub mod clientbound_login_disconnect_packet;
pub mod serverbound_custom_query_answer_packet;
pub mod serverbound_hello_packet;
pub mod serverbound_key_packet;
pub mod serverbound_login_acknowledged_packet;

use azalea_protocol_macros::declare_state_packets;

declare_state_packets!(
    LoginPacket,
    Serverbound => {
        0x08: serverbound_custom_query_answer_packet::ServerboundCustomQueryAnswerPacket,
        0x09: serverbound_hello_packet::ServerboundHelloPacket,
        0x0a: serverbound_key_packet::ServerboundKeyPacket,
        0x0b: serverbound_login_acknowledged_packet::ServerboundLoginAcknowledgedPacket,
    },
    Clientbound => {
        0x03: clientbound_custom_query_packet::ClientboundCustomQueryPacket,
        0x04: clientbound_game_profile_packet::ClientboundGameProfilePacket,
        0x05: clientbound_hello_packet::ClientboundHelloPacket,
        0x06: clientbound_login_compression_packet::ClientboundLoginCompressionPacket,
        0x07: clientbound_login_disconnect_packet::ClientboundLoginDisconnectPacket,
    }
);
