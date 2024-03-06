pub mod serverbound_ping_request_packet;
pub mod clientbound_pong_response_packet;
pub mod serverbound_status_request_packet;
pub mod clientbound_status_response_packet;

use azalea_protocol_macros::declare_state_packets;

declare_state_packets!(
    StatusPacket,
    Serverbound => {
        0xbb: serverbound_ping_request_packet::ServerboundPingRequestPacket,
        0xbc: serverbound_status_request_packet::ServerboundStatusRequestPacket,
    },
    Clientbound => {
        0xb9: clientbound_pong_response_packet::ClientboundPongResponsePacket,
        0xba: clientbound_status_response_packet::ClientboundStatusResponsePacket,
    }
);