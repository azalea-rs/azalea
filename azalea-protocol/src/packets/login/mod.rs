pub mod clientbound_cookie_request_packet;
pub mod clientbound_custom_query_packet;
pub mod clientbound_hello_packet;
pub mod clientbound_login_compression_packet;
pub mod clientbound_login_disconnect_packet;
pub mod clientbound_login_finished_packet;
pub mod serverbound_cookie_response_packet;
pub mod serverbound_custom_query_answer_packet;
pub mod serverbound_hello_packet;
pub mod serverbound_key_packet;
pub mod serverbound_login_acknowledged_packet;

use azalea_protocol_macros::declare_state_packets;

declare_state_packets!(
    LoginPacket,
    Serverbound => {
        0x00: serverbound_hello_packet::ServerboundHelloPacket,
        0x01: serverbound_key_packet::ServerboundKeyPacket,
        0x02: serverbound_custom_query_answer_packet::ServerboundCustomQueryAnswerPacket,
        0x03: serverbound_login_acknowledged_packet::ServerboundLoginAcknowledgedPacket,
        0x04: serverbound_cookie_response_packet::ServerboundCookieResponsePacket,
    },
    Clientbound => {
        0x00: clientbound_login_disconnect_packet::ClientboundLoginDisconnectPacket,
        0x01: clientbound_hello_packet::ClientboundHelloPacket,
        0x02: clientbound_login_finished_packet::ClientboundLoginFinishedPacket,
        0x03: clientbound_login_compression_packet::ClientboundLoginCompressionPacket,
        0x04: clientbound_custom_query_packet::ClientboundCustomQueryPacket,
        0x05: clientbound_cookie_request_packet::ClientboundCookieRequestPacket,
    }
);
