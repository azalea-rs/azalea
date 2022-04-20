pub mod clientbound_status_response_packet;
pub mod serverbound_status_request_packet;

use packet_macros::declare_state_packets;

declare_state_packets!(
    StatusPacket,
    Serverbound => {
        0x00: serverbound_status_request_packet::ServerboundStatusRequestPacket,
    },
    Clientbound => {
        0x00: clientbound_status_response_packet::ClientboundStatusResponsePacket,
    }
);
