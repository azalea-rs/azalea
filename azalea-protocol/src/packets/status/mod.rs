pub mod c_pong_response;
pub mod c_status_response;
pub mod s_ping_request;
pub mod s_status_request;

use azalea_protocol_macros::declare_state_packets;

declare_state_packets!(
    StatusPacket,
    Serverbound => {
        0x00: s_status_request::ServerboundStatusRequest,
        0x01: s_ping_request::ServerboundPingRequest,
    },
    Clientbound => {
        0x00: c_status_response::ClientboundStatusResponse,
        0x01: c_pong_response::ClientboundPongResponse,
    }
);
