pub mod clientbound_pong_response_packet;
pub mod clientbound_status_response_packet;
pub mod serverbound_ping_request_packet;
pub mod serverbound_status_request_packet;

use azalea_protocol_macros::declare_state_packets;

declare_state_packets!(
    StatusPacket,
    Serverbound => {
        0x00: serverbound_status_request_packet::ServerboundStatusRequestPacket,
        0x01: serverbound_ping_request_packet::ServerboundPingRequestPacket,
    },
    Clientbound => {
        0x00: clientbound_status_response_packet::ClientboundStatusResponsePacket,
        0x01: clientbound_pong_response_packet::ClientboundPongResponsePacket,
    }
);
