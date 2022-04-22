pub mod clientbound_custom_query_packet;
pub mod clientbound_game_profile_packet;
pub mod clientbound_hello_packet;
pub mod clientbound_login_compression_packet;
pub mod serverbound_hello_packet;

use packet_macros::declare_state_packets;

declare_state_packets!(
    LoginPacket,
    Serverbound => {
        0x00: serverbound_hello_packet::ServerboundHelloPacket,
    },
    Clientbound => {
        0x00: clientbound_hello_packet::ClientboundHelloPacket,
        0x02: clientbound_game_profile_packet::ClientboundGameProfilePacket,
        0x03: clientbound_login_compression_packet::ClientboundLoginCompressionPacket,
        0x04: clientbound_custom_query_packet::ClientboundCustomQueryPacket,
    }
);
